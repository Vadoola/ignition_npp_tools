//At some point it might be nice to figure out how to get bindgen working
//and clean up the manually recreated compmonents
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
/*#![warn(
    clippy::all,
    clippy::pedantic,
    //clippy::cargo,
)]*/

extern crate core;

use def::{to_wide_chars, FuncItem, NppData, Tchar};
use once_cell::sync::OnceCell;
use std::ffi::{c_uint, c_void};
use windows::Win32::Foundation::{HANDLE, LPARAM, LRESULT, WPARAM};

mod def;
mod functions;
mod plugindata;

static PROG_NAME: OnceCell<Vec<u16>> = OnceCell::new();
static mut FUNC_ITEMS: OnceCell<Vec<FuncItem>> = OnceCell::new();

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
pub unsafe extern "C" fn getFuncsArray(nbF: *mut i32) -> *const FuncItem {
    unsafe {
        *nbF = FUNC_ITEMS
            .get_or_init(|| vec![plugindata::FuncItem_MovePipes()])
            .len() as i32
    };
    FUNC_ITEMS.get().unwrap().as_ptr()
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
    if index < unsafe { FUNC_ITEMS.get().unwrap().len() } {
        unsafe {
            FUNC_ITEMS.get_mut().unwrap()[index]._pFunc = pFunc;
            FUNC_ITEMS.get_mut().unwrap()[index]._init2Check = checkOnInit;
            FUNC_ITEMS.get_mut().unwrap()[index]._pShKey = sk;
        }
        true
    } else {
        false
    }
}
