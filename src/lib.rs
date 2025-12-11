#![forbid(unsafe_code)]

mod de;
mod error;
mod se;

use std::sync::atomic::{AtomicI32, Ordering};

use error::Error;
use ljr::prelude::*;

use crate::de::try_to_value_ref;

const MAX_SAFE_INT: i64 = 9_007_199_254_740_991;
const MAX_SAFE_INT_AS_F64: f64 = 9_007_199_254_740_991.0;

static MAX_DEPTH: AtomicI32 = AtomicI32::new(128);

#[inline(always)]
pub(crate) fn check_depth(mut depth: i32) -> Result<i32, Error> {
    depth -= 1;
    if depth < 0 {
        Err(Error::MaxDepthExceeded)
    } else {
        Ok(depth)
    }
}

#[derive(Debug)]
pub struct Json {}

#[user_data]
impl Json {
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
        let max_depth = MAX_DEPTH.load(Ordering::Relaxed);
        let mut buf = text.as_bytes().to_vec();
        let value = simd_json::borrowed::to_value(&mut buf)?;
        try_to_value_ref(lua, &value, max_depth)
    }
}

#[ljr::module]
fn json(_lua: &Lua) -> Json {
    Json {}
}
