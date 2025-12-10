use ljr::{Nil, prelude::TableView, to_lua::ToLua};
use simd_json::{BorrowedValue, StaticNode, borrowed::Value};

pub fn insert_value(
    t: &mut TableView,
    key: impl ToLua,
    value: &Value,
) -> Result<(), ljr::error::Error> {
    match value {
        BorrowedValue::Static(StaticNode::Null) => t.set(key, Nil),
        BorrowedValue::Static(StaticNode::Bool(v)) => t.set(key, v),
        BorrowedValue::Static(StaticNode::I64(v)) => t.set(key, *v as i32),
        BorrowedValue::Static(StaticNode::U64(v)) => t.set(key, *v as i32),
        BorrowedValue::Static(StaticNode::F64(v)) => t.set(key, v),
        BorrowedValue::String(s) => t.set(key, s.as_bytes()),

        BorrowedValue::Array(vec) => {
            let len = vec.len();
            t.try_add_table(key, len as _, 0, |t| {
                let t = &mut *t.as_mut();
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
            })??;
        }
        BorrowedValue::Object(map) => {
            let len = map.len();
            t.try_add_table(key, 0, len as _, |t| {
                let t = &mut *t.as_mut();
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
            })??;
        }
    }
    Ok(())
}
