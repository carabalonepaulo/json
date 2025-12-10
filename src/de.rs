use ljr::{prelude::StackGuard, sys, to_lua::ToLua};
use simd_json::{BorrowedValue, StaticNode, borrowed::Value};

#[allow(unused)]
pub struct DeValue {
    value: Value<'static>,
    buf: Vec<u8>,
}

impl DeValue {
    pub fn new(text: &str) -> Result<Self, crate::error::Error> {
        let mut buf = text.as_bytes().to_vec();
        let value = simd_json::borrowed::to_value(&mut buf)?;
        let value: Value<'static> = unsafe { std::mem::transmute(value) };
        Ok(Self { value, buf })
    }
}

unsafe impl ToLua for DeValue {
    const LEN: i32 = 1;

    unsafe fn try_to_lua_unchecked(
        self,
        ptr: *mut ljr::sys::lua_State,
    ) -> Result<(), ljr::prelude::Error> {
        to_lua(ptr, &self.value)?;
        Ok(())
    }
}

fn to_lua(ptr: *mut sys::lua_State, val: &Value) -> Result<(), ljr::error::Error> {
    match val {
        BorrowedValue::Static(StaticNode::Null) => unsafe { sys::lua_pushnil(ptr) },
        BorrowedValue::Static(StaticNode::Bool(v)) => unsafe {
            sys::lua_pushboolean(ptr, if *v { 1 } else { 0 })
        },
        BorrowedValue::Static(StaticNode::I64(v)) => unsafe { sys::lua_pushinteger(ptr, *v) },
        BorrowedValue::Static(StaticNode::U64(v)) => unsafe { sys::lua_pushinteger(ptr, *v as _) },
        BorrowedValue::Static(StaticNode::F64(v)) => unsafe { sys::lua_pushnumber(ptr, *v) },

        BorrowedValue::String(s) => unsafe { sys::lua_pushlstring_(ptr, s.as_ptr() as _, s.len()) },
        BorrowedValue::Array(vec) => unsafe {
            ljr::helper::try_check_stack(ptr, 3)?;
            let guard = StackGuard::new(ptr);

            sys::lua_createtable(ptr, vec.len() as _, 0);

            for (i, v) in vec.iter().enumerate() {
                to_lua(ptr, v)?;
                sys::lua_rawseti_(ptr, -2, (i + 1) as _);
            }

            guard.commit();
        },
        BorrowedValue::Object(map) => unsafe {
            ljr::helper::try_check_stack(ptr, 3)?;
            let guard = StackGuard::new(ptr);

            sys::lua_createtable(ptr, 0, map.len() as _);

            for (k, v) in map.iter() {
                sys::lua_pushlstring_(ptr, k.as_ptr() as _, k.len());
                to_lua(ptr, v)?;
                sys::lua_rawset(ptr, -3);
            }

            guard.commit();
        },
    }

    Ok(())
}
