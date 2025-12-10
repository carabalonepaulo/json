use std::borrow::Cow;

use crate::Error;
use halfbrown::SizedHashMap;
use ljr::{Nil, lua::Lua, prelude::TableView, to_lua::ToLua, value::ValueRef};
use simd_json::{BorrowedValue, StaticNode, borrowed::Value};

const MAX_SAFE_INT: i64 = 9_007_199_254_740_991;

#[inline(always)]
fn push_vec(t: &mut TableView, vec: &Vec<Value<'_>>) -> Result<(), Error> {
    for (i, v) in vec.iter().enumerate() {
        insert_value(t, Some((i as i32) + 1), v)?;
    }
    Ok(())
}

#[inline(always)]
fn push_map(t: &mut TableView, map: &SizedHashMap<Cow<'_, str>, Value<'_>>) -> Result<(), Error> {
    for (k, v) in map.iter() {
        insert_value(t, Some(k.as_bytes()), v)?;
    }
    Ok(())
}

fn insert_value(t: &mut TableView, key: impl ToLua, value: &Value) -> Result<(), Error> {
    match value {
        BorrowedValue::Static(StaticNode::Null) => t.try_set(key, Nil)?,
        BorrowedValue::Static(StaticNode::Bool(v)) => t.try_set(key, v)?,
        BorrowedValue::Static(StaticNode::I64(v)) => {
            if let Ok(value) = i32::try_from(*v) {
                t.try_set(key, value)?
            } else if *v >= -MAX_SAFE_INT && *v <= MAX_SAFE_INT {
                t.try_set(key, *v as f64)?
            } else {
                return Err(Error::IntOutOfRange);
            }
        }
        BorrowedValue::Static(StaticNode::U64(v)) => {
            if let Ok(value) = i32::try_from(*v) {
                t.try_set(key, value)?
            } else if *v < (MAX_SAFE_INT as u64) {
                t.try_set(key, *v as f64)?
            } else {
                return Err(Error::IntOutOfRange);
            }
        }
        BorrowedValue::Static(StaticNode::F64(v)) => t.try_set(key, v)?,
        BorrowedValue::String(s) => t.try_set(key, s.as_bytes())?,
        BorrowedValue::Array(vec) => {
            let len = vec.len();
            t.try_add_table(key, len as _, 0, |t| {
                let t = &mut *t.as_mut();
                push_vec(t, &**vec)
            })??;
        }
        BorrowedValue::Object(map) => {
            let len = map.len();
            t.try_add_table(key, 0, len as _, |t| {
                let t = &mut *t.as_mut();
                push_map(t, &**map)
            })??;
        }
    }
    Ok(())
}

pub fn try_to_value_ref(lua: &Lua, value: &Value) -> Result<ValueRef, Error> {
    match value {
        BorrowedValue::Static(StaticNode::Null) => Ok(lua.try_create_value_ref(Nil)?),
        BorrowedValue::Static(StaticNode::Bool(v)) => Ok(lua.try_create_value_ref(*v)?),
        BorrowedValue::Static(StaticNode::I64(v)) => {
            if let Ok(value) = i32::try_from(*v) {
                Ok(lua.try_create_value_ref(value)?)
            } else if *v >= -MAX_SAFE_INT && *v <= MAX_SAFE_INT {
                Ok(lua.try_create_value_ref(*v as f64)?)
            } else {
                Err(Error::IntOutOfRange)
            }
        }
        BorrowedValue::Static(StaticNode::U64(v)) => {
            if let Ok(value) = i32::try_from(*v) {
                Ok(lua.try_create_value_ref(value)?)
            } else if *v < (MAX_SAFE_INT as u64) {
                Ok(lua.try_create_value_ref(*v as f64)?)
            } else {
                Err(Error::IntOutOfRange)
            }
        }
        BorrowedValue::Static(StaticNode::F64(v)) => Ok(lua.try_create_value_ref(*v)?),
        BorrowedValue::String(s) => Ok(lua.try_create_value_ref(s.as_bytes())?),
        BorrowedValue::Array(vec) => {
            let len = vec.len();
            let mut table = lua.try_create_table_with_capacity(len as _, 0)?;
            table.try_with_mut(|t| push_vec(t, &**vec))??;
            Ok(lua.try_create_value_ref(table)?)
        }
        BorrowedValue::Object(map) => {
            let len = map.len();
            let mut table = lua.try_create_table_with_capacity(0, len as _)?;
            table.try_with_mut(|t| push_map(t, &**map))??;
            Ok(lua.try_create_value_ref(table)?)
        }
    }
}
