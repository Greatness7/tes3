use crate::prelude::*;

mod reference;
pub use reference::*;

mod getter;
pub use getter::*;

mod userdata;

mod lowercase_ids;

pub use mlua::{prelude::*, IntoLua, Nil, UserData, UserDataFields, UserDataMethods};

fn load_plugin(_: &Lua, (path, options): (String, Option<LuaTable>)) -> LuaResult<Ref<&'static Plugin>> {
    let mut lowercase_ids = false;
    let mut ignored_types: Vec<[u8; 4]> = vec![];

    if let Some(options) = options {
        lowercase_ids = options.get("lowercase_ids").unwrap_or_default();
        let ignore: Vec<String> = options.get("ignored_types").unwrap_or_default();
        for s in ignore {
            match <[u8; 4]>::try_from(s.as_bytes()) {
                Ok(ok) => ignored_types.push(ok),
                Err(e) => return Err(LuaError::external(e)),
            }
        }
    }

    let filter = |tag| !ignored_types.contains(&tag);

    let mut plugin = Plugin::from_path_filtered(path, filter) //
        .map_err(LuaError::external)?;

    if lowercase_ids {
        plugin.lowercase_ids();
    }

    Ok(plugin.into_lua_ref())
}

pub fn lua_module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set("load_plugin", lua.create_function(load_plugin)?)?;

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
