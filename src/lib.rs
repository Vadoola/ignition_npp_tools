//At some point it might be nice to figure out how to get bindgen working
//and clean up the manually recreated compmonents
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//I like to put these in sometimes to see what else I might be able to clean up
//This generates a LOT off warnings in the Slint auto generated code, so I'm leving
//it commented out for now
/*#![warn(
    clippy::all,
    clippy::pedantic,
    //clippy::cargo,
)]*/

extern crate core;

use def::{to_wide_chars, FuncItem, NppData, Tchar};
use once_cell::sync::OnceCell;
use std::ffi::{c_int, c_uint, c_void};
use windows::Win32::Foundation::{HANDLE, LPARAM, LRESULT, WPARAM};

mod def;
mod functions;
mod plugindata;

static PROG_NAME: OnceCell<Vec<u16>> = OnceCell::new();
static mut FUNC_ITEMS: OnceCell<Vec<FuncItem>> = OnceCell::new();

unsafe fn get_func_items() -> &'static Vec<FuncItem> {
    unsafe { FUNC_ITEMS.get_or_init(|| vec![plugindata::FuncItem_MovePipes()]) }
}

#[no_mangle]
pub extern "C" fn isUnicode() -> bool {
    true
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn setInfo(notpadPlusData: NppData) {
    unsafe {
        plugindata::NPP_DATA = Some(notpadPlusData);
    }
}

#[no_mangle]
pub extern "C" fn getName() -> *const Tchar {
    //PROG_NAME.as_ptr()
    PROG_NAME
        .get_or_init(|| to_wide_chars("Ignition Tools"))
        .as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn getFuncsArray(nbF: *mut c_int) -> *const FuncItem {
    unsafe { *nbF = get_func_items().len() as c_int };

    get_func_items().as_ptr()
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn beNotified(notifyCode: *mut c_void) {}

#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn messageProc(Message: c_uint, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    LRESULT(1)
}

#[no_mangle]
pub extern "C" fn pluginInit(_hModule: HANDLE) {}

#[no_mangle]
pub extern "C" fn pluginCleanUp() {}

#[no_mangle]
pub extern "C" fn commandMenuInit() {
    setCommand(
        0,
        to_wide_chars("Move Pipes").as_ptr(),
        functions::move_objects,
        0,
        false,
    );
}

#[no_mangle]
pub extern "C" fn commandMenuCleanUp() {}

#[no_mangle]
pub extern "C" fn setCommand(
    index: usize,
    _cmdName: *const Tchar,
    pFunc: extern "C" fn(),
    sk: usize,
    checkOnInit: bool,
) -> bool {
    if let Some(fi) = unsafe { FUNC_ITEMS.get_mut() } {
        if let Some(func) = fi.get_mut(index) {
            func.pFunc = pFunc;
            func.init2Check = checkOnInit;
            func.pShKey = sk;
            true
        } else {
            false
        }
    } else {
        false
    }
}
