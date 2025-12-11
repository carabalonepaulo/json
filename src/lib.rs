#![forbid(unsafe_code)]

mod de;
mod error;
mod se;

use std::sync::atomic::{AtomicI32, Ordering};

use error::Error;
use ljr::prelude::*;

use crate::de::try_to_value_ref;

static MAX_DEPTH: AtomicI32 = AtomicI32::new(128);

#[derive(Debug)]
pub struct Api {}

#[user_data]
impl Api {
    pub fn set_max_depth(depth: i32) {
        MAX_DEPTH.store(depth, Ordering::Relaxed);
    }

    pub fn stringify(value: &StackValue) -> Result<String, Error> {
        let max_depth = MAX_DEPTH.load(Ordering::Relaxed);
        let mut buf = String::with_capacity(128);
        se::serialize_value(&mut buf, value, max_depth)?;
        Ok(buf)
    }

    pub fn parse(text: &str, lua: &mut Lua) -> Result<ValueRef, Error> {
        let mut buf = text.as_bytes().to_vec();
        let value = simd_json::borrowed::to_value(&mut buf)?;
        try_to_value_ref(lua, &value)
    }
}

#[ljr::module]
pub fn json(_lua: &Lua) -> Api {
    Api {}
}
