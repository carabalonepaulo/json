use ljr::{prelude::*, value::Kind};

use crate::error::Error;

pub fn serialize_value(buf: &mut String, value: &StackValue, depth: i32) -> Result<(), Error> {
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
            value.try_with_str(|s| {
                buf.push('"');
                buf.extend(json_escape::escape_str(s.as_str()));
                buf.push('"');
            })?;
            Ok(())
        }
        Kind::Table => value.with_table(|t| serialize_table(buf, t, depth)),
        _ => Err(Error::UnsupportedValue),
    }
}

pub fn serialize_table(buf: &mut String, table: &StackTable, mut depth: i32) -> Result<(), Error> {
    if depth <= 0 {
        return Err(Error::MaxDepthExceeded);
    }
    depth -= 1;

    let len = table.len();
    if len > 0 {
        serialize_array(buf, table, depth)
    } else {
        serialize_object(buf, table, depth)
    }
}

pub fn serialize_array(buf: &mut String, table: &StackTable, depth: i32) -> Result<(), Error> {
    buf.push('[');

    let mut err: Option<Error> = None;
    table.try_with(|t| {
        t.try_for_each_indexed(|i: i32, v: &StackValue| {
            if i > 1 {
                buf.push(',');
            }

            if let Err(e) = serialize_value(buf, v, depth) {
                err = Some(e);
                false
            } else {
                true
            }
        })
    })??;

    if let Some(err) = err {
        return Err(err);
    }

    buf.push(']');
    Ok(())
}

pub fn serialize_object(buf: &mut String, table: &StackTable, depth: i32) -> Result<(), Error> {
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
        match serialize_value(buf, v, depth) {
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
