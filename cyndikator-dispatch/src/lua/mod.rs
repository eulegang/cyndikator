use std::sync::{Arc, Mutex};

use crate::{Action, Event};
use regex::Regex;
use rlua::{Error as LuaError, Function, Lua, RegistryKey, Table, Value};

pub struct LuaDispatch {
    actions: Arc<Mutex<Vec<Action>>>,
    lua: Lua,
    reg_key: RegistryKey,
}

impl LuaDispatch {
    pub fn parse(input: &str) -> Result<LuaDispatch, LuaError> {
        let actions = Arc::new(Mutex::new(Vec::new()));
        let lua = Lua::new();

        let a = actions.clone();
        let reg_key = lua.context(|ctx| {
            let globals = ctx.globals();

            let actions = a.clone();
            let record = ctx.create_function_mut(move |_, ()| {
                let mut actions = match a.lock() {
                    Ok(a) => a,
                    Err(_) => return Err(rlua::Error::RuntimeError("Poisoned lock".to_string())),
                };
                actions.push(Action::Record);

                Ok(())
            })?;

            let a = actions.clone();
            let notify = ctx.create_function_mut(move |_, ()| {
                let mut actions = match a.lock() {
                    Ok(a) => a,
                    Err(_) => return Err(rlua::Error::RuntimeError("Poisoned lock".to_string())),
                };
                actions.push(Action::Notify);

                Ok(())
            })?;

            let a = actions.clone();
            let exec = ctx.create_function_mut(move |_, sh: String| {
                let mut actions = match a.lock() {
                    Ok(a) => a,
                    Err(_) => return Err(rlua::Error::RuntimeError("Poisoned lock".to_string())),
                };
                actions.push(Action::Exec(sh));

                Ok(())
            })?;

            let contains =
                ctx.create_function(|_, (t, s): (Table, String)| -> Result<bool, LuaError> {
                    for pair in t.pairs::<Value, String>() {
                        let (_, v) = pair?;

                        if v == s {
                            return Ok(true);
                        }
                    }

                    Ok(false)
                })?;

            let matches =
                ctx.create_function(|_, (s, r): (String, String)| -> Result<bool, LuaError> {
                    let re = Regex::new(&r)
                        .map_err(|err| LuaError::RuntimeError(format!("invalid regex: {err}")))?;

                    Ok(re.is_match(&s))
                })?;

            globals.set("record", record)?;
            globals.set("notify", notify)?;
            globals.set("exec", exec)?;

            globals.set("contains", contains)?;
            globals.set("matches", matches)?;

            let chunk = ctx.load(input);

            ctx.create_registry_value(chunk.into_function()?)
        })?;

        Ok(LuaDispatch {
            actions,
            lua,
            reg_key,
        })
    }

    pub fn dispatch(&self, event: &Event) -> Vec<Action> {
        {
            let mut actions = self.actions.lock().unwrap();
            actions.clear();
        }

        let _: Result<(), LuaError> = self.lua.context(|ctx| {
            let globals = ctx.globals();

            globals.set("url", event.url.as_deref())?;
            globals.set("title", event.title.as_deref())?;
            globals.set("description", event.description.as_deref())?;
            globals.set(
                "categories",
                ctx.create_sequence_from(event.categories.clone())?,
            )?;
            globals.set("feed_url", event.feed_url.as_str())?;
            globals.set("feed_title", event.feed_title.as_deref())?;
            globals.set(
                "feed_categories",
                ctx.create_sequence_from(event.feed_categories.clone())?,
            )?;

            let func: Function = ctx.registry_value(&self.reg_key)?;

            func.call(())?;

            Ok(())
        });

        {
            use std::borrow::Borrow;

            let actions = self.actions.lock().unwrap();
            actions.borrow().to_vec()
        }
    }
}
