use crate::prelude::*;

mod reference;
pub use reference::*;

mod getter;
pub use getter::*;

mod userdata;

pub use mlua::{prelude::*, IntoLua, Nil, UserData, UserDataFields, UserDataMethods};

pub fn lua_module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set(
        "load_plugin",
        lua.create_function(|_, path: String| {
            let plugin = Plugin::from_path(path).map_err(LuaError::external)?;
            Ok(plugin.into_lua_ref())
        })?,
    )?;

    Ok(exports)
}

/// Trait implemented on primitive (value) lua types which always clone.
///
pub trait LuaPrimitive: Clone {
    fn clone_into_lua(&self, lua: &Lua) -> LuaResult<LuaValue>;
}

macro_rules! impl_primitives {
    ($($T:ty)*) => {
        $(
            impl LuaPrimitive for $T {
                fn clone_into_lua(&self, lua: &Lua) -> LuaResult<LuaValue> {
                    self.clone().into_lua(lua)
                }
            }
        )*
    }
}
impl_primitives! { i8 u8 i16 u16 f32 i32 u32 f64 i64 u64 bool String }

impl<const N: usize> LuaPrimitive for FixedString<N> {
    fn clone_into_lua(&self, lua: &Lua) -> LuaResult<LuaValue> {
        self.0.clone_into_lua(lua)
    }
}
