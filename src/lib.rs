mod error;
mod json;

use std::collections::HashSet;

use error::Error;
use ljr::prelude::*;

thread_local! {}

#[derive(Debug)]
pub struct Api {}

#[user_data]
impl Api {
    pub fn stringify(value: &StackValue) -> Result<String, Error> {
        let mut buf = String::new();
        let mut visited = HashSet::new();
        json::serialize_value(&mut buf, value, &mut visited)?;
        Ok(buf)
    }
}

#[ljr::module]
pub fn json(_lua: &Lua) -> Api {
    Api {}
}
