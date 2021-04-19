use mlua::{Error, ExternalError, Function, Lua, LuaSerdeExt, Nil, Result, Table, Value};
use serde::Serialize;

/// Decodes input value (must be a string) that represents a yaml document to a Lua value
fn decode<'lua>(lua: &'lua Lua, s: Value<'lua>) -> Result<Value<'lua>> {
    let s = match s {
        Value::String(ref s) => Ok(s.as_bytes()),
        _ => Err(format!("invalid input type: {}", s.type_name()).to_lua_err()),
    }?;
    let yaml_value = serde_yaml::from_slice::<serde_yaml::Value>(s)
        .map_err(|e| Error::external(e.to_string()))?;
    lua.to_value(&yaml_value)
}

/// Encodes Lua value (any supported) to a yaml document
fn encode<'lua>(lua: &'lua Lua, v: Value<'lua>) -> Result<Value<'lua>> {
    let mut buf = Vec::new();
    v.serialize(&mut serde_yaml::Serializer::new(&mut buf))
        .map_err(|e| Error::external(e.to_string()))?;
    lua.create_string(&buf).map(Value::String)
}

fn make_exports<'lua>(
    lua: &'lua Lua,
    decode: Function<'lua>,
    encode: Function<'lua>,
) -> Result<Table<'lua>> {
    let exports = lua.create_table()?;
    exports.set("load", decode.clone())?;
    exports.set("decode", decode)?;
    exports.set("dump", encode.clone())?;
    exports.set("encode", encode)?;
    exports.set("null", lua.null()?)?;
    exports.set("array_mt", lua.array_metatable()?)?;
    Ok(exports)
}

#[mlua::lua_module]
fn ryaml(lua: &Lua) -> Result<Table> {
    let decode = lua.create_function(decode)?;
    let encode = lua.create_function(encode)?;
    make_exports(lua, decode, encode)
}

#[mlua::lua_module]
fn ryaml_safe(lua: &Lua) -> Result<Table> {
    let decode = lua.create_function(|lua, s| match decode(lua, s) {
        Ok(v) => Ok((v, None)),
        Err(e) => Ok((Nil, Some(e.to_string()))),
    })?;
    let encode = lua.create_function(|lua, v| match encode(lua, v) {
        Ok(s) => Ok((s, None)),
        Err(e) => Ok((Nil, Some(e.to_string()))),
    })?;
    make_exports(lua, decode, encode)
}
