//use crate::def::to_wide_chars;

use crate::plugindata::NPP_DATA;
use regex::{Captures, Regex};

use std::ffi::{c_int, CString};

use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{/*MB_OK, MessageBoxW, */SendMessageW};

slint::include_modules!();

fn replace_all<E>(
    re: &Regex,
    haystack: &str,
    replacement: impl Fn(&Captures) -> Result<String, E>,
) -> Result<String, E> {
    let mut new = String::with_capacity(haystack.len());
    let mut last_match = 0;
    for caps in re.captures_iter(haystack) {
        let m = caps.get(0).unwrap();
        new.push_str(&haystack[last_match..m.start()]);
        new.push_str(&replacement(&caps)?);
        last_match = m.end();
    }
    new.push_str(&haystack[last_match..]);
    Ok(new)
}

/*unsafe {
    MessageBoxW(HWND(0), PCWSTR(to_wide_chars("Hello Notepad++ From Rust").as_ptr()), PCWSTR(to_wide_chars("Rust Plugin").as_ptr()), MB_OK);
}*/

pub extern "C" fn move_objects() {
    use crate::def::npp::NPPM_GETCURRENTSCINTILLA;
    use crate::def::sci::{SCI_GETLENGTH, SCI_GETTEXT, SCI_SETTEXT};

    let dlg = Dialog::new().unwrap();

    let close_handle = dlg.as_weak();
    dlg.on_close(move || {
        let close_handle = close_handle.upgrade().unwrap();
        let x_shift = close_handle.get_x_shift();
        let y_shift = close_handle.get_y_shift();

        let test = unsafe { NPP_DATA.clone().unwrap() };
        let mut which: c_int = -1;

        let which_ffi: *mut c_int = &mut which;
        unsafe {
            SendMessageW(
                test._nppHandle,
                NPPM_GETCURRENTSCINTILLA,
                WPARAM(0),
                LPARAM(which_ffi as isize),
            );
        }

        if which != -1 {
            let curScintilla: HWND = if which == 0 {
                test._scintillaMainHandle
            } else {
                test._scintillaSecondHandle
            };

            let bytes = unsafe { SendMessageW(curScintilla, SCI_GETLENGTH, WPARAM(0), LPARAM(0)) };
            if bytes.0 > 0 {
                let bytes = bytes.0 as usize;
                let mut buffer: Vec<u8> = Vec::with_capacity(bytes + 1);
                let _my_res = unsafe {
                    buffer.set_len(bytes + 1);
                    SendMessageW(
                        curScintilla,
                        SCI_GETTEXT,
                        WPARAM(bytes),
                        LPARAM(buffer.as_mut_ptr() as isize),
                    )
                };
                let buffer = CString::from_vec_with_nul(buffer).unwrap();

                let document = buffer.into_string().unwrap();
                let x_coord_re = Regex::new(r#""x": (\d*[\.\d]+)"#).unwrap();
                let y_coord_re = Regex::new(r#""y": (\d*[\.\d]+)"#).unwrap();

                let output = replace_all(&x_coord_re, &document, |caps: &Captures| {
                    if let Ok(my_float) = caps[1].parse::<f32>() {
                        Ok(format!("\"x\": {}", my_float + x_shift))
                    } else {
                        Err("Failed to convert")
                    }
                })
                .unwrap();

                let output = replace_all(&y_coord_re, &output, |caps: &Captures| {
                    if let Ok(my_float) = caps[1].parse::<f32>() {
                        Ok(format!("\"y\": {}", my_float + y_shift))
                    } else {
                        Err("Failed to convert")
                    }
                })
                .unwrap();

                let output = CString::new(output.as_str()).unwrap();
                unsafe {
                    SendMessageW(
                        curScintilla,
                        SCI_SETTEXT,
                        WPARAM(0),
                        LPARAM(output.into_raw() as isize),
                    );
                }
            }
        }
        close_handle.hide().unwrap();
    });

    dlg.run().unwrap();
}
