use std::path::PathBuf;

use rlua::{FromLua, Value};

use crate::FeedItem;

mod env;

use crate::interp::inst::{Instruction, Program};

pub(crate) struct Runtime {
    send: std::sync::mpsc::Sender<Message>,
}

enum Message {
    Process(FeedItem, tokio::sync::oneshot::Sender<Program>),
}

impl Runtime {
    pub(crate) fn new(path: PathBuf) -> Runtime {
        let (send, recv) = std::sync::mpsc::channel();

        std::thread::spawn(|| runtime_main(recv, path));

        Self { send }
    }

    pub(crate) async fn process(&self, item: FeedItem) -> crate::Result<Program> {
        let (send, recv) = tokio::sync::oneshot::channel();
        self.send
            .send(Message::Process(item, send))
            .map_err(|_| crate::Error::RuntimeShutdown)?;

        recv.await.map_err(|_| crate::Error::RuntimeShutdown)
    }
}

fn runtime_main(recv: std::sync::mpsc::Receiver<Message>, path: PathBuf) {
    let interp = rlua::Lua::new();
    let env = env::Env::default();
    let inst = env.inst.clone();

    let Ok(conf) = interp.load(path).set_environment(env).eval::<Conf>() else {
        return;
    };

    while let Ok(msg) = recv.recv() {
        match msg {
            Message::Process(feed_item, sender) => {
                {
                    let Ok(mut guard) = inst.lock() else {
                        continue;
                    };
                    guard.clear();
                }

                if let Err(e) = conf.func.call::<FeedItem, Value>(feed_item) {
                    dbg!(e);
                    continue;
                }

                {
                    let Ok(guard) = inst.lock() else {
                        continue;
                    };

                    let _ = sender.send(Program {
                        instructions: (*guard).clone(),
                    });
                }
            }
        }
    }
}

struct Conf<'lua> {
    func: rlua::Function<'lua>,
}

impl<'lua> FromLua<'lua> for Conf<'lua> {
    fn from_lua(value: rlua::Value<'lua>, _: &'lua rlua::Lua) -> rlua::Result<Self> {
        if let rlua::Value::Table(table) = value {
            let func = table.get("process")?;
            Ok(Self { func })
        } else {
            Err(rlua::Error::runtime("expected an object for configuration"))
        }
    }
}
