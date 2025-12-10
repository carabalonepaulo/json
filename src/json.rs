use std::collections::HashSet;

use ljr::{prelude::*, value::Kind};

use crate::error::Error;

pub fn serialize_value(
    buf: &mut String,
    value: &StackValue,
    visited: &mut HashSet<usize>,
) -> Result<(), Error> {
    match value.kind() {
        Kind::Nil => {
            buf.push_str("null");
            Ok(())
        }
        Kind::Bool => {
            buf.push_str(if value.as_bool() { "true" } else { "false" });
            Ok(())
        }
        Kind::Number => {
            value.with_number(|n| {
                let mut buffer = ryu::Buffer::new();
                buf.push_str(buffer.format(n));
            });
            Ok(())
        }
        Kind::String => {
            value.with_str(|s| {
                buf.push('"');
                buf.extend(json_escape::escape_str(s.as_str()));
                buf.push('"');
            });
            Ok(())
        }
        Kind::Table => value.with_table(|t| serialize_table(buf, t, visited)),
        _ => Err(Error::UnsupportedValue),
    }
}

pub fn serialize_table(
    buf: &mut String,
    table: &StackTable,
    visited: &mut HashSet<usize>,
) -> Result<(), Error> {
    if visited.contains(&table.id()) {
        return Err(Error::CyclicValue);
    }
    visited.insert(table.id());

    let len = table.len();
    if len > 0 {
        serialize_array(buf, table, visited)
    } else {
        serialize_object(buf, table, visited)
    }
}

pub fn serialize_array(
    buf: &mut String,
    table: &StackTable,
    visited: &mut HashSet<usize>,
) -> Result<(), Error> {
    buf.push('[');

    let mut err: Option<Error> = None;
    table.for_each(|i: &i32, v: &StackValue| {
        if *i > 1 {
            buf.push(',');
        }
        match serialize_value(buf, v, visited) {
            Ok(_) => true,
            Err(e) => {
                err = Some(e);
                false
            }
        }
    });

    if let Some(err) = err {
        return Err(err);
    }

    buf.push(']');
    Ok(())
}

pub fn serialize_object(
    buf: &mut String,
    table: &StackTable,
    visited: &mut HashSet<usize>,
) -> Result<(), Error> {
    buf.push('{');

    let mut err: Option<Error> = None;
    let mut first = true;
    table.for_each(|k: &StackStr, v: &StackValue| {
        if !first {
            buf.push(',');
        }
        first = false;

        buf.push('"');
        buf.extend(json_escape::escape_str(k.as_str()));
        buf.push('"');
        buf.push(':');
        match serialize_value(buf, v, visited) {
            Ok(_) => true,
            Err(e) => {
                err = Some(e);
                false
            }
        }
    });

    buf.push('}');
    Ok(())
}
