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

    let owned = CString::new("test").unwrap();

    let string = owned.as_c_str();

    register_function(lua, string.as_ptr(), Some(empty));

    destroy_lua(lua);
}

#[test]
fn calling_function() {
    let lua = init_lua(listener);

    let owned = CString::new("test").unwrap();

    let string = owned.as_c_str();

    register_function(lua, string.as_ptr(), Some(empty));

    let raw = CString::new(
        r#"
        test("test")
        "#,
    )
    .unwrap();

    let cstr = raw.as_c_str();

    execute(lua, cstr.as_ptr());

    destroy_lua(lua);
}
#[test]
fn calling_function_error() {
    let lua = init_lua(listener);

    let owned = CString::new("test").unwrap();

    let string = owned.as_c_str();

    register_function(lua, string.as_ptr(), Some(empty));

    let raw = CString::new(
        r#"
        non_existent("test")
        "#,
    )
    .unwrap();

    let cstr = raw.as_c_str();

    execute(lua, cstr.as_ptr());

    destroy_lua(lua);
}
