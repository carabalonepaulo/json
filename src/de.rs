use ljr::{Nil, table::StackTable, to_lua::ToLua};
use simd_json::{BorrowedValue, StaticNode, borrowed::Value};

pub fn insert_value(
    table: &mut StackTable,
    key: impl ToLua,
    value: &Value,
) -> Result<(), ljr::error::Error> {
    match value {
        BorrowedValue::Static(StaticNode::Null) => table.with_mut(|t| t.set(key, Nil)),
        BorrowedValue::Static(StaticNode::Bool(v)) => table.with_mut(|t| t.set(key, v)),
        BorrowedValue::Static(StaticNode::I64(v)) => table.with_mut(|t| t.set(key, *v as i32)),
        BorrowedValue::Static(StaticNode::U64(v)) => table.with_mut(|t| t.set(key, *v as i32)),
        BorrowedValue::Static(StaticNode::F64(v)) => table.with_mut(|t| t.set(key, v)),
        BorrowedValue::String(s) => table.with_mut(|t| t.set(key, s.as_bytes())),

        BorrowedValue::Array(vec) => {
            let len = vec.len();
            table.try_with_mut(|t| {
                t.try_add_table(key, len as _, 0, |t| {
                    let mut err: Option<ljr::error::Error> = None;
                    for (i, v) in vec.iter().enumerate() {
                        match insert_value(t, (i as i32) + 1, v) {
                            Ok(_) => {}
                            Err(e) => {
                                err = Some(e);
                                break;
                            }
                        }
                    }

                    match err {
                        Some(e) => Err(e),
                        None => Ok(()),
                    }
                })?
            })??;
        }
        BorrowedValue::Object(map) => {
            let len = map.len();
            table.try_with_mut(|t| {
                t.try_add_table(key, 0, len as _, |t| {
                    let mut err: Option<ljr::error::Error> = None;
                    for (k, v) in map.iter() {
                        match insert_value(t, k.as_bytes(), v) {
                            Ok(_) => {}
                            Err(e) => {
                                err = Some(e);
                                break;
                            }
                        }
                    }

                    match err {
                        Some(e) => Err(e),
                        None => Ok(()),
                    }
                })?
            })??;
        }
    }
    Ok(())
}
