use mlua::prelude::*;
use std::ffi::c_void;
use std::ffi::{c_char, CStr, CString};

#[cfg(test)]
mod tests;

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

extern "C" fn empty_func() {}

#[no_mangle]
pub extern "C" fn register_function(
    lua: *mut c_void,
    name: *const c_char,
    func: Option<extern "C" fn()>,
) {
    let lua = unsafe { Box::<Lua>::from_raw(lua.cast()) };

    let name = unsafe { CStr::from_ptr(name) };

    match register_func(
        &lua,
        name.to_str().unwrap(),
        func.unwrap_or_else(|| {
            let er_func: LuaFunction = lua.globals().get("debuglog").unwrap();

            let _ = er_func.call::<_, ()>("No function provided. Using empty body instead.");
            empty_func
        }),
    ) {
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
fn execute(lua: *mut c_void, chunk: *const c_char) {
    let chunk = unsafe { CStr::from_ptr(chunk) };

    let lua = unsafe { Box::from_raw(lua.cast::<Lua>()) };

    if let Err(e) = lua.load(chunk.to_str().unwrap_or_default()).exec() {
        let arg = match e {
            LuaError::RuntimeError(s) => s,
            err => err.to_string(),
        };

        let func: LuaFunction = lua.globals().get("debuglog").unwrap();
        func.call::<_, ()>(arg).unwrap();
    }

    Box::into_raw(lua);
}

#[no_mangle]
pub unsafe extern "C" fn free_string(string: *mut c_char) {
    let cstr = unsafe { CString::from_raw(string) };
    drop(cstr);
}
