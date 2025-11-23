use std::sync::{Arc, Mutex};

use rlua::{FromLua, ToLua, Value};

use crate::{
    interp::{Alert, Exec, Record},
    runtime::Instruction,
};

#[derive(Default)]
pub(crate) struct Env {
    pub(crate) inst: Arc<Mutex<Vec<Instruction>>>,
}

struct AlertOptions {
    summary: Option<String>,
    message: Option<String>,
}

impl<'lua> FromLua<'lua> for AlertOptions {
    fn from_lua(value: Value<'lua>, _: &'lua rlua::Lua) -> rlua::Result<Self> {
        if let Some(message) = value.as_str() {
            Ok(AlertOptions {
                summary: None,
                message: Some(message.to_string()),
            })
        } else if let Some(table) = value.as_table() {
            let summary = table.get("summary")?;
            let message = table.get("message")?;

            Ok(AlertOptions { summary, message })
        } else {
            Err(rlua::Error::RuntimeError(
                "invalid type signature".to_string(),
            ))
        }
    }
}

impl<'lua> ToLua<'lua> for Env {
    fn into_lua(self, lua: &'lua rlua::Lua) -> rlua::Result<rlua::Value<'lua>> {
        let table = lua.globals();

        let inst = self.inst.clone();
        table.set(
            "record",
            lua.create_function(move |_, _: ()| {
                let Ok(mut inst) = inst.lock() else {
                    return Err(rlua::Error::runtime("failed to lock instructions"));
                };

                inst.push(Record {}.into());

                Ok(Value::Nil)
            })?,
        )?;

        let inst = self.inst.clone();
        table.set(
            "alert",
            lua.create_function(move |_, opts: Option<AlertOptions>| {
                let Ok(mut inst) = inst.lock() else {
                    return Err(rlua::Error::runtime("failed to lock instructions"));
                };

                let opts = opts.unwrap_or(AlertOptions {
                    summary: None,
                    message: None,
                });

                inst.push(
                    Alert {
                        summary: opts.summary,
                        message: opts.message,
                    }
                    .into(),
                );

                Ok(Value::Nil)
            })?,
        )?;

        let inst = self.inst.clone();
        table.set(
            "exec",
            lua.create_function(move |_, sh: String| {
                let Ok(mut inst) = inst.lock() else {
                    return Err(rlua::Error::runtime("failed to lock instructions"));
                };

                inst.push(Exec { sh }.into());

                Ok(Value::Nil)
            })?,
        )?;

        table.set(
            "log",
            lua.create_function(move |_, msg: String| {
                println!("log: {}", msg);

                Ok(Value::Nil)
            })?,
        )?;

        Ok(Value::Table(table))
    }
}
