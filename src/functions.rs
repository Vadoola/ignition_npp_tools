use crate::def::to_wide_chars;
use crate::plugindata::NPP_DATA;
use regex::{Captures, Regex};
use std::ffi::{c_int, CString};
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{HWND, LPARAM, WPARAM},
        UI::WindowsAndMessaging::{MessageBoxW, SendMessageW, MB_OK},
    },
};

slint::include_modules!();

fn replace_all<E>(
    re: &Regex,
    haystack: &str,
    replacement: impl Fn(&Captures) -> Result<String, E>,
) -> Result<String, E> {
    let mut new = String::with_capacity(haystack.len());
    let mut last_match = 0;
    for caps in re.captures_iter(haystack) {
        if let Some(m) = caps.get(0) {
            new.push_str(&haystack[last_match..m.start()]);
            new.push_str(&replacement(&caps)?);
            last_match = m.end();
        }
    }
    new.push_str(&haystack[last_match..]);
    Ok(new)
}

enum Processing_Err {
    NPPAccessErr,
    FailedToRead,
    FailedToProcess,
}

fn process_json(x_shift: f32, y_shift: f32) -> Result<(), Processing_Err> {
    use crate::def::npp::NPPM_GETCURRENTSCINTILLA;
    use crate::def::sci::{SCI_GETLENGTH, SCI_GETTEXT, SCI_SETTEXT};

    let npp = unsafe { NPP_DATA.clone().ok_or(Processing_Err::NPPAccessErr) }?;
    let mut which: c_int = -1;

    let which_ffi: *mut c_int = &mut which;
    unsafe {
        //Gets the handle fot the current active Scintilla window inside Notepad++
        SendMessageW(
            npp.nppHandle,
            NPPM_GETCURRENTSCINTILLA,
            WPARAM(0),
            LPARAM(which_ffi as isize),
        );
    }

    if which != -1 {
        let curScintilla: HWND = if which == 0 {
            npp.scintillaMainHandle
        } else {
            npp.scintillaSecondHandle
        };

        //Get the number of bytes for the text in the active window
        let bytes = unsafe { SendMessageW(curScintilla, SCI_GETLENGTH, WPARAM(0), LPARAM(0)) };

        //if the number of bytes is greater than 0 read the text and process it
        if bytes.0 > 0 {
            let bytes = bytes.0 as usize;
            let mut buffer: Vec<u8> = Vec::with_capacity(bytes + 1);

            //Get the text for the active window and copy it into the buffer with the capactiy pre-allocated
            let _my_res = unsafe {
                buffer.set_len(bytes + 1);
                SendMessageW(
                    curScintilla,
                    SCI_GETTEXT,
                    WPARAM(bytes),
                    LPARAM(buffer.as_mut_ptr() as isize),
                )
            };

            //Convert the raw buffer into a a string
            let buffer =
                CString::from_vec_with_nul(buffer).map_err(|_e| Processing_Err::FailedToRead)?;
            let document = buffer
                .into_string()
                .map_err(|_e| Processing_Err::FailedToRead)?;

            let x_coord_re =
                Regex::new(r#""x": (\d*[\.\d]+)"#).map_err(|_e| Processing_Err::FailedToProcess)?;
            let y_coord_re =
                Regex::new(r#""y": (\d*[\.\d]+)"#).map_err(|_e| Processing_Err::FailedToProcess)?;

            let output = replace_all(&x_coord_re, &document, |caps: &Captures| {
                if let Ok(my_float) = caps[1].parse::<f32>() {
                    Ok(format!("\"x\": {}", my_float + x_shift))
                } else {
                    Err(Processing_Err::FailedToProcess)
                }
            })?;

            let output = replace_all(&y_coord_re, &output, |caps: &Captures| {
                if let Ok(my_float) = caps[1].parse::<f32>() {
                    Ok(format!("\"y\": {}", my_float + y_shift))
                } else {
                    Err(Processing_Err::FailedToProcess)
                }
            })?;

            //convert the modified text back to a C-Style string with null terminator for sending to Notepad++
            let output =
                CString::new(output.as_str()).map_err(|_e| Processing_Err::FailedToProcess)?;
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
    Ok(())
}

pub extern "C" fn move_objects() {
    if let Ok(dlg) = Dialog::new() {
        let close_handle = dlg.as_weak();
        dlg.on_close(move || {
            //not really sure what good options I have if this fails...I guess just do nothing
            //this will leave the dialog box up though...
            if let Some(close_handle) = close_handle.upgrade() {
                if let Err(e) = process_json(close_handle.get_x_shift(), close_handle.get_y_shift())
                {
                    let err_str = match e {
                        Processing_Err::NPPAccessErr => "Unable to access Notepad++ Instance",
                        Processing_Err::FailedToRead => "Unable to read text buffer from Notepad++",
                        Processing_Err::FailedToProcess => {
                            "Unable to process document. No changes have been made."
                        }
                    };
                    unsafe {
                        MessageBoxW(
                            HWND(0),
                            PCWSTR(to_wide_chars(err_str).as_ptr()),
                            PCWSTR(to_wide_chars("Ignition Tools").as_ptr()),
                            MB_OK,
                        );
                    }
                }

                //Try to hide / close the window, not sure what else I can do if this fails
                //then drop the handle. Since it's been upgraded, will this close the window?
                //or will the main dialog reference still keep it open?
                if close_handle.hide().is_err() {
                    drop(close_handle);
                }
            }
        });

        if dlg.run().is_err() {
            unsafe {
                MessageBoxW(
                    HWND(0),
                    PCWSTR(to_wide_chars("Error launching plugin. Exiting..").as_ptr()),
                    PCWSTR(to_wide_chars("Ignition Tools").as_ptr()),
                    MB_OK,
                );
            }
        }
    } else {
        unsafe {
            MessageBoxW(
                HWND(0),
                PCWSTR(to_wide_chars("Error launching plugin. Exiting..").as_ptr()),
                PCWSTR(to_wide_chars("Ignition Tools").as_ptr()),
                MB_OK,
            );
        }
    }
}
