use std::borrow::Cow;

use crate::Error;
use halfbrown::SizedHashMap;
use ljr::{Nil, prelude::TableView, to_lua::ToLua};
use simd_json::{BorrowedValue, StaticNode, borrowed::Value};

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

pub fn insert_value(
    t: &mut TableView,
    key: Option<impl ToLua>,
    value: &Value,
) -> Result<(), Error> {
    match value {
        BorrowedValue::Static(StaticNode::Null) => {
            let key = key.ok_or(Error::InvalidKey)?;
            t.try_set(key, Nil)?;
        }
        BorrowedValue::Static(StaticNode::Bool(v)) => {
            let key = key.ok_or(Error::InvalidKey)?;
            t.try_set(key, v)?
        }
        BorrowedValue::Static(StaticNode::I64(v)) => {
            let key = key.ok_or(Error::InvalidKey)?;
            let value = i32::try_from(*v).map_err(|_| Error::IntOutOfRange)?;
            t.try_set(key, value)?
        }
        BorrowedValue::Static(StaticNode::U64(v)) => {
            let key = key.ok_or(Error::InvalidKey)?;
            let value = i32::try_from(*v).map_err(|_| Error::IntOutOfRange)?;
            t.try_set(key, value)?;
        }
        BorrowedValue::Static(StaticNode::F64(v)) => {
            let key = key.ok_or(Error::InvalidKey)?;
            t.try_set(key, v)?
        }
        BorrowedValue::String(s) => {
            let key = key.ok_or(Error::InvalidKey)?;
            t.try_set(key, s.as_bytes())?;
        }
        BorrowedValue::Array(vec) => {
            let len = vec.len();
            if let Some(key) = key {
                t.try_add_table(key, len as _, 0, |t| {
                    let t = &mut *t.as_mut();
                    push_vec(t, &**vec)
                })??;
            } else {
                push_vec(t, &**vec)?;
            }
        }
        BorrowedValue::Object(map) => {
            let len = map.len();
            if let Some(key) = key {
                t.try_add_table(key, 0, len as _, |t| {
                    let t = &mut *t.as_mut();
                    push_map(t, &**map)
                })??;
            } else {
                push_map(t, &**map)?;
            }
        }
    }
    Ok(())
}
