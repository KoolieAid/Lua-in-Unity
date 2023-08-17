use super::*;
use std::ptr::null_mut;

#[test]
fn initialization() {
    let lua = init_lua(listener);

    assert!(lua != null_mut());

    destroy_lua(lua);
}

extern "C" fn listener(ch: *mut c_char) {
    let str = unsafe { CStr::from_ptr(ch) };
    println!("Output: {str:?}");

    unsafe {
        free_string(str.as_ptr().cast_mut());
    }
}

extern "C" fn empty() {}

#[test]
fn registering_function() {
    let lua = init_lua(listener);

    let function_name = CString::new("test").unwrap();

    register_function(lua, function_name.as_ptr(), Some(empty));

    destroy_lua(lua);
}

#[test]
fn calling_function() {
    let lua = init_lua(listener);

    let func_name = CString::new("test").unwrap();

    register_function(lua, func_name.as_ptr(), Some(empty));

    let code = CString::new(
        r#"
        test("test")
        "#,
    )
    .unwrap();

    execute(lua, code.as_ptr());

    destroy_lua(lua);
}

#[test]
fn calling_function_error() {
    let lua = init_lua(listener);

    let func_name = CString::new("test").unwrap();

    register_function(lua, func_name.as_ptr(), Some(empty));

    let code = CString::new(
        r#"
        non_existent("test")
        "#,
    )
    .unwrap();

    execute(lua, code.as_ptr());

    destroy_lua(lua);
}
