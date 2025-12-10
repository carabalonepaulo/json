mod de;
mod error;
mod se;

use std::collections::HashSet;

use error::Error;
use ljr::prelude::*;

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

    pub fn parse(text: &str, lua: &mut Lua) -> Result<TableRef, Error> {
        let mut buf = text.as_bytes().to_vec();
        let value = simd_json::borrowed::to_value(&mut buf)?;

        let mut table = lua.try_create_table()?;
        table.try_with_mut(|view| de::insert_value(view, 1, &value))??;

        Ok(table.try_with(|t| t.try_get(1))??)
    }
}

#[ljr::module]
pub fn json(_lua: &Lua) -> Api {
    Api {}
}
