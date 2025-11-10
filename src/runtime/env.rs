use std::sync::{Arc, Mutex};

use rlua::{ToLua, Value};

use crate::runtime::Instruction;

#[derive(Default)]
pub(crate) struct Env {
    pub(crate) inst: Arc<Mutex<Vec<Instruction>>>,
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

                inst.push(Instruction::Record);

                Ok(Value::Nil)
            })?,
        )?;

        let inst = self.inst.clone();
        table.set(
            "report",
            lua.create_function(move |_, _: ()| {
                let Ok(mut inst) = inst.lock() else {
                    return Err(rlua::Error::runtime("failed to lock instructions"));
                };

                inst.push(Instruction::Alert);

                Ok(Value::Nil)
            })?,
        )?;

        let inst = self.inst.clone();
        table.set(
            "exec",
            lua.create_function(move |_, cmd: String| {
                let Ok(mut inst) = inst.lock() else {
                    return Err(rlua::Error::runtime("failed to lock instructions"));
                };

                inst.push(Instruction::Exec(cmd));

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
