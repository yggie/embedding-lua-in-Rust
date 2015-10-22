extern crate libc;

use std::ptr;
use std::ffi::{ CStr, CString };
use libc::{ c_char, c_int, size_t };

// I don’t modify these directly so, it will probably be safe to assume have an
// undefined size. Always use LuaState instead!
#[repr(C)]
struct lua_State;

type LuaState = *const lua_State;

#[link(name="lua")]
extern {
    fn luaL_newstate() -> LuaState;

    fn luaL_openlibs(state: LuaState);
    fn luaL_loadstring(state: LuaState, string: *const c_char) -> c_int;
    fn lua_close(state: LuaState);

    fn lua_tolstring(state: LuaState, stack_index: c_int, string_length: *const size_t) -> *const c_char;

    fn lua_pcallk(state: LuaState, num_args: c_int, num_results: c_int, msg_handler: c_int, context: c_int, k: c_int) -> c_int;
    fn lua_settop(state: LuaState, stack_index: c_int);
}

unsafe fn lua_tostring(state: LuaState, stack_index: c_int) -> *const c_char {
    lua_tolstring(state, stack_index, ptr::null())
}

unsafe fn lua_pcall(state: LuaState, num_args: c_int, num_results: c_int, msg_handler: c_int) -> c_int {
    lua_pcallk(state, num_args, num_results, msg_handler, 0, 0)
}

unsafe fn lua_pop(state: LuaState, index: c_int) {
    lua_settop(state, -index - 1)
}

fn handle_error(state: LuaState, error_code: c_int) -> Result<(), ()> {
    if error_code != 0 {
        unsafe {
            let error_string = CStr::from_ptr(lua_tostring(state, -1));
            println!("error: {:?}", error_string.to_bytes());
            lua_pop(state, 1);
            return Err(());
        }
    }

    return Ok(());
}

fn main() {
    unsafe {
        let state = luaL_newstate();

        luaL_openlibs(state);

        if let Ok(_) = handle_error(state, luaL_loadstring(state, CString::new("print(\"Hello World!\")").unwrap().as_ptr())) {
            handle_error(state, lua_pcall(state, 0, 0, 0)).unwrap();
        }

        lua_close(state);
    }
}
