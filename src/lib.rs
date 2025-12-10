mod de;
mod error;
mod se;

use std::collections::HashSet;

use error::Error;
use ljr::prelude::*;

use crate::de::DeValue;

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

    pub fn parse(text: &str) -> Result<DeValue, Error> {
        DeValue::new(text)
    }
}

#[ljr::module]
pub fn json(_lua: &Lua) -> Api {
    Api {}
}
