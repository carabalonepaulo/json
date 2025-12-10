mod de;
mod error;
mod se;

use std::collections::HashSet;

use error::Error;
use ljr::prelude::*;

use crate::de::try_to_value_ref;

#[derive(Debug)]
pub struct Api {}

#[user_data]
impl Api {
    pub fn stringify(value: &StackValue) -> Result<String, Error> {
        let mut buf = String::new();
        let mut visited = HashSet::new();
        se::serialize_value(&mut buf, value, &mut visited)?;
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
