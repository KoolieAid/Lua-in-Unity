use mlua::prelude::*;
use std::ffi::c_void;
use std::ffi::{c_char, CStr, CString};

#[no_mangle]
pub extern "C" fn init_lua(debug: extern "C" fn(*mut c_char)) -> *mut c_void {
    let lua = Box::new(Lua::new());

    let func = lua
        .create_function(move |_, s: String| {
            let cstr = CString::new(s).unwrap();
            debug(cstr.into_raw());
            Ok(())
        })
        .unwrap();

    lua.globals().set("debuglog", func).unwrap();

    Box::into_raw(lua).cast()
}

#[no_mangle]
pub extern "C" fn destroy_lua(lua: *mut c_void) {
    unsafe { Box::<Lua>::from_raw(lua.cast()) };
}

#[no_mangle]
pub extern "C" fn register_function(
    lua: *mut c_void,
    name: *const i8,

    func: Option<extern "C" fn()>,
) {
    let lua = unsafe { Box::<Lua>::from_raw(lua.cast::<Lua>()) };

    let name = unsafe { CStr::from_ptr(name) };

    match register_func(&lua, name.to_str().unwrap(), func.unwrap()) {
        Ok(_) => {}
        Err(e) => println!("Error: {e:?}"),
    }

    Box::into_raw(lua);
}

fn register_func(lua: &Lua, name: &str, f: extern "C" fn()) -> LuaResult<()> {
    let fun = lua.create_function(move |_, _: ()| {
        f();
        Ok(())
    })?;

    lua.globals().set(name, fun)?;

    Ok(())
}

#[no_mangle]
fn execute(lua: *mut c_void, chunk: *const i8) {
    let chunk = unsafe { CStr::from_ptr(chunk) };

    let lua = unsafe { Box::from_raw(lua.cast::<Lua>()) };

    if let Err(e) = lua.load(chunk.to_str().unwrap_or_default()).exec() {
        match e {
            LuaError::RuntimeError(s) => {
                let func: LuaFunction = lua.globals().get("debuglog").unwrap();
                func.call::<_, ()>(s).unwrap();
            }
            err => {
                let func: LuaFunction = lua.globals().get("debuglog").unwrap();
                func.call::<_, ()>(err.to_string()).unwrap();
            }
        }
    }

    Box::into_raw(lua);
}

#[no_mangle]
pub unsafe extern "C" fn free_string(string: *mut c_char) {
    let cstr = unsafe { CString::from_raw(string) };
    drop(cstr);
}
