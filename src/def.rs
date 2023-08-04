use std::ffi::{c_int, c_uchar, c_uint};
use windows::Win32::{Foundation::HWND, UI::WindowsAndMessaging::WM_USER};

pub type Tchar = u16;

#[repr(C)]
#[derive(Clone)]
pub struct NppData {
    pub nppHandle: HWND,
    pub scintillaMainHandle: HWND,
    pub scintillaSecondHandle: HWND,
}

#[repr(C)]
pub struct ShortcutKey {
    pub isCtrl: bool,
    pub isAlt: bool,
    pub isShift: bool,
    pub key: c_uchar,
}

#[repr(C)]
pub struct FuncItem {
    pub itemName: [Tchar; 64],
    pub pFunc: extern "C" fn(),
    pub cmdID: c_int,
    pub init2Check: bool,

    /*This was taken from the rustnpp project (which is many years old and doesn't compile)
     **per the plugin template project from Notepad++ this is actually a pointer to a ShortcutKey struct
     **might need to look into updating this later, but for now it's probably fine because I don't think I'm using it
     */
    pub pShKey: usize,
}

pub fn to_wide_chars(s: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    OsStr::new(s)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect::<Vec<_>>()
}

pub fn from_wide_ptr(ptr: *const u16) -> String {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    assert!(!ptr.is_null());
    let len = unsafe {
        (0..std::isize::MAX)
            .position(|i| *ptr.offset(i) == 0)
            //If no null terminating character is found return an empty string instead of pottentially reading memory I shouldn't
            .unwrap_or(0)
    };
    if len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
        OsString::from_wide(slice).to_string_lossy().into_owned()
    } else {
        String::new()
    }
}

pub fn function_item_text(s: &str) -> [Tchar; 64] {
    let mut arr: [Tchar; 64] = [0; 64];
    let vecStr = to_wide_chars(s);
    for (i, ch) in vecStr.iter().enumerate() {
        arr[i] = *ch;
    }

    arr
}

//In the below 3 mods all these pub consts are #define's in the original header
//I'm not quite sure what bindgen would do with these if I could get it working
//I'm pretty sure they would be made consts.
//right now I'm not using most of these, but the few I'm using seem to be a mix of c_uint, c_uint, isize, u64 etc
//for now I've set them ALL as c_uint, and if I get a type mismatch error when trying to use them, I will update them
#[allow(dead_code)]
pub mod Menu {
    use super::c_uint;

    pub const IDM: c_uint = 40000;

    pub const IDM_FILE: c_uint = IDM + 1000;
    // IMPORTANT: If list below is modified, you have to change the value of IDM_FILEMENU_LASTONE and IDM_FILEMENU_EXISTCMDPOSITION
    pub const IDM_FILE_NEW: c_uint = IDM_FILE + 1;
    pub const IDM_FILE_OPEN: c_uint = IDM_FILE + 2;
    pub const IDM_FILE_CLOSE: c_uint = IDM_FILE + 3;
    pub const IDM_FILE_CLOSEALL: c_uint = IDM_FILE + 4;
    pub const IDM_FILE_CLOSEALL_BUT_CURRENT: c_uint = IDM_FILE + 5;
    pub const IDM_FILE_SAVE: c_uint = IDM_FILE + 6;
    pub const IDM_FILE_SAVEALL: c_uint = IDM_FILE + 7;
    pub const IDM_FILE_SAVEAS: c_uint = IDM_FILE + 8;
    pub const IDM_FILE_CLOSEALL_TOLEFT: c_uint = IDM_FILE + 9;
    pub const IDM_FILE_PRINT: c_uint = IDM_FILE + 10;
    pub const IDM_FILE_PRINTNOW: c_uint = 1001;
    pub const IDM_FILE_EXIT: c_uint = IDM_FILE + 11;
    pub const IDM_FILE_LOADSESSION: c_uint = IDM_FILE + 12;
    pub const IDM_FILE_SAVESESSION: c_uint = IDM_FILE + 13;
    pub const IDM_FILE_RELOAD: c_uint = IDM_FILE + 14;
    pub const IDM_FILE_SAVECOPYAS: c_uint = IDM_FILE + 15;
    pub const IDM_FILE_DELETE: c_uint = IDM_FILE + 16;
    pub const IDM_FILE_RENAME: c_uint = IDM_FILE + 17;
    pub const IDM_FILE_CLOSEALL_TORIGHT: c_uint = IDM_FILE + 18;
    pub const IDM_FILE_OPEN_FOLDER: c_uint = IDM_FILE + 19;
    pub const IDM_FILE_OPEN_CMD: c_uint = IDM_FILE + 20;
    pub const IDM_FILE_RESTORELASTCLOSEDFILE: c_uint = IDM_FILE + 21;
    pub const IDM_FILE_OPENFOLDERASWORSPACE: c_uint = IDM_FILE + 22;
    pub const IDM_FILE_OPEN_DEFAULT_VIEWER: c_uint = IDM_FILE + 23;
    pub const IDM_FILE_CLOSEALL_UNCHANGED: c_uint = IDM_FILE + 24;
    pub const IDM_FILE_CONTAININGFOLDERASWORKSPACE: c_uint = IDM_FILE + 25;
    // IMPORTANT: If list above is modified, you have to change the following values:

    // To be updated if new menu item(s) is (are) added in menu "File"
    pub const IDM_FILEMENU_LASTONE: c_uint = IDM_FILE_CONTAININGFOLDERASWORKSPACE;

    // 0 based position of command "Exit" including the bars in the file menu
    // and without counting "Recent files history" items

    // 0  New
    // 1  Open...
    // 2  Open Containing Folder
    // 3  Open Folder as Workspace
    // 4  Open in Default Viewer
    // 5  Reload from Disk
    // 6  Save
    // 7  Save As...
    // 8  Save a Copy As...
    // 9  Save All
    //10  Rename...
    //11  Close
    //12  Close All
    //13  Close Multiple Documents
    //14  Move to Recycle Bin
    //15  --------
    //16  Load Session...
    //17  Save Session...
    //18  --------
    //19  Print...
    //20  Print Now
    //21  --------
    //22  Exit
    pub const IDM_FILEMENU_EXISTCMDPOSITION: c_uint = 22;

    pub const IDM_EDIT: c_uint = IDM + 2000;
    pub const IDM_EDIT_CUT: c_uint = IDM_EDIT + 1;
    pub const IDM_EDIT_COPY: c_uint = IDM_EDIT + 2;
    pub const IDM_EDIT_UNDO: c_uint = IDM_EDIT + 3;
    pub const IDM_EDIT_REDO: c_uint = IDM_EDIT + 4;
    pub const IDM_EDIT_PASTE: c_uint = IDM_EDIT + 5;
    pub const IDM_EDIT_DELETE: c_uint = IDM_EDIT + 6;
    pub const IDM_EDIT_SELECTALL: c_uint = IDM_EDIT + 7;
    pub const IDM_EDIT_INS_TAB: c_uint = IDM_EDIT + 8;
    pub const IDM_EDIT_RMV_TAB: c_uint = IDM_EDIT + 9;
    pub const IDM_EDIT_DUP_LINE: c_uint = IDM_EDIT + 10;
    pub const IDM_EDIT_TRANSPOSE_LINE: c_uint = IDM_EDIT + 11;
    pub const IDM_EDIT_SPLIT_LINES: c_uint = IDM_EDIT + 12;
    pub const IDM_EDIT_JOIN_LINES: c_uint = IDM_EDIT + 13;
    pub const IDM_EDIT_LINE_UP: c_uint = IDM_EDIT + 14;
    pub const IDM_EDIT_LINE_DOWN: c_uint = IDM_EDIT + 15;
    pub const IDM_EDIT_UPPERCASE: c_uint = IDM_EDIT + 16;
    pub const IDM_EDIT_LOWERCASE: c_uint = IDM_EDIT + 17;
    pub const IDM_MACRO_STARTRECORDINGMACRO: c_uint = IDM_EDIT + 18;
    pub const IDM_MACRO_STOPRECORDINGMACRO: c_uint = IDM_EDIT + 19;
    pub const IDM_EDIT_BEGINENDSELECT: c_uint = IDM_EDIT + 20;
    pub const IDM_MACRO_PLAYBACKRECORDEDMACRO: c_uint = IDM_EDIT + 21;
    pub const IDM_EDIT_BLOCK_COMMENT: c_uint = IDM_EDIT + 22;
    pub const IDM_EDIT_STREAM_COMMENT: c_uint = IDM_EDIT + 23;
    pub const IDM_EDIT_TRIMTRAILING: c_uint = IDM_EDIT + 24;
    pub const IDM_MACRO_SAVECURRENTMACRO: c_uint = IDM_EDIT + 25;
    pub const IDM_EDIT_RTL: c_uint = IDM_EDIT + 26;
    pub const IDM_EDIT_LTR: c_uint = IDM_EDIT + 27;
    pub const IDM_EDIT_SETREADONLY: c_uint = IDM_EDIT + 28;
    pub const IDM_EDIT_FULLPATHTOCLIP: c_uint = IDM_EDIT + 29;
    pub const IDM_EDIT_FILENAMETOCLIP: c_uint = IDM_EDIT + 30;
    pub const IDM_EDIT_CURRENTDIRTOCLIP: c_uint = IDM_EDIT + 31;
    pub const IDM_MACRO_RUNMULTIMACRODLG: c_uint = IDM_EDIT + 32;
    pub const IDM_EDIT_CLEARREADONLY: c_uint = IDM_EDIT + 33;
    pub const IDM_EDIT_COLUMNMODE: c_uint = IDM_EDIT + 34;
    pub const IDM_EDIT_BLOCK_COMMENT_SET: c_uint = IDM_EDIT + 35;
    pub const IDM_EDIT_BLOCK_UNCOMMENT: c_uint = IDM_EDIT + 36;
    pub const IDM_EDIT_COLUMNMODETIP: c_uint = IDM_EDIT + 37;
    pub const IDM_EDIT_PASTE_AS_HTML: c_uint = IDM_EDIT + 38;
    pub const IDM_EDIT_PASTE_AS_RTF: c_uint = IDM_EDIT + 39;
    pub const IDM_OPEN_ALL_RECENT_FILE: c_uint = IDM_EDIT + 40;
    pub const IDM_CLEAN_RECENT_FILE_LIST: c_uint = IDM_EDIT + 41;
    pub const IDM_EDIT_TRIMLINEHEAD: c_uint = IDM_EDIT + 42;
    pub const IDM_EDIT_TRIM_BOTH: c_uint = IDM_EDIT + 43;
    pub const IDM_EDIT_EOL2WS: c_uint = IDM_EDIT + 44;
    pub const IDM_EDIT_TRIMALL: c_uint = IDM_EDIT + 45;
    pub const IDM_EDIT_TAB2SW: c_uint = IDM_EDIT + 46;
    pub const IDM_EDIT_STREAM_UNCOMMENT: c_uint = IDM_EDIT + 47;
    pub const IDM_EDIT_COPY_BINARY: c_uint = IDM_EDIT + 48;
    pub const IDM_EDIT_CUT_BINARY: c_uint = IDM_EDIT + 49;
    pub const IDM_EDIT_PASTE_BINARY: c_uint = IDM_EDIT + 50;
    pub const IDM_EDIT_CHAR_PANEL: c_uint = IDM_EDIT + 51;
    pub const IDM_EDIT_CLIPBOARDHISTORY_PANEL: c_uint = IDM_EDIT + 52;
    pub const IDM_EDIT_SW2TAB_LEADING: c_uint = IDM_EDIT + 53;
    pub const IDM_EDIT_SW2TAB_ALL: c_uint = IDM_EDIT + 54;
    pub const IDM_EDIT_REMOVEEMPTYLINES: c_uint = IDM_EDIT + 55;
    pub const IDM_EDIT_REMOVEEMPTYLINESWITHBLANK: c_uint = IDM_EDIT + 56;
    pub const IDM_EDIT_BLANKLINEABOVECURRENT: c_uint = IDM_EDIT + 57;
    pub const IDM_EDIT_BLANKLINEBELOWCURRENT: c_uint = IDM_EDIT + 58;
    pub const IDM_EDIT_SORTLINES_LEXICOGRAPHIC_ASCENDING: c_uint = IDM_EDIT + 59;
    pub const IDM_EDIT_SORTLINES_LEXICOGRAPHIC_DESCENDING: c_uint = IDM_EDIT + 60;
    pub const IDM_EDIT_SORTLINES_INTEGER_ASCENDING: c_uint = IDM_EDIT + 61;
    pub const IDM_EDIT_SORTLINES_INTEGER_DESCENDING: c_uint = IDM_EDIT + 62;
    pub const IDM_EDIT_SORTLINES_DECIMALCOMMA_ASCENDING: c_uint = IDM_EDIT + 63;
    pub const IDM_EDIT_SORTLINES_DECIMALCOMMA_DESCENDING: c_uint = IDM_EDIT + 64;
    pub const IDM_EDIT_SORTLINES_DECIMALDOT_ASCENDING: c_uint = IDM_EDIT + 65;
    pub const IDM_EDIT_SORTLINES_DECIMALDOT_DESCENDING: c_uint = IDM_EDIT + 66;
    pub const IDM_EDIT_PROPERCASE_FORCE: c_uint = IDM_EDIT + 67;
    pub const IDM_EDIT_PROPERCASE_BLEND: c_uint = IDM_EDIT + 68;
    pub const IDM_EDIT_SENTENCECASE_FORCE: c_uint = IDM_EDIT + 69;
    pub const IDM_EDIT_SENTENCECASE_BLEND: c_uint = IDM_EDIT + 70;
    pub const IDM_EDIT_INVERTCASE: c_uint = IDM_EDIT + 71;
    pub const IDM_EDIT_RANDOMCASE: c_uint = IDM_EDIT + 72;
    pub const IDM_EDIT_OPENASFILE: c_uint = IDM_EDIT + 73;
    pub const IDM_EDIT_OPENINFOLDER: c_uint = IDM_EDIT + 74;
    pub const IDM_EDIT_SEARCHONINTERNET: c_uint = IDM_EDIT + 75;
    pub const IDM_EDIT_CHANGESEARCHENGINE: c_uint = IDM_EDIT + 76;
    pub const IDM_EDIT_REMOVE_CONSECUTIVE_DUP_LINES: c_uint = IDM_EDIT + 77;
    pub const IDM_EDIT_SORTLINES_RANDOMLY: c_uint = IDM_EDIT + 78;
    pub const IDM_EDIT_REMOVE_ANY_DUP_LINES: c_uint = IDM_EDIT + 79;
    pub const IDM_EDIT_SORTLINES_LEXICO_CASE_INSENS_ASCENDING: c_uint = IDM_EDIT + 80;
    pub const IDM_EDIT_SORTLINES_LEXICO_CASE_INSENS_DESCENDING: c_uint = IDM_EDIT + 81;
    pub const IDM_EDIT_COPY_LINK: c_uint = IDM_EDIT + 82;
    pub const IDM_EDIT_SORTLINES_REVERSE_ORDER: c_uint = IDM_EDIT + 83;
    pub const IDM_EDIT_INSERT_DATETIME_SHORT: c_uint = IDM_EDIT + 84;
    pub const IDM_EDIT_INSERT_DATETIME_LONG: c_uint = IDM_EDIT + 85;
    pub const IDM_EDIT_INSERT_DATETIME_CUSTOMIZED: c_uint = IDM_EDIT + 86;
    pub const IDM_EDIT_COPY_ALL_NAMES: c_uint = IDM_EDIT + 87;
    pub const IDM_EDIT_COPY_ALL_PATHS: c_uint = IDM_EDIT + 88;

    pub const IDM_EDIT_AUTOCOMPLETE: c_uint = 50000;
    pub const IDM_EDIT_AUTOCOMPLETE_CURRENTFILE: c_uint = 50000 + 1;
    pub const IDM_EDIT_FUNCCALLTIP: c_uint = 50000 + 2;
    pub const IDM_EDIT_AUTOCOMPLETE_PATH: c_uint = 50000 + 6;
    pub const IDM_EDIT_FUNCCALLTIP_PREVIOUS: c_uint = 50000 + 10;
    pub const IDM_EDIT_FUNCCALLTIP_NEXT: c_uint = 50000 + 11;

    pub const IDM_SEARCH: c_uint = IDM + 3000;
    pub const IDM_SEARCH_FIND: c_uint = IDM_SEARCH + 1;
    pub const IDM_SEARCH_FINDNEXT: c_uint = IDM_SEARCH + 2;
    pub const IDM_SEARCH_REPLACE: c_uint = IDM_SEARCH + 3;
    pub const IDM_SEARCH_GOTOLINE: c_uint = IDM_SEARCH + 4;
    pub const IDM_SEARCH_TOGGLE_BOOKMARK: c_uint = IDM_SEARCH + 5;
    pub const IDM_SEARCH_NEXT_BOOKMARK: c_uint = IDM_SEARCH + 6;
    pub const IDM_SEARCH_PREV_BOOKMARK: c_uint = IDM_SEARCH + 7;
    pub const IDM_SEARCH_CLEAR_BOOKMARKS: c_uint = IDM_SEARCH + 8;
    pub const IDM_SEARCH_GOTOMATCHINGBRACE: c_uint = IDM_SEARCH + 9;
    pub const IDM_SEARCH_FINDPREV: c_uint = IDM_SEARCH + 10;
    pub const IDM_SEARCH_FINDINCREMENT: c_uint = IDM_SEARCH + 11;
    pub const IDM_SEARCH_FINDINFILES: c_uint = IDM_SEARCH + 13;
    pub const IDM_SEARCH_VOLATILE_FINDNEXT: c_uint = IDM_SEARCH + 14;
    pub const IDM_SEARCH_VOLATILE_FINDPREV: c_uint = IDM_SEARCH + 15;
    pub const IDM_SEARCH_CUTMARKEDLINES: c_uint = IDM_SEARCH + 18;
    pub const IDM_SEARCH_COPYMARKEDLINES: c_uint = IDM_SEARCH + 19;
    pub const IDM_SEARCH_PASTEMARKEDLINES: c_uint = IDM_SEARCH + 20;
    pub const IDM_SEARCH_DELETEMARKEDLINES: c_uint = IDM_SEARCH + 21;
    pub const IDM_SEARCH_MARKALLEXT1: c_uint = IDM_SEARCH + 22;
    pub const IDM_SEARCH_UNMARKALLEXT1: c_uint = IDM_SEARCH + 23;
    pub const IDM_SEARCH_MARKALLEXT2: c_uint = IDM_SEARCH + 24;
    pub const IDM_SEARCH_UNMARKALLEXT2: c_uint = IDM_SEARCH + 25;
    pub const IDM_SEARCH_MARKALLEXT3: c_uint = IDM_SEARCH + 26;
    pub const IDM_SEARCH_UNMARKALLEXT3: c_uint = IDM_SEARCH + 27;
    pub const IDM_SEARCH_MARKALLEXT4: c_uint = IDM_SEARCH + 28;
    pub const IDM_SEARCH_UNMARKALLEXT4: c_uint = IDM_SEARCH + 29;
    pub const IDM_SEARCH_MARKALLEXT5: c_uint = IDM_SEARCH + 30;
    pub const IDM_SEARCH_UNMARKALLEXT5: c_uint = IDM_SEARCH + 31;
    pub const IDM_SEARCH_CLEARALLMARKS: c_uint = IDM_SEARCH + 32;

    pub const IDM_SEARCH_GOPREVMARKER1: c_uint = IDM_SEARCH + 33;
    pub const IDM_SEARCH_GOPREVMARKER2: c_uint = IDM_SEARCH + 34;
    pub const IDM_SEARCH_GOPREVMARKER3: c_uint = IDM_SEARCH + 35;
    pub const IDM_SEARCH_GOPREVMARKER4: c_uint = IDM_SEARCH + 36;
    pub const IDM_SEARCH_GOPREVMARKER5: c_uint = IDM_SEARCH + 37;
    pub const IDM_SEARCH_GOPREVMARKER_DEF: c_uint = IDM_SEARCH + 38;

    pub const IDM_SEARCH_GONEXTMARKER1: c_uint = IDM_SEARCH + 39;
    pub const IDM_SEARCH_GONEXTMARKER2: c_uint = IDM_SEARCH + 40;
    pub const IDM_SEARCH_GONEXTMARKER3: c_uint = IDM_SEARCH + 41;
    pub const IDM_SEARCH_GONEXTMARKER4: c_uint = IDM_SEARCH + 42;
    pub const IDM_SEARCH_GONEXTMARKER5: c_uint = IDM_SEARCH + 43;
    pub const IDM_SEARCH_GONEXTMARKER_DEF: c_uint = IDM_SEARCH + 44;

    pub const IDM_FOCUS_ON_FOUND_RESULTS: c_uint = IDM_SEARCH + 45;
    pub const IDM_SEARCH_GOTONEXTFOUND: c_uint = IDM_SEARCH + 46;
    pub const IDM_SEARCH_GOTOPREVFOUND: c_uint = IDM_SEARCH + 47;

    pub const IDM_SEARCH_SETANDFINDNEXT: c_uint = IDM_SEARCH + 48;
    pub const IDM_SEARCH_SETANDFINDPREV: c_uint = IDM_SEARCH + 49;
    pub const IDM_SEARCH_INVERSEMARKS: c_uint = IDM_SEARCH + 50;
    pub const IDM_SEARCH_DELETEUNMARKEDLINES: c_uint = IDM_SEARCH + 51;
    pub const IDM_SEARCH_FINDCHARINRANGE: c_uint = IDM_SEARCH + 52;
    pub const IDM_SEARCH_SELECTMATCHINGBRACES: c_uint = IDM_SEARCH + 53;
    pub const IDM_SEARCH_MARK: c_uint = IDM_SEARCH + 54;

    pub const IDM_SEARCH_STYLE1TOCLIP: c_uint = IDM_SEARCH + 55;
    pub const IDM_SEARCH_STYLE2TOCLIP: c_uint = IDM_SEARCH + 56;
    pub const IDM_SEARCH_STYLE3TOCLIP: c_uint = IDM_SEARCH + 57;
    pub const IDM_SEARCH_STYLE4TOCLIP: c_uint = IDM_SEARCH + 58;
    pub const IDM_SEARCH_STYLE5TOCLIP: c_uint = IDM_SEARCH + 59;
    pub const IDM_SEARCH_ALLSTYLESTOCLIP: c_uint = IDM_SEARCH + 60;
    pub const IDM_SEARCH_MARKEDTOCLIP: c_uint = IDM_SEARCH + 61;

    pub const IDM_SEARCH_MARKONEEXT1: c_uint = IDM_SEARCH + 62;
    pub const IDM_SEARCH_MARKONEEXT2: c_uint = IDM_SEARCH + 63;
    pub const IDM_SEARCH_MARKONEEXT3: c_uint = IDM_SEARCH + 64;
    pub const IDM_SEARCH_MARKONEEXT4: c_uint = IDM_SEARCH + 65;
    pub const IDM_SEARCH_MARKONEEXT5: c_uint = IDM_SEARCH + 66;

    pub const IDM_MISC: c_uint = IDM + 3500;
    pub const IDM_DOCLIST_FILESCLOSE: c_uint = IDM_MISC + 1;
    pub const IDM_DOCLIST_FILESCLOSEOTHERS: c_uint = IDM_MISC + 2;
    pub const IDM_DOCLIST_COPYNAMES: c_uint = IDM_MISC + 3;
    pub const IDM_DOCLIST_COPYPATHS: c_uint = IDM_MISC + 4;

    pub const IDM_VIEW: c_uint = IDM + 4000;
    //pub const IDM_VIEW_TOOLBAR_HIDE            : c_uint = (IDM_VIEW + 1);
    pub const IDM_VIEW_TOOLBAR_REDUCE: c_uint = IDM_VIEW + 2;
    pub const IDM_VIEW_TOOLBAR_ENLARGE: c_uint = IDM_VIEW + 3;
    pub const IDM_VIEW_TOOLBAR_STANDARD: c_uint = IDM_VIEW + 4;
    pub const IDM_VIEW_REDUCETABBAR: c_uint = IDM_VIEW + 5;
    pub const IDM_VIEW_LOCKTABBAR: c_uint = IDM_VIEW + 6;
    pub const IDM_VIEW_DRAWTABBAR_TOPBAR: c_uint = IDM_VIEW + 7;
    pub const IDM_VIEW_DRAWTABBAR_INACIVETAB: c_uint = IDM_VIEW + 8;
    pub const IDM_VIEW_POSTIT: c_uint = IDM_VIEW + 9;
    pub const IDM_VIEW_FOLDALL: c_uint = IDM_VIEW + 10;
    pub const IDM_VIEW_DISTRACTIONFREE: c_uint = IDM_VIEW + 11;
    pub const IDM_VIEW_LINENUMBER: c_uint = IDM_VIEW + 12;
    pub const IDM_VIEW_SYMBOLMARGIN: c_uint = IDM_VIEW + 13;
    pub const IDM_VIEW_FOLDERMAGIN: c_uint = IDM_VIEW + 14;
    pub const IDM_VIEW_FOLDERMAGIN_SIMPLE: c_uint = IDM_VIEW + 15;
    pub const IDM_VIEW_FOLDERMAGIN_ARROW: c_uint = IDM_VIEW + 16;
    pub const IDM_VIEW_FOLDERMAGIN_CIRCLE: c_uint = IDM_VIEW + 17;
    pub const IDM_VIEW_FOLDERMAGIN_BOX: c_uint = IDM_VIEW + 18;
    pub const IDM_VIEW_ALL_CHARACTERS: c_uint = IDM_VIEW + 19;
    pub const IDM_VIEW_INDENT_GUIDE: c_uint = IDM_VIEW + 20;
    pub const IDM_VIEW_CURLINE_HILITING: c_uint = IDM_VIEW + 21;
    pub const IDM_VIEW_WRAP: c_uint = IDM_VIEW + 22;
    pub const IDM_VIEW_ZOOMIN: c_uint = IDM_VIEW + 23;
    pub const IDM_VIEW_ZOOMOUT: c_uint = IDM_VIEW + 24;
    pub const IDM_VIEW_TAB_SPACE: c_uint = IDM_VIEW + 25;
    pub const IDM_VIEW_EOL: c_uint = IDM_VIEW + 26;
    pub const IDM_VIEW_TOOLBAR_REDUCE_SET2: c_uint = IDM_VIEW + 27;
    pub const IDM_VIEW_TOOLBAR_ENLARGE_SET2: c_uint = IDM_VIEW + 28;
    pub const IDM_VIEW_UNFOLDALL: c_uint = IDM_VIEW + 29;
    pub const IDM_VIEW_FOLD_CURRENT: c_uint = IDM_VIEW + 30;
    pub const IDM_VIEW_UNFOLD_CURRENT: c_uint = IDM_VIEW + 31;
    pub const IDM_VIEW_FULLSCREENTOGGLE: c_uint = IDM_VIEW + 32;
    pub const IDM_VIEW_ZOOMRESTORE: c_uint = IDM_VIEW + 33;
    pub const IDM_VIEW_ALWAYSONTOP: c_uint = IDM_VIEW + 34;
    pub const IDM_VIEW_SYNSCROLLV: c_uint = IDM_VIEW + 35;
    pub const IDM_VIEW_SYNSCROLLH: c_uint = IDM_VIEW + 36;
    //pub const IDM_VIEW_EDGENONE                  : c_uint = (IDM_VIEW + 37);
    pub const IDM_VIEW_DRAWTABBAR_CLOSEBOTTUN: c_uint = IDM_VIEW + 38;
    pub const IDM_VIEW_DRAWTABBAR_DBCLK2CLOSE: c_uint = IDM_VIEW + 39;
    pub const IDM_VIEW_REFRESHTABAR: c_uint = IDM_VIEW + 40;
    pub const IDM_VIEW_WRAP_SYMBOL: c_uint = IDM_VIEW + 41;
    pub const IDM_VIEW_HIDELINES: c_uint = IDM_VIEW + 42;
    pub const IDM_VIEW_DRAWTABBAR_VERTICAL: c_uint = IDM_VIEW + 43;
    pub const IDM_VIEW_DRAWTABBAR_MULTILINE: c_uint = IDM_VIEW + 44;
    //pub const IDM_VIEW_DOCCHANGEMARGIN           : c_uint = (IDM_VIEW + 45);
    pub const IDM_VIEW_LWDEF: c_uint = IDM_VIEW + 46;
    pub const IDM_VIEW_LWALIGN: c_uint = IDM_VIEW + 47;
    pub const IDM_VIEW_LWINDENT: c_uint = IDM_VIEW + 48;
    pub const IDM_VIEW_SUMMARY: c_uint = IDM_VIEW + 49;

    pub const IDM_VIEW_FOLD: c_uint = IDM_VIEW + 50;
    pub const IDM_VIEW_FOLD_1: c_uint = IDM_VIEW_FOLD + 1;
    pub const IDM_VIEW_FOLD_2: c_uint = IDM_VIEW_FOLD + 2;
    pub const IDM_VIEW_FOLD_3: c_uint = IDM_VIEW_FOLD + 3;
    pub const IDM_VIEW_FOLD_4: c_uint = IDM_VIEW_FOLD + 4;
    pub const IDM_VIEW_FOLD_5: c_uint = IDM_VIEW_FOLD + 5;
    pub const IDM_VIEW_FOLD_6: c_uint = IDM_VIEW_FOLD + 6;
    pub const IDM_VIEW_FOLD_7: c_uint = IDM_VIEW_FOLD + 7;
    pub const IDM_VIEW_FOLD_8: c_uint = IDM_VIEW_FOLD + 8;

    pub const IDM_VIEW_UNFOLD: c_uint = IDM_VIEW + 60;
    pub const IDM_VIEW_UNFOLD_1: c_uint = IDM_VIEW_UNFOLD + 1;
    pub const IDM_VIEW_UNFOLD_2: c_uint = IDM_VIEW_UNFOLD + 2;
    pub const IDM_VIEW_UNFOLD_3: c_uint = IDM_VIEW_UNFOLD + 3;
    pub const IDM_VIEW_UNFOLD_4: c_uint = IDM_VIEW_UNFOLD + 4;
    pub const IDM_VIEW_UNFOLD_5: c_uint = IDM_VIEW_UNFOLD + 5;
    pub const IDM_VIEW_UNFOLD_6: c_uint = IDM_VIEW_UNFOLD + 6;
    pub const IDM_VIEW_UNFOLD_7: c_uint = IDM_VIEW_UNFOLD + 7;
    pub const IDM_VIEW_UNFOLD_8: c_uint = IDM_VIEW_UNFOLD + 8;

    pub const IDM_VIEW_DOCLIST: c_uint = IDM_VIEW + 70;
    pub const IDM_VIEW_SWITCHTO_OTHER_VIEW: c_uint = IDM_VIEW + 72;
    pub const IDM_EXPORT_FUNC_LIST_AND_QUIT: c_uint = IDM_VIEW + 73;

    pub const IDM_VIEW_DOC_MAP: c_uint = IDM_VIEW + 80;

    pub const IDM_VIEW_PROJECT_PANEL_1: c_uint = IDM_VIEW + 81;
    pub const IDM_VIEW_PROJECT_PANEL_2: c_uint = IDM_VIEW + 82;
    pub const IDM_VIEW_PROJECT_PANEL_3: c_uint = IDM_VIEW + 83;

    pub const IDM_VIEW_FUNC_LIST: c_uint = IDM_VIEW + 84;
    pub const IDM_VIEW_FILEBROWSER: c_uint = IDM_VIEW + 85;

    pub const IDM_VIEW_TAB1: c_uint = IDM_VIEW + 86;
    pub const IDM_VIEW_TAB2: c_uint = IDM_VIEW + 87;
    pub const IDM_VIEW_TAB3: c_uint = IDM_VIEW + 88;
    pub const IDM_VIEW_TAB4: c_uint = IDM_VIEW + 89;
    pub const IDM_VIEW_TAB5: c_uint = IDM_VIEW + 90;
    pub const IDM_VIEW_TAB6: c_uint = IDM_VIEW + 91;
    pub const IDM_VIEW_TAB7: c_uint = IDM_VIEW + 92;
    pub const IDM_VIEW_TAB8: c_uint = IDM_VIEW + 93;
    pub const IDM_VIEW_TAB9: c_uint = IDM_VIEW + 94;
    pub const IDM_VIEW_TAB_NEXT: c_uint = IDM_VIEW + 95;
    pub const IDM_VIEW_TAB_PREV: c_uint = IDM_VIEW + 96;
    pub const IDM_VIEW_MONITORING: c_uint = IDM_VIEW + 97;
    pub const IDM_VIEW_TAB_MOVEFORWARD: c_uint = IDM_VIEW + 98;
    pub const IDM_VIEW_TAB_MOVEBACKWARD: c_uint = IDM_VIEW + 99;
    pub const IDM_VIEW_IN_FIREFOX: c_uint = IDM_VIEW + 100;
    pub const IDM_VIEW_IN_CHROME: c_uint = IDM_VIEW + 101;
    pub const IDM_VIEW_IN_EDGE: c_uint = IDM_VIEW + 102;
    pub const IDM_VIEW_IN_IE: c_uint = IDM_VIEW + 103;

    pub const IDM_VIEW_SWITCHTO_PROJECT_PANEL_1: c_uint = IDM_VIEW + 104;
    pub const IDM_VIEW_SWITCHTO_PROJECT_PANEL_2: c_uint = IDM_VIEW + 105;
    pub const IDM_VIEW_SWITCHTO_PROJECT_PANEL_3: c_uint = IDM_VIEW + 106;
    pub const IDM_VIEW_SWITCHTO_FILEBROWSER: c_uint = IDM_VIEW + 107;
    pub const IDM_VIEW_SWITCHTO_FUNC_LIST: c_uint = IDM_VIEW + 108;
    pub const IDM_VIEW_SWITCHTO_DOCLIST: c_uint = IDM_VIEW + 109;

    pub const IDM_VIEW_TAB_COLOUR_NONE: c_uint = IDM_VIEW + 110;
    pub const IDM_VIEW_TAB_COLOUR_1: c_uint = IDM_VIEW + 111;
    pub const IDM_VIEW_TAB_COLOUR_2: c_uint = IDM_VIEW + 112;
    pub const IDM_VIEW_TAB_COLOUR_3: c_uint = IDM_VIEW + 113;
    pub const IDM_VIEW_TAB_COLOUR_4: c_uint = IDM_VIEW + 114;
    pub const IDM_VIEW_TAB_COLOUR_5: c_uint = IDM_VIEW + 115;

    pub const IDM_VIEW_GOTO_ANOTHER_VIEW: c_uint = 10001;
    pub const IDM_VIEW_CLONE_TO_ANOTHER_VIEW: c_uint = 10002;
    pub const IDM_VIEW_GOTO_NEW_INSTANCE: c_uint = 10003;
    pub const IDM_VIEW_LOAD_IN_NEW_INSTANCE: c_uint = 10004;

    pub const IDM_FORMAT: c_uint = IDM + 5000;
    pub const IDM_FORMAT_TODOS: c_uint = IDM_FORMAT + 1;
    pub const IDM_FORMAT_TOUNIX: c_uint = IDM_FORMAT + 2;
    pub const IDM_FORMAT_TOMAC: c_uint = IDM_FORMAT + 3;
    pub const IDM_FORMAT_ANSI: c_uint = IDM_FORMAT + 4;
    pub const IDM_FORMAT_UTF_8: c_uint = IDM_FORMAT + 5;
    pub const IDM_FORMAT_UTF_16BE: c_uint = IDM_FORMAT + 6;
    pub const IDM_FORMAT_UTF_16LE: c_uint = IDM_FORMAT + 7;
    pub const IDM_FORMAT_AS_UTF_8: c_uint = IDM_FORMAT + 8;
    pub const IDM_FORMAT_CONV2_ANSI: c_uint = IDM_FORMAT + 9;
    pub const IDM_FORMAT_CONV2_AS_UTF_8: c_uint = IDM_FORMAT + 10;
    pub const IDM_FORMAT_CONV2_UTF_8: c_uint = IDM_FORMAT + 11;
    pub const IDM_FORMAT_CONV2_UTF_16BE: c_uint = IDM_FORMAT + 12;
    pub const IDM_FORMAT_CONV2_UTF_16LE: c_uint = IDM_FORMAT + 13;

    pub const IDM_FORMAT_ENCODE: c_uint = IDM_FORMAT + 20;
    pub const IDM_FORMAT_WIN_1250: c_uint = IDM_FORMAT_ENCODE;
    pub const IDM_FORMAT_WIN_1251: c_uint = IDM_FORMAT_ENCODE + 1;
    pub const IDM_FORMAT_WIN_1252: c_uint = IDM_FORMAT_ENCODE + 2;
    pub const IDM_FORMAT_WIN_1253: c_uint = IDM_FORMAT_ENCODE + 3;
    pub const IDM_FORMAT_WIN_1254: c_uint = IDM_FORMAT_ENCODE + 4;
    pub const IDM_FORMAT_WIN_1255: c_uint = IDM_FORMAT_ENCODE + 5;
    pub const IDM_FORMAT_WIN_1256: c_uint = IDM_FORMAT_ENCODE + 6;
    pub const IDM_FORMAT_WIN_1257: c_uint = IDM_FORMAT_ENCODE + 7;
    pub const IDM_FORMAT_WIN_1258: c_uint = IDM_FORMAT_ENCODE + 8;
    pub const IDM_FORMAT_ISO_8859_1: c_uint = IDM_FORMAT_ENCODE + 9;
    pub const IDM_FORMAT_ISO_8859_2: c_uint = IDM_FORMAT_ENCODE + 10;
    pub const IDM_FORMAT_ISO_8859_3: c_uint = IDM_FORMAT_ENCODE + 11;
    pub const IDM_FORMAT_ISO_8859_4: c_uint = IDM_FORMAT_ENCODE + 12;
    pub const IDM_FORMAT_ISO_8859_5: c_uint = IDM_FORMAT_ENCODE + 13;
    pub const IDM_FORMAT_ISO_8859_6: c_uint = IDM_FORMAT_ENCODE + 14;
    pub const IDM_FORMAT_ISO_8859_7: c_uint = IDM_FORMAT_ENCODE + 15;
    pub const IDM_FORMAT_ISO_8859_8: c_uint = IDM_FORMAT_ENCODE + 16;
    pub const IDM_FORMAT_ISO_8859_9: c_uint = IDM_FORMAT_ENCODE + 17;
    //pub const IDM_FORMAT_ISO_8859_10       : c_uint = (IDM_FORMAT_ENCODE + 18);
    //pub const IDM_FORMAT_ISO_8859_11       : c_uint = (IDM_FORMAT_ENCODE + 19);
    pub const IDM_FORMAT_ISO_8859_13: c_uint = IDM_FORMAT_ENCODE + 20;
    pub const IDM_FORMAT_ISO_8859_14: c_uint = IDM_FORMAT_ENCODE + 21;
    pub const IDM_FORMAT_ISO_8859_15: c_uint = IDM_FORMAT_ENCODE + 22;
    //pub const IDM_FORMAT_ISO_8859_16       : c_uint = (IDM_FORMAT_ENCODE + 23);
    pub const IDM_FORMAT_DOS_437: c_uint = IDM_FORMAT_ENCODE + 24;
    pub const IDM_FORMAT_DOS_720: c_uint = IDM_FORMAT_ENCODE + 25;
    pub const IDM_FORMAT_DOS_737: c_uint = IDM_FORMAT_ENCODE + 26;
    pub const IDM_FORMAT_DOS_775: c_uint = IDM_FORMAT_ENCODE + 27;
    pub const IDM_FORMAT_DOS_850: c_uint = IDM_FORMAT_ENCODE + 28;
    pub const IDM_FORMAT_DOS_852: c_uint = IDM_FORMAT_ENCODE + 29;
    pub const IDM_FORMAT_DOS_855: c_uint = IDM_FORMAT_ENCODE + 30;
    pub const IDM_FORMAT_DOS_857: c_uint = IDM_FORMAT_ENCODE + 31;
    pub const IDM_FORMAT_DOS_858: c_uint = IDM_FORMAT_ENCODE + 32;
    pub const IDM_FORMAT_DOS_860: c_uint = IDM_FORMAT_ENCODE + 33;
    pub const IDM_FORMAT_DOS_861: c_uint = IDM_FORMAT_ENCODE + 34;
    pub const IDM_FORMAT_DOS_862: c_uint = IDM_FORMAT_ENCODE + 35;
    pub const IDM_FORMAT_DOS_863: c_uint = IDM_FORMAT_ENCODE + 36;
    pub const IDM_FORMAT_DOS_865: c_uint = IDM_FORMAT_ENCODE + 37;
    pub const IDM_FORMAT_DOS_866: c_uint = IDM_FORMAT_ENCODE + 38;
    pub const IDM_FORMAT_DOS_869: c_uint = IDM_FORMAT_ENCODE + 39;
    pub const IDM_FORMAT_BIG5: c_uint = IDM_FORMAT_ENCODE + 40;
    pub const IDM_FORMAT_GB2312: c_uint = IDM_FORMAT_ENCODE + 41;
    pub const IDM_FORMAT_SHIFT_JIS: c_uint = IDM_FORMAT_ENCODE + 42;
    pub const IDM_FORMAT_KOREAN_WIN: c_uint = IDM_FORMAT_ENCODE + 43;
    pub const IDM_FORMAT_EUC_KR: c_uint = IDM_FORMAT_ENCODE + 44;
    pub const IDM_FORMAT_TIS_620: c_uint = IDM_FORMAT_ENCODE + 45;
    pub const IDM_FORMAT_MAC_CYRILLIC: c_uint = IDM_FORMAT_ENCODE + 46;
    pub const IDM_FORMAT_KOI8U_CYRILLIC: c_uint = IDM_FORMAT_ENCODE + 47;
    pub const IDM_FORMAT_KOI8R_CYRILLIC: c_uint = IDM_FORMAT_ENCODE + 48;
    pub const IDM_FORMAT_ENCODE_END: c_uint = IDM_FORMAT_KOI8R_CYRILLIC;

    //pub const IDM_FORMAT_CONVERT           200

    pub const IDM_LANG: c_uint = IDM + 6000;
    pub const IDM_LANGSTYLE_CONFIG_DLG: c_uint = IDM_LANG + 1;
    pub const IDM_LANG_C: c_uint = IDM_LANG + 2;
    pub const IDM_LANG_CPP: c_uint = IDM_LANG + 3;
    pub const IDM_LANG_JAVA: c_uint = IDM_LANG + 4;
    pub const IDM_LANG_HTML: c_uint = IDM_LANG + 5;
    pub const IDM_LANG_XML: c_uint = IDM_LANG + 6;
    pub const IDM_LANG_JS: c_uint = IDM_LANG + 7;
    pub const IDM_LANG_PHP: c_uint = IDM_LANG + 8;
    pub const IDM_LANG_ASP: c_uint = IDM_LANG + 9;
    pub const IDM_LANG_CSS: c_uint = IDM_LANG + 10;
    pub const IDM_LANG_PASCAL: c_uint = IDM_LANG + 11;
    pub const IDM_LANG_PYTHON: c_uint = IDM_LANG + 12;
    pub const IDM_LANG_PERL: c_uint = IDM_LANG + 13;
    pub const IDM_LANG_OBJC: c_uint = IDM_LANG + 14;
    pub const IDM_LANG_ASCII: c_uint = IDM_LANG + 15;
    pub const IDM_LANG_TEXT: c_uint = IDM_LANG + 16;
    pub const IDM_LANG_RC: c_uint = IDM_LANG + 17;
    pub const IDM_LANG_MAKEFILE: c_uint = IDM_LANG + 18;
    pub const IDM_LANG_INI: c_uint = IDM_LANG + 19;
    pub const IDM_LANG_SQL: c_uint = IDM_LANG + 20;
    pub const IDM_LANG_VB: c_uint = IDM_LANG + 21;
    pub const IDM_LANG_BATCH: c_uint = IDM_LANG + 22;
    pub const IDM_LANG_CS: c_uint = IDM_LANG + 23;
    pub const IDM_LANG_LUA: c_uint = IDM_LANG + 24;
    pub const IDM_LANG_TEX: c_uint = IDM_LANG + 25;
    pub const IDM_LANG_FORTRAN: c_uint = IDM_LANG + 26;
    pub const IDM_LANG_BASH: c_uint = IDM_LANG + 27;
    pub const IDM_LANG_FLASH: c_uint = IDM_LANG + 28;
    pub const IDM_LANG_NSIS: c_uint = IDM_LANG + 29;
    pub const IDM_LANG_TCL: c_uint = IDM_LANG + 30;
    pub const IDM_LANG_LISP: c_uint = IDM_LANG + 31;
    pub const IDM_LANG_SCHEME: c_uint = IDM_LANG + 32;
    pub const IDM_LANG_ASM: c_uint = IDM_LANG + 33;
    pub const IDM_LANG_DIFF: c_uint = IDM_LANG + 34;
    pub const IDM_LANG_PROPS: c_uint = IDM_LANG + 35;
    pub const IDM_LANG_PS: c_uint = IDM_LANG + 36;
    pub const IDM_LANG_RUBY: c_uint = IDM_LANG + 37;
    pub const IDM_LANG_SMALLTALK: c_uint = IDM_LANG + 38;
    pub const IDM_LANG_VHDL: c_uint = IDM_LANG + 39;
    pub const IDM_LANG_CAML: c_uint = IDM_LANG + 40;
    pub const IDM_LANG_KIX: c_uint = IDM_LANG + 41;
    pub const IDM_LANG_ADA: c_uint = IDM_LANG + 42;
    pub const IDM_LANG_VERILOG: c_uint = IDM_LANG + 43;
    pub const IDM_LANG_AU3: c_uint = IDM_LANG + 44;
    pub const IDM_LANG_MATLAB: c_uint = IDM_LANG + 45;
    pub const IDM_LANG_HASKELL: c_uint = IDM_LANG + 46;
    pub const IDM_LANG_INNO: c_uint = IDM_LANG + 47;
    pub const IDM_LANG_CMAKE: c_uint = IDM_LANG + 48;
    pub const IDM_LANG_YAML: c_uint = IDM_LANG + 49;
    pub const IDM_LANG_COBOL: c_uint = IDM_LANG + 50;
    pub const IDM_LANG_D: c_uint = IDM_LANG + 51;
    pub const IDM_LANG_GUI4CLI: c_uint = IDM_LANG + 52;
    pub const IDM_LANG_POWERSHELL: c_uint = IDM_LANG + 53;
    pub const IDM_LANG_R: c_uint = IDM_LANG + 54;
    pub const IDM_LANG_JSP: c_uint = IDM_LANG + 55;
    pub const IDM_LANG_COFFEESCRIPT: c_uint = IDM_LANG + 56;
    pub const IDM_LANG_JSON: c_uint = IDM_LANG + 57;
    pub const IDM_LANG_FORTRAN_77: c_uint = IDM_LANG + 58;
    pub const IDM_LANG_BAANC: c_uint = IDM_LANG + 59;
    pub const IDM_LANG_SREC: c_uint = IDM_LANG + 60;
    pub const IDM_LANG_IHEX: c_uint = IDM_LANG + 61;
    pub const IDM_LANG_TEHEX: c_uint = IDM_LANG + 62;
    pub const IDM_LANG_SWIFT: c_uint = IDM_LANG + 63;
    pub const IDM_LANG_ASN1: c_uint = IDM_LANG + 64;
    pub const IDM_LANG_AVS: c_uint = IDM_LANG + 65;
    pub const IDM_LANG_BLITZBASIC: c_uint = IDM_LANG + 66;
    pub const IDM_LANG_PUREBASIC: c_uint = IDM_LANG + 67;
    pub const IDM_LANG_FREEBASIC: c_uint = IDM_LANG + 68;
    pub const IDM_LANG_CSOUND: c_uint = IDM_LANG + 69;
    pub const IDM_LANG_ERLANG: c_uint = IDM_LANG + 70;
    pub const IDM_LANG_ESCRIPT: c_uint = IDM_LANG + 71;
    pub const IDM_LANG_FORTH: c_uint = IDM_LANG + 72;
    pub const IDM_LANG_LATEX: c_uint = IDM_LANG + 73;
    pub const IDM_LANG_MMIXAL: c_uint = IDM_LANG + 74;
    pub const IDM_LANG_NIM: c_uint = IDM_LANG + 75;
    pub const IDM_LANG_NNCRONTAB: c_uint = IDM_LANG + 76;
    pub const IDM_LANG_OSCRIPT: c_uint = IDM_LANG + 77;
    pub const IDM_LANG_REBOL: c_uint = IDM_LANG + 78;
    pub const IDM_LANG_REGISTRY: c_uint = IDM_LANG + 79;
    pub const IDM_LANG_RUST: c_uint = IDM_LANG + 80;
    pub const IDM_LANG_SPICE: c_uint = IDM_LANG + 81;
    pub const IDM_LANG_TXT2TAGS: c_uint = IDM_LANG + 82;
    pub const IDM_LANG_VISUALPROLOG: c_uint = IDM_LANG + 83;
    pub const IDM_LANG_TYPESCRIPT: c_uint = IDM_LANG + 84;

    pub const IDM_LANG_EXTERNAL: c_uint = IDM_LANG + 165;
    pub const IDM_LANG_EXTERNAL_LIMIT: c_uint = IDM_LANG + 179;

    pub const IDM_LANG_USER: c_uint = IDM_LANG + 180; //46180: Used for translation;
    pub const IDM_LANG_USER_LIMIT: c_uint = IDM_LANG + 210; //46210: Ajust with IDM_LANG_USER;
    pub const IDM_LANG_USER_DLG: c_uint = IDM_LANG + 250; //46250: Used for translation;
    pub const IDM_LANG_OPENUDLDIR: c_uint = IDM_LANG + 300;
    pub const IDM_LANG_UDLCOLLECTION_PROJECT_SITE: c_uint = IDM_LANG + 301;

    pub const IDM_ABOUT: c_uint = IDM + 7000;
    pub const IDM_HOMESWEETHOME: c_uint = IDM_ABOUT + 1;
    pub const IDM_PROJECTPAGE: c_uint = IDM_ABOUT + 2;
    pub const IDM_ONLINEDOCUMENT: c_uint = IDM_ABOUT + 3;
    pub const IDM_FORUM: c_uint = IDM_ABOUT + 4;
    //pub const IDM_PLUGINSHOME      : c_uint = (IDM_ABOUT  + 5);
    pub const IDM_UPDATE_NPP: c_uint = IDM_ABOUT + 6;
    pub const IDM_WIKIFAQ: c_uint = IDM_ABOUT + 7;
    //pub const IDM_HELP             : c_uint = (IDM_ABOUT  + 8);
    pub const IDM_CONFUPDATERPROXY: c_uint = IDM_ABOUT + 9;
    pub const IDM_CMDLINEARGUMENTS: c_uint = IDM_ABOUT + 10;
    //pub const IDM_ONLINESUPPORT    : c_uint = (IDM_ABOUT  + 11);
    pub const IDM_DEBUGINFO: c_uint = IDM_ABOUT + 12;

    pub const IDM_SETTING: c_uint = IDM + 8000;
    //    pub const IDM_SETTING_TAB_SIZE                 : c_uint = (IDM_SETTING + 1);
    //    pub const IDM_SETTING_TAB_REPLCESPACE          : c_uint = (IDM_SETTING + 2);
    //    pub const IDM_SETTING_HISTORY_SIZE             : c_uint = (IDM_SETTING + 3);
    //    pub const IDM_SETTING_EDGE_SIZE                : c_uint = (IDM_SETTING + 4);
    pub const IDM_SETTING_IMPORTPLUGIN: c_uint = IDM_SETTING + 5;
    pub const IDM_SETTING_IMPORTSTYLETHEMS: c_uint = IDM_SETTING + 6;
    pub const IDM_SETTING_TRAYICON: c_uint = IDM_SETTING + 8;
    pub const IDM_SETTING_SHORTCUT_MAPPER: c_uint = IDM_SETTING + 9;
    pub const IDM_SETTING_REMEMBER_LAST_SESSION: c_uint = IDM_SETTING + 10;
    pub const IDM_SETTING_PREFERENCE: c_uint = IDM_SETTING + 11;
    pub const IDM_SETTING_OPENPLUGINSDIR: c_uint = IDM_SETTING + 14;
    pub const IDM_SETTING_PLUGINADM: c_uint = IDM_SETTING + 15;
    pub const IDM_SETTING_SHORTCUT_MAPPER_MACRO: c_uint = IDM_SETTING + 16;
    pub const IDM_SETTING_SHORTCUT_MAPPER_RUN: c_uint = IDM_SETTING + 17;
    pub const IDM_SETTING_EDITCONTEXTMENU: c_uint = IDM_SETTING + 18;

    pub const IDM_TOOL: c_uint = IDM + 8500;
    pub const IDM_TOOL_MD5_GENERATE: c_uint = IDM_TOOL + 1;
    pub const IDM_TOOL_MD5_GENERATEFROMFILE: c_uint = IDM_TOOL + 2;
    pub const IDM_TOOL_MD5_GENERATEINTOCLIPBOARD: c_uint = IDM_TOOL + 3;
    pub const IDM_TOOL_SHA256_GENERATE: c_uint = IDM_TOOL + 4;
    pub const IDM_TOOL_SHA256_GENERATEFROMFILE: c_uint = IDM_TOOL + 5;
    pub const IDM_TOOL_SHA256_GENERATEINTOCLIPBOARD: c_uint = IDM_TOOL + 6;

    pub const IDM_EXECUTE: c_uint = IDM + 9000;

    pub const IDM_SYSTRAYPOPUP: c_uint = IDM + 3100;
    pub const IDM_SYSTRAYPOPUP_ACTIVATE: c_uint = IDM_SYSTRAYPOPUP + 1;
    pub const IDM_SYSTRAYPOPUP_NEWDOC: c_uint = IDM_SYSTRAYPOPUP + 2;
    pub const IDM_SYSTRAYPOPUP_NEW_AND_PASTE: c_uint = IDM_SYSTRAYPOPUP + 3;
    pub const IDM_SYSTRAYPOPUP_OPENFILE: c_uint = IDM_SYSTRAYPOPUP + 4;
    pub const IDM_SYSTRAYPOPUP_CLOSE: c_uint = IDM_SYSTRAYPOPUP + 5;

    const IDR_WINDOWS_MENU: c_uint = 11000;
    pub const IDM_WINDOW_WINDOWS: c_uint = IDR_WINDOWS_MENU + 1;
    pub const IDM_WINDOW_SORT_FN_ASC: c_uint = IDR_WINDOWS_MENU + 2;
    pub const IDM_WINDOW_SORT_FN_DSC: c_uint = IDR_WINDOWS_MENU + 3;
    pub const IDM_WINDOW_SORT_FP_ASC: c_uint = IDR_WINDOWS_MENU + 4;
    pub const IDM_WINDOW_SORT_FP_DSC: c_uint = IDR_WINDOWS_MENU + 5;
    pub const IDM_WINDOW_SORT_FT_ASC: c_uint = IDR_WINDOWS_MENU + 6;
    pub const IDM_WINDOW_SORT_FT_DSC: c_uint = IDR_WINDOWS_MENU + 7;
    pub const IDM_WINDOW_SORT_FS_ASC: c_uint = IDR_WINDOWS_MENU + 8;
    pub const IDM_WINDOW_SORT_FS_DSC: c_uint = IDR_WINDOWS_MENU + 9;
    pub const IDM_WINDOW_MRU_FIRST: c_uint = IDR_WINDOWS_MENU + 20;
    pub const IDM_WINDOW_MRU_LIMIT: c_uint = IDR_WINDOWS_MENU + 59;
    pub const IDM_WINDOW_COPY_NAME: c_uint = IDM_WINDOW_MRU_LIMIT + 1;
    pub const IDM_WINDOW_COPY_PATH: c_uint = IDM_WINDOW_MRU_LIMIT + 2;

    const IDR_DROPLIST_MENU: c_uint = 14000;
    pub const IDM_DROPLIST_LIST: c_uint = IDR_DROPLIST_MENU + 1;
    pub const IDM_DROPLIST_MRU_FIRST: c_uint = IDR_DROPLIST_MENU + 20;
}

#[allow(dead_code)]
pub mod npp {
    use super::c_uint;
    /*enum LangType {L_TEXT, L_PHP , L_C, L_CPP, L_CS, L_OBJC, L_JAVA, L_RC,\
            L_HTML, L_XML, L_MAKEFILE, L_PASCAL, L_BATCH, L_INI, L_ASCII, L_USER,\
            L_ASP, L_SQL, L_VB, L_JS, L_CSS, L_PERL, L_PYTHON, L_LUA, \
            L_TEX, L_FORTRAN, L_BASH, L_FLASH, L_NSIS, L_TCL, L_LISP, L_SCHEME,\
            L_ASM, L_DIFF, L_PROPS, L_PS, L_RUBY, L_SMALLTALK, L_VHDL, L_KIX, L_AU3,\
            L_CAML, L_ADA, L_VERILOG, L_MATLAB, L_HASKELL, L_INNO, L_SEARCHRESULT,\
            L_CMAKE, L_YAML, L_COBOL, L_GUI4CLI, L_D, L_POWERSHELL, L_R, L_JSP,\
            L_COFFEESCRIPT, L_JSON, L_JAVASCRIPT, L_FORTRAN_77, L_BAANC, L_SREC,\
            L_IHEX, L_TEHEX, L_SWIFT,\
            L_ASN1, L_AVS, L_BLITZBASIC, L_PUREBASIC, L_FREEBASIC, \
            L_CSOUND, L_ERLANG, L_ESCRIPT, L_FORTH, L_LATEX, \
            L_MMIXAL, L_NIM, L_NNCRONTAB, L_OSCRIPT, L_REBOL, \
            L_REGISTRY, L_RUST, L_SPICE, L_TXT2TAGS, L_VISUALPROLOG, L_TYPESCRIPT,\
            // Don't use L_JS, use L_JAVASCRIPT instead
            // The end of enumated language type, so it should be always at the end
            L_EXTERNAL};
    enum ExternalLexerAutoIndentMode { Standard, C_Like, Custom };
    enum MacroStatus { Idle, RecordInProgress, RecordingStopped, PlayingBack };

    enum winVer { WV_UNKNOWN, WV_WIN32S, WV_95, WV_98, WV_ME, WV_NT, WV_W2K, WV_XP, WV_S2003, WV_XPX64, WV_VISTA, WV_WIN7, WV_WIN8, WV_WIN81, WV_WIN10, WV_WIN11 };
    enum Platform { PF_UNKNOWN, PF_X86, PF_X64, PF_IA64, PF_ARM64 };*/

    pub const NPPMSG: c_uint = super::WM_USER + 1000;

    pub const NPPM_GETCURRENTSCINTILLA: c_uint = NPPMSG + 4;
    pub const NPPM_GETCURRENTLANGTYPE: c_uint = NPPMSG + 5;
    pub const NPPM_SETCURRENTLANGTYPE: c_uint = NPPMSG + 6;

    pub const NPPM_GETNBOPENFILES: c_uint = NPPMSG + 7;
    const ALL_OPEN_FILES: c_uint = 0;
    const PRIMARY_VIEW: c_uint = 1;
    const SECOND_VIEW: c_uint = 2;

    pub const NPPM_GETOPENFILENAMES: c_uint = NPPMSG + 8;

    pub const NPPM_MODELESSDIALOG: c_uint = NPPMSG + 12;
    const MODELESSDIALOGADD: c_uint = 0;
    const MODELESSDIALOGREMOVE: c_uint = 1;

    pub const NPPM_GETNBSESSIONFILES: c_uint = NPPMSG + 13;
    pub const NPPM_GETSESSIONFILES: c_uint = NPPMSG + 14;
    pub const NPPM_SAVESESSION: c_uint = NPPMSG + 15;
    pub const NPPM_SAVECURRENTSESSION: c_uint = NPPMSG + 16;

    /*struct sessionInfo {
        TCHAR* sessionFilePathName;
        int nbFile;
        TCHAR** files;
    };*/

    pub const NPPM_GETOPENFILENAMESPRIMARY: c_uint = NPPMSG + 17;
    pub const NPPM_GETOPENFILENAMESSECOND: c_uint = NPPMSG + 18;

    pub const NPPM_CREATESCINTILLAHANDLE: c_uint = NPPMSG + 20;
    pub const NPPM_DESTROYSCINTILLAHANDLE: c_uint = NPPMSG + 21;
    pub const NPPM_GETNBUSERLANG: c_uint = NPPMSG + 22;

    pub const NPPM_GETCURRENTDOCINDEX: c_uint = NPPMSG + 23;
    const MAIN_VIEW: c_uint = 0;
    const SUB_VIEW: c_uint = 1;

    pub const NPPM_SETSTATUSBAR: c_uint = NPPMSG + 24;
    pub const STATUSBAR_DOC_TYPE: c_uint = 0;
    pub const STATUSBAR_DOC_SIZE: c_uint = 1;
    pub const STATUSBAR_CUR_POS: c_uint = 2;
    pub const STATUSBAR_EOF_FORMAT: c_uint = 3;
    pub const STATUSBAR_UNICODE_TYPE: c_uint = 4;
    pub const STATUSBAR_TYPING_MODE: c_uint = 5;

    pub const NPPM_GETMENUHANDLE: c_uint = NPPMSG + 25;
    pub const NPPPLUGINMENU: c_uint = 0;
    pub const NPPMAINMENU: c_uint = 1;
    // INT NPPM_GETMENUHANDLE(INT menuChoice, 0)
    // Return: menu handle (HMENU) of choice (plugin menu handle or Notepad++ main menu handle)

    pub const NPPM_ENCODESCI: c_uint = NPPMSG + 26;
    //ascii file to unicode
    //int NPPM_ENCODESCI(MAIN_VIEW/SUB_VIEW, 0)
    //return new unicodeMode

    pub const NPPM_DECODESCI: c_uint = NPPMSG + 27;
    //unicode file to ascii
    //int NPPM_DECODESCI(MAIN_VIEW/SUB_VIEW, 0)
    //return old unicodeMode

    pub const NPPM_ACTIVATEDOC: c_uint = NPPMSG + 28;
    //void NPPM_ACTIVATEDOC(int view, int index2Activate)

    pub const NPPM_LAUNCHFINDINFILESDLG: c_uint = NPPMSG + 29;
    //void NPPM_LAUNCHFINDINFILESDLG(TCHAR * dir2Search, TCHAR * filtre)

    pub const NPPM_DMMSHOW: c_uint = NPPMSG + 30;
    //void NPPM_DMMSHOW(0, tTbData->hClient)

    pub const NPPM_DMMHIDE: c_uint = NPPMSG + 31;
    //void NPPM_DMMHIDE(0, tTbData->hClient)

    pub const NPPM_DMMUPDATEDISPINFO: c_uint = NPPMSG + 32;
    //void NPPM_DMMUPDATEDISPINFO(0, tTbData->hClient)

    pub const NPPM_DMMREGASDCKDLG: c_uint = NPPMSG + 33;
    //void NPPM_DMMREGASDCKDLG(0, &tTbData)

    pub const NPPM_LOADSESSION: c_uint = NPPMSG + 34;
    //void NPPM_LOADSESSION(0, const TCHAR* file name)

    pub const NPPM_DMMVIEWOTHERTAB: c_uint = NPPMSG + 35;
    //void WM_DMM_VIEWOTHERTAB(0, tTbData->pszName)

    pub const NPPM_RELOADFILE: c_uint = NPPMSG + 36;
    //BOOL NPPM_RELOADFILE(BOOL withAlert, TCHAR *filePathName2Reload)

    pub const NPPM_SWITCHTOFILE: c_uint = NPPMSG + 37;
    //BOOL NPPM_SWITCHTOFILE(0, TCHAR *filePathName2switch)

    pub const NPPM_SAVECURRENTFILE: c_uint = NPPMSG + 38;
    //BOOL NPPM_SAVECURRENTFILE(0, 0)

    pub const NPPM_SAVEALLFILES: c_uint = NPPMSG + 39;
    //BOOL NPPM_SAVEALLFILES(0, 0)

    pub const NPPM_SETMENUITEMCHECK: c_uint = NPPMSG + 40;
    //void WM_PIMENU_CHECK(UINT	funcItem[X]._cmdID, TRUE/FALSE)

    pub const NPPM_ADDTOOLBARICON_DEPRECATED: c_uint = NPPMSG + 41;
    //void NPPM_ADDTOOLBARICON(UINT funcItem[X]._cmdID, toolbarIcons iconHandles) -- DEPRECATED : use NPPM_ADDTOOLBARICON_FORDARKMODE instead
    //2 formats of icon are needed: .ico & .bmp
    //Both handles below should be set so the icon will be displayed correctly if toolbar icon sets are changed by users
    /*struct toolbarIcons {
        HBITMAP	hToolbarBmp;
        HICON	hToolbarIcon;
    };*/

    pub const NPPM_GETWINDOWSVERSION: c_uint = NPPMSG + 42;
    //winVer NPPM_GETWINDOWSVERSION(0, 0)

    pub const NPPM_DMMGETPLUGINHWNDBYNAME: c_uint = NPPMSG + 43;
    //HWND WM_DMM_GETPLUGINHWNDBYNAME(const TCHAR *windowName, const TCHAR *moduleName)
    // if moduleName is NULL, then return value is NULL
    // if windowName is NULL, then the first found window handle which matches with the moduleName will be returned

    pub const NPPM_MAKECURRENTBUFFERDIRTY: c_uint = NPPMSG + 44;
    //BOOL NPPM_MAKECURRENTBUFFERDIRTY(0, 0)

    pub const NPPM_GETENABLETHEMETEXTUREFUNC: c_uint = NPPMSG + 45;
    //BOOL NPPM_GETENABLETHEMETEXTUREFUNC(0, 0)

    pub const NPPM_GETPLUGINSCONFIGDIR: c_uint = NPPMSG + 46;
    //INT NPPM_GETPLUGINSCONFIGDIR(int strLen, TCHAR *str)
    // Get user's plugin config directory path. It's useful if plugins want to save/load parameters for the current user
    // Returns the number of TCHAR copied/to copy.
    // Users should call it with "str" be NULL to get the required number of TCHAR (not including the terminating nul character),
    // allocate "str" buffer with the return value + 1, then call it again to get the path.

    pub const NPPM_MSGTOPLUGIN: c_uint = NPPMSG + 47;
    //BOOL NPPM_MSGTOPLUGIN(TCHAR *destModuleName, CommunicationInfo *info)
    // return value is TRUE when the message arrive to the destination plugins.
    // if destModule or info is NULL, then return value is FALSE
    /*struct CommunicationInfo {
        long internalMsg;
        const TCHAR * srcModuleName;
        void * info; // defined by plugin
    };*/

    pub const NPPM_MENUCOMMAND: c_uint = NPPMSG + 48;
    //void NPPM_MENUCOMMAND(0, int cmdID)
    // uncomment //#include "menuCmdID.h"
    // in the beginning of this file then use the command symbols defined in "menuCmdID.h" file
    // to access all the Notepad++ menu command items

    pub const NPPM_TRIGGERTABBARCONTEXTMENU: c_uint = NPPMSG + 49;
    //void NPPM_TRIGGERTABBARCONTEXTMENU(int view, int index2Activate)

    pub const NPPM_GETNPPVERSION: c_uint = NPPMSG + 50;
    // int NPPM_GETNPPVERSION(BOOL ADD_ZERO_PADDING, 0)
    // Get Notepad++ version
    // HIWORD(returned_value) is major part of version: the 1st number
    // LOWORD(returned_value) is minor part of version: the 3 last numbers
    //
    // ADD_ZERO_PADDING == TRUE
    //
    // version  | HIWORD | LOWORD
    //------------------------------
    // 8.9.6.4  | 8      | 964
    // 9        | 9      | 0
    // 6.9      | 6      | 900
    // 6.6.6    | 6      | 660
    // 13.6.6.6 | 13     | 666
    //
    //
    // ADD_ZERO_PADDING == FALSE
    //
    // version  | HIWORD | LOWORD
    //------------------------------
    // 8.9.6.4  | 8      | 964
    // 9        | 9      | 0
    // 6.9      | 6      | 9
    // 6.6.6    | 6      | 66
    // 13.6.6.6 | 13     | 666

    pub const NPPM_HIDETABBAR: c_uint = NPPMSG + 51;
    // BOOL NPPM_HIDETABBAR(0, BOOL hideOrNot)
    // if hideOrNot is set as TRUE then tab bar will be hidden
    // otherwise it'll be shown.
    // return value : the old status value

    pub const NPPM_ISTABBARHIDDEN: c_uint = NPPMSG + 52;
    // BOOL NPPM_ISTABBARHIDDEN(0, 0)
    // returned value : TRUE if tab bar is hidden, otherwise FALSE

    pub const NPPM_GETPOSFROMBUFFERID: c_uint = NPPMSG + 57;
    // INT NPPM_GETPOSFROMBUFFERID(UINT_PTR bufferID, INT priorityView)
    // Return VIEW|INDEX from a buffer ID. -1 if the bufferID non existing
    // if priorityView set to SUB_VIEW, then SUB_VIEW will be search firstly
    //
    // VIEW takes 2 highest bits and INDEX (0 based) takes the rest (30 bits)
    // Here's the values for the view :
    //  MAIN_VIEW 0
    //  SUB_VIEW  1

    pub const NPPM_GETFULLPATHFROMBUFFERID: c_uint = NPPMSG + 58;
    // INT NPPM_GETFULLPATHFROMBUFFERID(UINT_PTR bufferID, TCHAR *fullFilePath)
    // Get full path file name from a bufferID.
    // Return -1 if the bufferID non existing, otherwise the number of TCHAR copied/to copy
    // User should call it with fullFilePath be NULL to get the number of TCHAR (not including the nul character),
    // allocate fullFilePath with the return values + 1, then call it again to get full path file name

    pub const NPPM_GETBUFFERIDFROMPOS: c_uint = NPPMSG + 59;
    // LRESULT NPPM_GETBUFFERIDFROMPOS(INT index, INT iView)
    // wParam: Position of document
    // lParam: View to use, 0 = Main, 1 = Secondary
    // Returns 0 if invalid

    pub const NPPM_GETCURRENTBUFFERID: c_uint = NPPMSG + 60;
    // LRESULT NPPM_GETCURRENTBUFFERID(0, 0)
    // Returns active Buffer

    pub const NPPM_RELOADBUFFERID: c_uint = NPPMSG + 61;
    // VOID NPPM_RELOADBUFFERID(UINT_PTR bufferID, BOOL alert)
    // Reloads Buffer
    // wParam: Buffer to reload
    // lParam: 0 if no alert, else alert

    pub const NPPM_GETBUFFERLANGTYPE: c_uint = NPPMSG + 64;
    // INT NPPM_GETBUFFERLANGTYPE(UINT_PTR bufferID, 0)
    // wParam: BufferID to get LangType from
    // lParam: 0
    // Returns as int, see LangType. -1 on error

    pub const NPPM_SETBUFFERLANGTYPE: c_uint = NPPMSG + 65;
    // BOOL NPPM_SETBUFFERLANGTYPE(UINT_PTR bufferID, INT langType)
    // wParam: BufferID to set LangType of
    // lParam: LangType
    // Returns TRUE on success, FALSE otherwise
    // use int, see LangType for possible values
    // L_USER and L_EXTERNAL are not supported

    pub const NPPM_GETBUFFERENCODING: c_uint = NPPMSG + 66;
    // INT NPPM_GETBUFFERENCODING(UINT_PTR bufferID, 0)
    // wParam: BufferID to get encoding from
    // lParam: 0
    // returns as int, see UniMode. -1 on error

    pub const NPPM_SETBUFFERENCODING: c_uint = NPPMSG + 67;
    // BOOL NPPM_SETBUFFERENCODING(UINT_PTR bufferID, INT encoding)
    // wParam: BufferID to set encoding of
    // lParam: encoding
    // Returns TRUE on success, FALSE otherwise
    // use int, see UniMode
    // Can only be done on new, unedited files

    pub const NPPM_GETBUFFERFORMAT: c_uint = NPPMSG + 68;
    // INT NPPM_GETBUFFERFORMAT(UINT_PTR bufferID, 0)
    // wParam: BufferID to get EolType format from
    // lParam: 0
    // returns as int, see EolType format. -1 on error

    pub const NPPM_SETBUFFERFORMAT: c_uint = NPPMSG + 69;
    // BOOL NPPM_SETBUFFERFORMAT(UINT_PTR bufferID, INT format)
    // wParam: BufferID to set EolType format of
    // lParam: format
    // Returns TRUE on success, FALSE otherwise
    // use int, see EolType format

    pub const NPPM_HIDETOOLBAR: c_uint = NPPMSG + 70;
    // BOOL NPPM_HIDETOOLBAR(0, BOOL hideOrNot)
    // if hideOrNot is set as TRUE then tool bar will be hidden
    // otherwise it'll be shown.
    // return value : the old status value

    pub const NPPM_ISTOOLBARHIDDEN: c_uint = NPPMSG + 71;
    // BOOL NPPM_ISTOOLBARHIDDEN(0, 0)
    // returned value : TRUE if tool bar is hidden, otherwise FALSE

    pub const NPPM_HIDEMENU: c_uint = NPPMSG + 72;
    // BOOL NPPM_HIDEMENU(0, BOOL hideOrNot)
    // if hideOrNot is set as TRUE then menu will be hidden
    // otherwise it'll be shown.
    // return value : the old status value

    pub const NPPM_ISMENUHIDDEN: c_uint = NPPMSG + 73;
    // BOOL NPPM_ISMENUHIDDEN(0, 0)
    // returned value : TRUE if menu is hidden, otherwise FALSE

    pub const NPPM_HIDESTATUSBAR: c_uint = NPPMSG + 74;
    // BOOL NPPM_HIDESTATUSBAR(0, BOOL hideOrNot)
    // if hideOrNot is set as TRUE then STATUSBAR will be hidden
    // otherwise it'll be shown.
    // return value : the old status value

    pub const NPPM_ISSTATUSBARHIDDEN: c_uint = NPPMSG + 75;
    // BOOL NPPM_ISSTATUSBARHIDDEN(0, 0)
    // returned value : TRUE if STATUSBAR is hidden, otherwise FALSE

    pub const NPPM_GETSHORTCUTBYCMDID: c_uint = NPPMSG + 76;
    // BOOL NPPM_GETSHORTCUTBYCMDID(int cmdID, ShortcutKey *sk)
    // get your plugin command current mapped shortcut into sk via cmdID
    // You may need it after getting NPPN_READY notification
    // returned value : TRUE if this function call is successful and shortcut is enable, otherwise FALSE

    pub const NPPM_DOOPEN: c_uint = NPPMSG + 77;
    // BOOL NPPM_DOOPEN(0, const TCHAR *fullPathName2Open)
    // fullPathName2Open indicates the full file path name to be opened.
    // The return value is TRUE (1) if the operation is successful, otherwise FALSE (0).

    pub const NPPM_SAVECURRENTFILEAS: c_uint = NPPMSG + 78;
    // BOOL NPPM_SAVECURRENTFILEAS (BOOL asCopy, const TCHAR* filename)

    pub const NPPM_GETCURRENTNATIVELANGENCODING: c_uint = NPPMSG + 79;
    // INT NPPM_GETCURRENTNATIVELANGENCODING(0, 0)
    // returned value : the current native language encoding

    pub const NPPM_ALLOCATESUPPORTED: c_uint = NPPMSG + 80;
    // returns TRUE if NPPM_ALLOCATECMDID is supported
    // Use to identify if subclassing is necessary

    pub const NPPM_ALLOCATECMDID: c_uint = NPPMSG + 81;
    // BOOL NPPM_ALLOCATECMDID(int numberRequested, int* startNumber)
    // sets startNumber to the initial command ID if successful
    // Returns: TRUE if successful, FALSE otherwise. startNumber will also be set to 0 if unsuccessful

    pub const NPPM_ALLOCATEMARKER: c_uint = NPPMSG + 82;
    // BOOL NPPM_ALLOCATEMARKER(int numberRequested, int* startNumber)
    // sets startNumber to the initial command ID if successful
    // Allocates a marker number to a plugin: if a plugin need to add a marker on Notepad++'s Scintilla marker margin,
    // it has to use this message to get marker number, in order to prevent from the conflict with the other plugins.
    // Returns: TRUE if successful, FALSE otherwise. startNumber will also be set to 0 if unsuccessful

    pub const NPPM_GETLANGUAGENAME: c_uint = NPPMSG + 83;
    // INT NPPM_GETLANGUAGENAME(int langType, TCHAR *langName)
    // Get programming language name from the given language type (LangType)
    // Return value is the number of copied character / number of character to copy (\0 is not included)
    // You should call this function 2 times - the first time you pass langName as NULL to get the number of characters to copy.
    // You allocate a buffer of the length of (the number of characters + 1) then call NPPM_GETLANGUAGENAME function the 2nd time
    // by passing allocated buffer as argument langName

    pub const NPPM_GETLANGUAGEDESC: c_uint = NPPMSG + 84;
    // INT NPPM_GETLANGUAGEDESC(int langType, TCHAR *langDesc)
    // Get programming language short description from the given language type (LangType)
    // Return value is the number of copied character / number of character to copy (\0 is not included)
    // You should call this function 2 times - the first time you pass langDesc as NULL to get the number of characters to copy.
    // You allocate a buffer of the length of (the number of characters + 1) then call NPPM_GETLANGUAGEDESC function the 2nd time
    // by passing allocated buffer as argument langDesc

    pub const NPPM_SHOWDOCLIST: c_uint = NPPMSG + 85;
    // VOID NPPM_SHOWDOCLIST(0, BOOL toShowOrNot)
    // Send this message to show or hide Document List.
    // if toShowOrNot is TRUE then show Document List, otherwise hide it.

    pub const NPPM_ISDOCLISTSHOWN: c_uint = NPPMSG + 86;
    // BOOL NPPM_ISDOCLISTSHOWN(0, 0)
    // Check to see if Document List is shown.

    pub const NPPM_GETAPPDATAPLUGINSALLOWED: c_uint = NPPMSG + 87;
    // BOOL NPPM_GETAPPDATAPLUGINSALLOWED(0, 0)
    // Check to see if loading plugins from "%APPDATA%\..\Local\Notepad++\plugins" is allowed.

    pub const NPPM_GETCURRENTVIEW: c_uint = NPPMSG + 88;
    // INT NPPM_GETCURRENTVIEW(0, 0)
    // Return: current edit view of Notepad++. Only 2 possible values: 0 = Main, 1 = Secondary

    pub const NPPM_DOCLISTDISABLEEXTCOLUMN: c_uint = NPPMSG + 89;
    // VOID NPPM_DOCLISTDISABLEEXTCOLUMN(0, BOOL disableOrNot)
    // Disable or enable extension column of Document List

    pub const NPPM_DOCLISTDISABLEPATHCOLUMN: c_uint = NPPMSG + 102;
    // VOID NPPM_DOCLISTDISABLEPATHCOLUMN(0, BOOL disableOrNot)
    // Disable or enable path column of Document List

    pub const NPPM_GETEDITORDEFAULTFOREGROUNDCOLOR: c_uint = NPPMSG + 90;
    // INT NPPM_GETEDITORDEFAULTFOREGROUNDCOLOR(0, 0)
    // Return: current editor default foreground color. You should convert the returned value in COLORREF

    pub const NPPM_GETEDITORDEFAULTBACKGROUNDCOLOR: c_uint = NPPMSG + 91;
    // INT NPPM_GETEDITORDEFAULTBACKGROUNDCOLOR(0, 0)
    // Return: current editor default background color. You should convert the returned value in COLORREF

    pub const NPPM_SETSMOOTHFONT: c_uint = NPPMSG + 92;
    // VOID NPPM_SETSMOOTHFONT(0, BOOL setSmoothFontOrNot)

    pub const NPPM_SETEDITORBORDEREDGE: c_uint = NPPMSG + 93;
    // VOID NPPM_SETEDITORBORDEREDGE(0, BOOL withEditorBorderEdgeOrNot)

    pub const NPPM_SAVEFILE: c_uint = NPPMSG + 94;
    // VOID NPPM_SAVEFILE(0, const TCHAR *fileNameToSave)

    pub const NPPM_DISABLEAUTOUPDATE: c_uint = NPPMSG + 95; // 2119 in decimal
                                                            // VOID NPPM_DISABLEAUTOUPDATE(0, 0)

    pub const NPPM_REMOVESHORTCUTBYCMDID: c_uint = NPPMSG + 96; // 2120 in decimal
                                                                // BOOL NPPM_REMOVESHORTCUTASSIGNMENT(int cmdID)
                                                                // removes the assigned shortcut mapped to cmdID
                                                                // returned value : TRUE if function call is successful, otherwise FALSE

    pub const NPPM_GETPLUGINHOMEPATH: c_uint = NPPMSG + 97;
    // INT NPPM_GETPLUGINHOMEPATH(size_t strLen, TCHAR *pluginRootPath)
    // Get plugin home root path. It's useful if plugins want to get its own path
    // by appending <pluginFolderName> which is the name of plugin without extension part.
    // Returns the number of TCHAR copied/to copy.
    // Users should call it with pluginRootPath be NULL to get the required number of TCHAR (not including the terminating nul character),
    // allocate pluginRootPath buffer with the return value + 1, then call it again to get the path.

    pub const NPPM_GETSETTINGSONCLOUDPATH: c_uint = NPPMSG + 98;
    // INT NPPM_GETSETTINGSCLOUDPATH(size_t strLen, TCHAR *settingsOnCloudPath)
    // Get settings on cloud path. It's useful if plugins want to store its settings on Cloud, if this path is set.
    // Returns the number of TCHAR copied/to copy. If the return value is 0, then this path is not set, or the "strLen" is not enough to copy the path.
    // Users should call it with settingsCloudPath be NULL to get the required number of TCHAR (not including the terminating nul character),
    // allocate settingsCloudPath buffer with the return value + 1, then call it again to get the path.

    pub const NPPM_SETLINENUMBERWIDTHMODE: c_uint = NPPMSG + 99;
    const LINENUMWIDTH_DYNAMIC: c_uint = 0;
    const LINENUMWIDTH_CONSTANT: c_uint = 1;
    // BOOL NPPM_SETLINENUMBERWIDTHMODE(0, INT widthMode)
    // Set line number margin width in dynamic width mode (LINENUMWIDTH_DYNAMIC) or constant width mode (LINENUMWIDTH_CONSTANT)
    // It may help some plugins to disable non-dynamic line number margins width to have a smoothly visual effect while vertical scrolling the content in Notepad++
    // If calling is successful return TRUE, otherwise return FALSE.

    pub const NPPM_GETLINENUMBERWIDTHMODE: c_uint = NPPMSG + 100;
    // INT NPPM_GETLINENUMBERWIDTHMODE(0, 0)
    // Get line number margin width in dynamic width mode (LINENUMWIDTH_DYNAMIC) or constant width mode (LINENUMWIDTH_CONSTANT)

    pub const NPPM_ADDTOOLBARICON_FORDARKMODE: c_uint = NPPMSG + 101;
    // VOID NPPM_ADDTOOLBARICON_FORDARKMODE(UINT funcItem[X]._cmdID, toolbarIconsWithDarkMode iconHandles)
    // Use NPPM_ADDTOOLBARICON_FORDARKMODE instead obsolete NPPM_ADDTOOLBARICON which doesn't support the dark mode
    // 2 formats / 3 icons are needed:  1 * BMP + 2 * ICO
    // All 3 handles below should be set so the icon will be displayed correctly if toolbar icon sets are changed by users, also in dark mode
    /*struct toolbarIconsWithDarkMode {
     HBITMAP	hToolbarBmp;
     HICON	hToolbarIcon;
     HICON	hToolbarIconDarkMode;
    };*/

    pub const NPPM_GETEXTERNALLEXERAUTOINDENTMODE: c_uint = NPPMSG + 103;
    // BOOL NPPM_GETEXTERNALLEXERAUTOINDENTMODE(const TCHAR *languageName, ExternalLexerAutoIndentMode &autoIndentMode)
    // Get ExternalLexerAutoIndentMode for an installed external programming language.
    // - Standard means Notepad++ will keep the same TAB indentation between lines;
    // - C_Like means Notepad++ will perform a C-Language style indentation for the selected external language;
    // - Custom means a Plugin will be controlling auto-indentation for the current language.
    // returned values: TRUE for successful searches, otherwise FALSE.

    pub const NPPM_SETEXTERNALLEXERAUTOINDENTMODE: c_uint = NPPMSG + 104;
    // BOOL NPPM_SETEXTERNALLEXERAUTOINDENTMODE(const TCHAR *languageName, ExternalLexerAutoIndentMode autoIndentMode)
    // Set ExternalLexerAutoIndentMode for an installed external programming language.
    // - Standard means Notepad++ will keep the same TAB indentation between lines;
    // - C_Like means Notepad++ will perform a C-Language style indentation for the selected external language;
    // - Custom means a Plugin will be controlling auto-indentation for the current language.
    // returned value: TRUE if function call was successful, otherwise FALSE.

    pub const NPPM_ISAUTOINDENTON: c_uint = NPPMSG + 105;
    // BOOL NPPM_ISAUTOINDENTON(0, 0)
    // Returns the current Use Auto-Indentation setting in Notepad++ Preferences.

    pub const NPPM_GETCURRENTMACROSTATUS: c_uint = NPPMSG + 106;
    // MacroStatus NPPM_GETCURRENTMACROSTATUS(0, 0)
    // Gets current enum class MacroStatus { Idle - means macro is not in use and it's empty, RecordInProgress, RecordingStopped, PlayingBack }

    pub const NPPM_ISDARKMODEENABLED: c_uint = NPPMSG + 107;
    // bool NPPM_ISDARKMODEENABLED(0, 0)
    // Returns true when Notepad++ Dark Mode is enable, false when it is not.

    pub const NPPM_GETDARKMODECOLORS: c_uint = NPPMSG + 108;
    // bool NPPM_GETDARKMODECOLORS (size_t cbSize, NppDarkMode::Colors* returnColors)
    // - cbSize must be filled with sizeof(NppDarkMode::Colors).
    // - returnColors must be a pre-allocated NppDarkMode::Colors struct.
    // Returns true when successful, false otherwise.
    // You need to uncomment the following code to use NppDarkMode::Colors structure:
    //
    // namespace NppDarkMode
    // {
    //	struct Colors
    //	{
    //		COLORREF background = 0;
    //		COLORREF softerBackground = 0;
    //		COLORREF hotBackground = 0;
    //		COLORREF pureBackground = 0;
    //		COLORREF errorBackground = 0;
    //		COLORREF text = 0;
    //		COLORREF darkerText = 0;
    //		COLORREF disabledText = 0;
    //		COLORREF linkText = 0;
    //		COLORREF edge = 0;
    //		COLORREF hotEdge = 0;
    //		COLORREF disabledEdge = 0;
    //	};
    // }
    //
    // Note: in the case of calling failure ("false" is returned), you may need to change NppDarkMode::Colors structure to:
    // https://github.com/notepad-plus-plus/notepad-plus-plus/blob/master/PowerEditor/src/NppDarkMode.h#L32

    pub const NPPM_GETCURRENTCMDLINE: c_uint = NPPMSG + 109;
    // INT NPPM_GETCURRENTCMDLINE(size_t strLen, TCHAR *commandLineStr)
    // Get the Current Command Line string.
    // Returns the number of TCHAR copied/to copy.
    // Users should call it with commandLineStr as NULL to get the required number of TCHAR (not including the terminating nul character),
    // allocate commandLineStr buffer with the return value + 1, then call it again to get the current command line string.

    pub const NPPM_CREATELEXER: c_uint = NPPMSG + 110;
    // void* NPPN_CREATELEXER(0, const TCHAR *lexer_name)
    // Returns the ILexer pointer created by Lexilla

    pub const NPPM_GETBOOKMARKID: c_uint = NPPMSG + 111;
    // void* NPPM_GETBOOKMARKID(0, 0)
    // Returns the bookmark ID

    // For RUNCOMMAND_USER
    const VAR_NOT_RECOGNIZED: c_uint = 0;
    const FULL_CURRENT_PATH: c_uint = 1;
    const CURRENT_DIRECTORY: c_uint = 2;
    const FILE_NAME: c_uint = 3;
    const NAME_PART: c_uint = 4;
    const EXT_PART: c_uint = 5;
    const CURRENT_WORD: c_uint = 6;
    const NPP_DIRECTORY: c_uint = 7;
    const CURRENT_LINE: c_uint = 8;
    const CURRENT_COLUMN: c_uint = 9;
    const NPP_FULL_FILE_PATH: c_uint = 10;
    const GETFILENAMEATCURSOR: c_uint = 11;
    const CURRENT_LINESTR: c_uint = 12;

    const RUNCOMMAND_USER: c_uint = super::WM_USER + 3000;

    pub const NPPM_GETFULLCURRENTPATH: c_uint = RUNCOMMAND_USER + FULL_CURRENT_PATH;
    pub const NPPM_GETCURRENTDIRECTORY: c_uint = RUNCOMMAND_USER + CURRENT_DIRECTORY;
    pub const NPPM_GETFILENAME: c_uint = RUNCOMMAND_USER + FILE_NAME;
    pub const NPPM_GETNAMEPART: c_uint = RUNCOMMAND_USER + NAME_PART;
    pub const NPPM_GETEXTPART: c_uint = RUNCOMMAND_USER + EXT_PART;
    pub const NPPM_GETCURRENTWORD: c_uint = RUNCOMMAND_USER + CURRENT_WORD;
    pub const NPPM_GETNPPDIRECTORY: c_uint = RUNCOMMAND_USER + NPP_DIRECTORY;
    pub const NPPM_GETFILENAMEATCURSOR: c_uint = RUNCOMMAND_USER + GETFILENAMEATCURSOR;
    pub const NPPM_GETCURRENTLINESTR: c_uint = RUNCOMMAND_USER + CURRENT_LINESTR;
    // BOOL NPPM_GETXXXXXXXXXXXXXXXX(size_t strLen, TCHAR *str)
    // where str is the allocated TCHAR array,
    //	     strLen is the allocated array size
    // The return value is TRUE when get generic_string operation success
    // Otherwise (allocated array size is too small) FALSE

    pub const NPPM_GETCURRENTLINE: c_uint = RUNCOMMAND_USER + CURRENT_LINE;
    // INT NPPM_GETCURRENTLINE(0, 0)
    // return the caret current position line
    pub const NPPM_GETCURRENTCOLUMN: c_uint = RUNCOMMAND_USER + CURRENT_COLUMN;
    // INT NPPM_GETCURRENTCOLUMN(0, 0)
    // return the caret current position column

    pub const NPPM_GETNPPFULLFILEPATH: c_uint = RUNCOMMAND_USER + NPP_FULL_FILE_PATH;

    // Notification code
    const NPPN_FIRST: c_uint = 1000;
    const NPPN_READY: c_uint = NPPN_FIRST + 1; // To notify plugins that all the procedures of launchment of notepad++ are done.;
                                               //scnNotification->nmhdr.code = NPPN_READY;
                                               //scnNotification->nmhdr.hwndFrom = hwndNpp;
                                               //scnNotification->nmhdr.idFrom = 0;

    const NPPN_TBMODIFICATION: c_uint = NPPN_FIRST + 2; // To notify plugins that toolbar icons can be registered
                                                        //scnNotification->nmhdr.code = NPPN_TBMODIFICATION;
                                                        //scnNotification->nmhdr.hwndFrom = hwndNpp;
                                                        //scnNotification->nmhdr.idFrom = 0;

    const NPPN_FILEBEFORECLOSE: c_uint = NPPN_FIRST + 3; // To notify plugins that the current file is about to be closed
                                                         //scnNotification->nmhdr.code = NPPN_FILEBEFORECLOSE;
                                                         //scnNotification->nmhdr.hwndFrom = hwndNpp;
                                                         //scnNotification->nmhdr.idFrom = BufferID;

    const NPPN_FILEOPENED: c_uint = NPPN_FIRST + 4; // To notify plugins that the current file is just opened
                                                    //scnNotification->nmhdr.code = NPPN_FILEOPENED;
                                                    //scnNotification->nmhdr.hwndFrom = hwndNpp;
                                                    //scnNotification->nmhdr.idFrom = BufferID;

    const NPPN_FILECLOSED: c_uint = NPPN_FIRST + 5; // To notify plugins that the current file is just closed
                                                    //scnNotification->nmhdr.code = NPPN_FILECLOSED;
                                                    //scnNotification->nmhdr.hwndFrom = hwndNpp;
                                                    //scnNotification->nmhdr.idFrom = BufferID;

    const NPPN_FILEBEFOREOPEN: c_uint = NPPN_FIRST + 6; // To notify plugins that the current file is about to be opened
                                                        //scnNotification->nmhdr.code = NPPN_FILEBEFOREOPEN;
                                                        //scnNotification->nmhdr.hwndFrom = hwndNpp;
                                                        //scnNotification->nmhdr.idFrom = BufferID;

    const NPPN_FILEBEFORESAVE: c_uint = NPPN_FIRST + 7; // To notify plugins that the current file is about to be saved
                                                        //scnNotification->nmhdr.code = NPPN_FILEBEFOREOPEN;
                                                        //scnNotification->nmhdr.hwndFrom = hwndNpp;
                                                        //scnNotification->nmhdr.idFrom = BufferID;

    const NPPN_FILESAVED: c_uint = NPPN_FIRST + 8; // To notify plugins that the current file is just saved
                                                   //scnNotification->nmhdr.code = NPPN_FILESAVED;
                                                   //scnNotification->nmhdr.hwndFrom = hwndNpp;
                                                   //scnNotification->nmhdr.idFrom = BufferID;

    const NPPN_SHUTDOWN: c_uint = NPPN_FIRST + 9; // To notify plugins that Notepad++ is about to be shutdowned.
                                                  //scnNotification->nmhdr.code = NPPN_SHUTDOWN;
                                                  //scnNotification->nmhdr.hwndFrom = hwndNpp;
                                                  //scnNotification->nmhdr.idFrom = 0;

    const NPPN_BUFFERACTIVATED: c_uint = NPPN_FIRST + 10; // To notify plugins that a buffer was activated (put to foreground).
                                                          //scnNotification->nmhdr.code = NPPN_BUFFERACTIVATED;
                                                          //scnNotification->nmhdr.hwndFrom = hwndNpp;
                                                          //scnNotification->nmhdr.idFrom = activatedBufferID;

    const NPPN_LANGCHANGED: c_uint = NPPN_FIRST + 11; // To notify plugins that the language in the current doc is just changed.
                                                      //scnNotification->nmhdr.code = NPPN_LANGCHANGED;
                                                      //scnNotification->nmhdr.hwndFrom = hwndNpp;
                                                      //scnNotification->nmhdr.idFrom = currentBufferID;

    const NPPN_WORDSTYLESUPDATED: c_uint = NPPN_FIRST + 12; // To notify plugins that user initiated a WordStyleDlg change.
                                                            //scnNotification->nmhdr.code = NPPN_WORDSTYLESUPDATED;
                                                            //scnNotification->nmhdr.hwndFrom = hwndNpp;
                                                            //scnNotification->nmhdr.idFrom = currentBufferID;

    const NPPN_SHORTCUTREMAPPED: c_uint = NPPN_FIRST + 13; // To notify plugins that plugin command shortcut is remapped.
                                                           //scnNotification->nmhdr.code = NPPN_SHORTCUTSREMAPPED;
                                                           //scnNotification->nmhdr.hwndFrom = ShortcutKeyStructurePointer;
                                                           //scnNotification->nmhdr.idFrom = cmdID;
                                                           //where ShortcutKeyStructurePointer is pointer of struct ShortcutKey:
                                                           //struct ShortcutKey {
                                                           //	bool _isCtrl;
                                                           //	bool _isAlt;
                                                           //	bool _isShift;
                                                           //	UCHAR _key;
                                                           //};

    const NPPN_FILEBEFORELOAD: c_uint = NPPN_FIRST + 14; // To notify plugins that the current file is about to be loaded
                                                         //scnNotification->nmhdr.code = NPPN_FILEBEFOREOPEN;
                                                         //scnNotification->nmhdr.hwndFrom = hwndNpp;
                                                         //scnNotification->nmhdr.idFrom = NULL;

    const NPPN_FILELOADFAILED: c_uint = NPPN_FIRST + 15; // To notify plugins that file open operation failed
                                                         //scnNotification->nmhdr.code = NPPN_FILEOPENFAILED;
                                                         //scnNotification->nmhdr.hwndFrom = hwndNpp;
                                                         //scnNotification->nmhdr.idFrom = BufferID;

    const NPPN_READONLYCHANGED: c_uint = NPPN_FIRST + 16; // To notify plugins that current document change the readonly status,
                                                          //scnNotification->nmhdr.code = NPPN_READONLYCHANGED;
                                                          //scnNotification->nmhdr.hwndFrom = bufferID;
                                                          //scnNotification->nmhdr.idFrom = docStatus;
                                                          // where bufferID is BufferID
                                                          //       docStatus can be combined by DOCSTATUS_READONLY and DOCSTATUS_BUFFERDIRTY

    const DOCSTATUS_READONLY: c_uint = 1;
    const DOCSTATUS_BUFFERDIRTY: c_uint = 2;

    const NPPN_DOCORDERCHANGED: c_uint = NPPN_FIRST + 17; // To notify plugins that document order is changed
                                                          //scnNotification->nmhdr.code = NPPN_DOCORDERCHANGED;
                                                          //scnNotification->nmhdr.hwndFrom = newIndex;
                                                          //scnNotification->nmhdr.idFrom = BufferID;

    const NPPN_SNAPSHOTDIRTYFILELOADED: c_uint = NPPN_FIRST + 18; // To notify plugins that a snapshot dirty file is loaded on startup
                                                                  //scnNotification->nmhdr.code = NPPN_SNAPSHOTDIRTYFILELOADED;
                                                                  //scnNotification->nmhdr.hwndFrom = NULL;
                                                                  //scnNotification->nmhdr.idFrom = BufferID;

    const NPPN_BEFORESHUTDOWN: c_uint = NPPN_FIRST + 19; // To notify plugins that Npp shutdown has been triggered, files have not been closed yet
                                                         //scnNotification->nmhdr.code = NPPN_BEFORESHUTDOWN;
                                                         //scnNotification->nmhdr.hwndFrom = hwndNpp;
                                                         //scnNotification->nmhdr.idFrom = 0;

    const NPPN_CANCELSHUTDOWN: c_uint = NPPN_FIRST + 20; // To notify plugins that Npp shutdown has been cancelled
                                                         //scnNotification->nmhdr.code = NPPN_CANCELSHUTDOWN;
                                                         //scnNotification->nmhdr.hwndFrom = hwndNpp;
                                                         //scnNotification->nmhdr.idFrom = 0;

    const NPPN_FILEBEFORERENAME: c_uint = NPPN_FIRST + 21; // To notify plugins that file is to be renamed
                                                           //scnNotification->nmhdr.code = NPPN_FILEBEFORERENAME;
                                                           //scnNotification->nmhdr.hwndFrom = hwndNpp;
                                                           //scnNotification->nmhdr.idFrom = BufferID;

    const NPPN_FILERENAMECANCEL: c_uint = NPPN_FIRST + 22; // To notify plugins that file rename has been cancelled
                                                           //scnNotification->nmhdr.code = NPPN_FILERENAMECANCEL;
                                                           //scnNotification->nmhdr.hwndFrom = hwndNpp;
                                                           //scnNotification->nmhdr.idFrom = BufferID;

    const NPPN_FILERENAMED: c_uint = NPPN_FIRST + 23; // To notify plugins that file has been renamed
                                                      //scnNotification->nmhdr.code = NPPN_FILERENAMED;
                                                      //scnNotification->nmhdr.hwndFrom = hwndNpp;
                                                      //scnNotification->nmhdr.idFrom = BufferID;

    const NPPN_FILEBEFOREDELETE: c_uint = NPPN_FIRST + 24; // To notify plugins that file is to be deleted
                                                           //scnNotification->nmhdr.code = NPPN_FILEBEFOREDELETE;
                                                           //scnNotification->nmhdr.hwndFrom = hwndNpp;
                                                           //scnNotification->nmhdr.idFrom = BufferID;

    const NPPN_FILEDELETEFAILED: c_uint = NPPN_FIRST + 25; // To notify plugins that file deletion has failed
                                                           //scnNotification->nmhdr.code = NPPN_FILEDELETEFAILED;
                                                           //scnNotification->nmhdr.hwndFrom = hwndNpp;
                                                           //scnNotification->nmhdr.idFrom = BufferID;

    const NPPN_FILEDELETED: c_uint = NPPN_FIRST + 26; // To notify plugins that file has been deleted
                                                      //scnNotification->nmhdr.code = NPPN_FILEDELETED;
                                                      //scnNotification->nmhdr.hwndFrom = hwndNpp;
                                                      //scnNotification->nmhdr.idFrom = BufferID;

    const NPPN_DARKMODECHANGED: c_uint = NPPN_FIRST + 27; // To notify plugins that Dark Mode was enabled/disabled
                                                          //scnNotification->nmhdr.code = NPPN_DARKMODECHANGED;
                                                          //scnNotification->nmhdr.hwndFrom = hwndNpp;
                                                          //scnNotification->nmhdr.idFrom = 0;

    const NPPN_CMDLINEPLUGINMSG: c_uint = NPPN_FIRST + 28; // To notify plugins that the new argument for plugins (via '-pluginMessage="YOUR_PLUGIN_ARGUMENT"' in command line) is available
                                                           //scnNotification->nmhdr.code = NPPN_CMDLINEPLUGINMSG;
                                                           //scnNotification->nmhdr.hwndFrom = hwndNpp;
                                                           //scnNotification->nmhdr.idFrom = pluginMessage; //where pluginMessage is pointer of type wchar_t
}

#[allow(dead_code)]
pub mod sci {
    use super::c_uint;

    pub const INVALID_POSITION: i32 = -1;
    pub const SCI_START: c_uint = 2000;
    pub const SCI_OPTIONAL_START: c_uint = 3000;
    pub const SCI_LEXER_START: c_uint = 4000;
    pub const SCI_ADDTEXT: c_uint = 2001;
    pub const SCI_ADDSTYLEDTEXT: c_uint = 2002;
    pub const SCI_INSERTTEXT: c_uint = 2003;
    pub const SCI_CHANGEINSERTION: c_uint = 2672;
    pub const SCI_CLEARALL: c_uint = 2004;
    pub const SCI_DELETERANGE: c_uint = 2645;
    pub const SCI_CLEARDOCUMENTSTYLE: c_uint = 2005;
    pub const SCI_GETLENGTH: c_uint = 2006;
    pub const SCI_GETCHARAT: c_uint = 2007;
    pub const SCI_GETCURRENTPOS: c_uint = 2008;
    pub const SCI_GETANCHOR: c_uint = 2009;
    pub const SCI_GETSTYLEAT: c_uint = 2010;
    pub const SCI_GETSTYLEINDEXAT: c_uint = 2038;
    pub const SCI_REDO: c_uint = 2011;
    pub const SCI_SETUNDOCOLLECTION: c_uint = 2012;
    pub const SCI_SELECTALL: c_uint = 2013;
    pub const SCI_SETSAVEPOINT: c_uint = 2014;
    pub const SCI_GETSTYLEDTEXT: c_uint = 2015;
    pub const SCI_CANREDO: c_uint = 2016;
    pub const SCI_MARKERLINEFROMHANDLE: c_uint = 2017;
    pub const SCI_MARKERDELETEHANDLE: c_uint = 2018;
    pub const SCI_MARKERHANDLEFROMLINE: c_uint = 2732;
    pub const SCI_MARKERNUMBERFROMLINE: c_uint = 2733;
    pub const SCI_GETUNDOCOLLECTION: c_uint = 2019;
    pub const SCWS_INVISIBLE: c_uint = 0;
    pub const SCWS_VISIBLEALWAYS: c_uint = 1;
    pub const SCWS_VISIBLEAFTERINDENT: c_uint = 2;
    pub const SCWS_VISIBLEONLYININDENT: c_uint = 3;
    pub const SCI_GETVIEWWS: c_uint = 2020;
    pub const SCI_SETVIEWWS: c_uint = 2021;
    pub const SCTD_LONGARROW: c_uint = 0;
    pub const SCTD_STRIKEOUT: c_uint = 1;
    pub const SCI_GETTABDRAWMODE: c_uint = 2698;
    pub const SCI_SETTABDRAWMODE: c_uint = 2699;
    pub const SCI_POSITIONFROMPOINT: c_uint = 2022;
    pub const SCI_POSITIONFROMPOINTCLOSE: c_uint = 2023;
    pub const SCI_GOTOLINE: c_uint = 2024;
    pub const SCI_GOTOPOS: c_uint = 2025;
    pub const SCI_SETANCHOR: c_uint = 2026;
    pub const SCI_GETCURLINE: c_uint = 2027;
    pub const SCI_GETENDSTYLED: c_uint = 2028;
    pub const SC_EOL_CRLF: c_uint = 0;
    pub const SC_EOL_CR: c_uint = 1;
    pub const SC_EOL_LF: c_uint = 2;
    pub const SCI_CONVERTEOLS: c_uint = 2029;
    pub const SCI_GETEOLMODE: c_uint = 2030;
    pub const SCI_SETEOLMODE: c_uint = 2031;
    pub const SCI_STARTSTYLING: c_uint = 2032;
    pub const SCI_SETSTYLING: c_uint = 2033;
    pub const SCI_GETBUFFEREDDRAW: c_uint = 2034;
    pub const SCI_SETBUFFEREDDRAW: c_uint = 2035;
    pub const SCI_SETTABWIDTH: c_uint = 2036;
    pub const SCI_GETTABWIDTH: c_uint = 2121;
    pub const SCI_SETTABMINIMUMWIDTH: c_uint = 2724;
    pub const SCI_GETTABMINIMUMWIDTH: c_uint = 2725;
    pub const SCI_CLEARTABSTOPS: c_uint = 2675;
    pub const SCI_ADDTABSTOP: c_uint = 2676;
    pub const SCI_GETNEXTTABSTOP: c_uint = 2677;
    pub const SC_CP_UTF8: c_uint = 65001;
    pub const SCI_SETCODEPAGE: c_uint = 2037;
    pub const SCI_SETFONTLOCALE: c_uint = 2760;
    pub const SCI_GETFONTLOCALE: c_uint = 2761;
    pub const SC_IME_WINDOWED: c_uint = 0;
    pub const SC_IME_INLINE: c_uint = 1;
    pub const SCI_GETIMEINTERACTION: c_uint = 2678;
    pub const SCI_SETIMEINTERACTION: c_uint = 2679;
    pub const SC_ALPHA_TRANSPARENT: c_uint = 0;
    pub const SC_ALPHA_OPAQUE: c_uint = 255;
    pub const SC_ALPHA_NOALPHA: c_uint = 256;
    pub const SC_CURSORNORMAL: i32 = -1;
    pub const SC_CURSORARROW: c_uint = 2;
    pub const SC_CURSORWAIT: c_uint = 4;
    pub const SC_CURSORREVERSEARROW: c_uint = 7;
    pub const MARKER_MAX: c_uint = 31;
    pub const SC_MARK_CIRCLE: c_uint = 0;
    pub const SC_MARK_ROUNDRECT: c_uint = 1;
    pub const SC_MARK_ARROW: c_uint = 2;
    pub const SC_MARK_SMALLRECT: c_uint = 3;
    pub const SC_MARK_SHORTARROW: c_uint = 4;
    pub const SC_MARK_EMPTY: c_uint = 5;
    pub const SC_MARK_ARROWDOWN: c_uint = 6;
    pub const SC_MARK_MINUS: c_uint = 7;
    pub const SC_MARK_PLUS: c_uint = 8;
    pub const SC_MARK_VLINE: c_uint = 9;
    pub const SC_MARK_LCORNER: c_uint = 10;
    pub const SC_MARK_TCORNER: c_uint = 11;
    pub const SC_MARK_BOXPLUS: c_uint = 12;
    pub const SC_MARK_BOXPLUSCONNECTED: c_uint = 13;
    pub const SC_MARK_BOXMINUS: c_uint = 14;
    pub const SC_MARK_BOXMINUSCONNECTED: c_uint = 15;
    pub const SC_MARK_LCORNERCURVE: c_uint = 16;
    pub const SC_MARK_TCORNERCURVE: c_uint = 17;
    pub const SC_MARK_CIRCLEPLUS: c_uint = 18;
    pub const SC_MARK_CIRCLEPLUSCONNECTED: c_uint = 19;
    pub const SC_MARK_CIRCLEMINUS: c_uint = 20;
    pub const SC_MARK_CIRCLEMINUSCONNECTED: c_uint = 21;
    pub const SC_MARK_BACKGROUND: c_uint = 22;
    pub const SC_MARK_DOTDOTDOT: c_uint = 23;
    pub const SC_MARK_ARROWS: c_uint = 24;
    pub const SC_MARK_PIXMAP: c_uint = 25;
    pub const SC_MARK_FULLRECT: c_uint = 26;
    pub const SC_MARK_LEFTRECT: c_uint = 27;
    pub const SC_MARK_AVAILABLE: c_uint = 28;
    pub const SC_MARK_UNDERLINE: c_uint = 29;
    pub const SC_MARK_RGBAIMAGE: c_uint = 30;
    pub const SC_MARK_BOOKMARK: c_uint = 31;
    pub const SC_MARK_VERTICALBOOKMARK: c_uint = 32;
    pub const SC_MARK_BAR: c_uint = 33;
    pub const SC_MARK_CHARACTER: c_uint = 10000;
    pub const SC_MARKNUM_HISTORY_REVERTED_TO_ORIGIN: c_uint = 21;
    pub const SC_MARKNUM_HISTORY_SAVED: c_uint = 22;
    pub const SC_MARKNUM_HISTORY_MODIFIED: c_uint = 23;
    pub const SC_MARKNUM_HISTORY_REVERTED_TO_MODIFIED: c_uint = 24;
    pub const SC_MARKNUM_FOLDEREND: c_uint = 25;
    pub const SC_MARKNUM_FOLDEROPENMID: c_uint = 26;
    pub const SC_MARKNUM_FOLDERMIDTAIL: c_uint = 27;
    pub const SC_MARKNUM_FOLDERTAIL: c_uint = 28;
    pub const SC_MARKNUM_FOLDERSUB: c_uint = 29;
    pub const SC_MARKNUM_FOLDER: c_uint = 30;
    pub const SC_MARKNUM_FOLDEROPEN: c_uint = 31;
    pub const SC_MASK_FOLDERS: c_uint = 0xFE000000;
    pub const SCI_MARKERDEFINE: c_uint = 2040;
    pub const SCI_MARKERSETFORE: c_uint = 2041;
    pub const SCI_MARKERSETBACK: c_uint = 2042;
    pub const SCI_MARKERSETBACKSELECTED: c_uint = 2292;
    pub const SCI_MARKERSETFORETRANSLUCENT: c_uint = 2294;
    pub const SCI_MARKERSETBACKTRANSLUCENT: c_uint = 2295;
    pub const SCI_MARKERSETBACKSELECTEDTRANSLUCENT: c_uint = 2296;
    pub const SCI_MARKERSETSTROKEWIDTH: c_uint = 2297;
    pub const SCI_MARKERENABLEHIGHLIGHT: c_uint = 2293;
    pub const SCI_MARKERADD: c_uint = 2043;
    pub const SCI_MARKERDELETE: c_uint = 2044;
    pub const SCI_MARKERDELETEALL: c_uint = 2045;
    pub const SCI_MARKERGET: c_uint = 2046;
    pub const SCI_MARKERNEXT: c_uint = 2047;
    pub const SCI_MARKERPREVIOUS: c_uint = 2048;
    pub const SCI_MARKERDEFINEPIXMAP: c_uint = 2049;
    pub const SCI_MARKERADDSET: c_uint = 2466;
    pub const SCI_MARKERSETALPHA: c_uint = 2476;
    pub const SCI_MARKERGETLAYER: c_uint = 2734;
    pub const SCI_MARKERSETLAYER: c_uint = 2735;
    pub const SC_MAX_MARGIN: c_uint = 4;
    pub const SC_MARGIN_SYMBOL: c_uint = 0;
    pub const SC_MARGIN_NUMBER: c_uint = 1;
    pub const SC_MARGIN_BACK: c_uint = 2;
    pub const SC_MARGIN_FORE: c_uint = 3;
    pub const SC_MARGIN_TEXT: c_uint = 4;
    pub const SC_MARGIN_RTEXT: c_uint = 5;
    pub const SC_MARGIN_COLOUR: c_uint = 6;
    pub const SCI_SETMARGINTYPEN: c_uint = 2240;
    pub const SCI_GETMARGINTYPEN: c_uint = 2241;
    pub const SCI_SETMARGINWIDTHN: c_uint = 2242;
    pub const SCI_GETMARGINWIDTHN: c_uint = 2243;
    pub const SCI_SETMARGINMASKN: c_uint = 2244;
    pub const SCI_GETMARGINMASKN: c_uint = 2245;
    pub const SCI_SETMARGINSENSITIVEN: c_uint = 2246;
    pub const SCI_GETMARGINSENSITIVEN: c_uint = 2247;
    pub const SCI_SETMARGINCURSORN: c_uint = 2248;
    pub const SCI_GETMARGINCURSORN: c_uint = 2249;
    pub const SCI_SETMARGINBACKN: c_uint = 2250;
    pub const SCI_GETMARGINBACKN: c_uint = 2251;
    pub const SCI_SETMARGINS: c_uint = 2252;
    pub const SCI_GETMARGINS: c_uint = 2253;
    pub const STYLE_DEFAULT: c_uint = 32;
    pub const STYLE_LINENUMBER: c_uint = 33;
    pub const STYLE_BRACELIGHT: c_uint = 34;
    pub const STYLE_BRACEBAD: c_uint = 35;
    pub const STYLE_CONTROLCHAR: c_uint = 36;
    pub const STYLE_INDENTGUIDE: c_uint = 37;
    pub const STYLE_CALLTIP: c_uint = 38;
    pub const STYLE_FOLDDISPLAYTEXT: c_uint = 39;
    pub const STYLE_LASTPREDEFINED: c_uint = 39;
    pub const STYLE_MAX: c_uint = 255;
    pub const SC_CHARSET_ANSI: c_uint = 0;
    pub const SC_CHARSET_DEFAULT: c_uint = 1;
    pub const SC_CHARSET_BALTIC: c_uint = 186;
    pub const SC_CHARSET_CHINESEBIG5: c_uint = 136;
    pub const SC_CHARSET_EASTEUROPE: c_uint = 238;
    pub const SC_CHARSET_GB2312: c_uint = 134;
    pub const SC_CHARSET_GREEK: c_uint = 161;
    pub const SC_CHARSET_HANGUL: c_uint = 129;
    pub const SC_CHARSET_MAC: c_uint = 77;
    pub const SC_CHARSET_OEM: c_uint = 255;
    pub const SC_CHARSET_RUSSIAN: c_uint = 204;
    pub const SC_CHARSET_OEM866: c_uint = 866;
    pub const SC_CHARSET_CYRILLIC: c_uint = 1251;
    pub const SC_CHARSET_SHIFTJIS: c_uint = 128;
    pub const SC_CHARSET_SYMBOL: c_uint = 2;
    pub const SC_CHARSET_TURKISH: c_uint = 162;
    pub const SC_CHARSET_JOHAB: c_uint = 130;
    pub const SC_CHARSET_HEBREW: c_uint = 177;
    pub const SC_CHARSET_ARABIC: c_uint = 178;
    pub const SC_CHARSET_VIETNAMESE: c_uint = 163;
    pub const SC_CHARSET_THAI: c_uint = 222;
    pub const SC_CHARSET_8859_15: c_uint = 1000;
    pub const SCI_STYLECLEARALL: c_uint = 2050;
    pub const SCI_STYLESETFORE: c_uint = 2051;
    pub const SCI_STYLESETBACK: c_uint = 2052;
    pub const SCI_STYLESETBOLD: c_uint = 2053;
    pub const SCI_STYLESETITALIC: c_uint = 2054;
    pub const SCI_STYLESETSIZE: c_uint = 2055;
    pub const SCI_STYLESETFONT: c_uint = 2056;
    pub const SCI_STYLESETEOLFILLED: c_uint = 2057;
    pub const SCI_STYLERESETDEFAULT: c_uint = 2058;
    pub const SCI_STYLESETUNDERLINE: c_uint = 2059;
    pub const SC_CASE_MIXED: c_uint = 0;
    pub const SC_CASE_UPPER: c_uint = 1;
    pub const SC_CASE_LOWER: c_uint = 2;
    pub const SC_CASE_CAMEL: c_uint = 3;
    pub const SCI_STYLEGETFORE: c_uint = 2481;
    pub const SCI_STYLEGETBACK: c_uint = 2482;
    pub const SCI_STYLEGETBOLD: c_uint = 2483;
    pub const SCI_STYLEGETITALIC: c_uint = 2484;
    pub const SCI_STYLEGETSIZE: c_uint = 2485;
    pub const SCI_STYLEGETFONT: c_uint = 2486;
    pub const SCI_STYLEGETEOLFILLED: c_uint = 2487;
    pub const SCI_STYLEGETUNDERLINE: c_uint = 2488;
    pub const SCI_STYLEGETCASE: c_uint = 2489;
    pub const SCI_STYLEGETCHARACTERSET: c_uint = 2490;
    pub const SCI_STYLEGETVISIBLE: c_uint = 2491;
    pub const SCI_STYLEGETCHANGEABLE: c_uint = 2492;
    pub const SCI_STYLEGETHOTSPOT: c_uint = 2493;
    pub const SCI_STYLESETCASE: c_uint = 2060;
    pub const SC_FONT_SIZE_MULTIPLIER: c_uint = 100;
    pub const SCI_STYLESETSIZEFRACTIONAL: c_uint = 2061;
    pub const SCI_STYLEGETSIZEFRACTIONAL: c_uint = 2062;
    pub const SC_WEIGHT_NORMAL: c_uint = 400;
    pub const SC_WEIGHT_SEMIBOLD: c_uint = 600;
    pub const SC_WEIGHT_BOLD: c_uint = 700;
    pub const SCI_STYLESETWEIGHT: c_uint = 2063;
    pub const SCI_STYLEGETWEIGHT: c_uint = 2064;
    pub const SCI_STYLESETCHARACTERSET: c_uint = 2066;
    pub const SCI_STYLESETHOTSPOT: c_uint = 2409;
    pub const SCI_STYLESETCHECKMONOSPACED: c_uint = 2254;
    pub const SCI_STYLEGETCHECKMONOSPACED: c_uint = 2255;
    pub const SCI_STYLESETINVISIBLEREPRESENTATION: c_uint = 2256;
    pub const SCI_STYLEGETINVISIBLEREPRESENTATION: c_uint = 2257;
    pub const SC_ELEMENT_LIST: c_uint = 0;
    pub const SC_ELEMENT_LIST_BACK: c_uint = 1;
    pub const SC_ELEMENT_LIST_SELECTED: c_uint = 2;
    pub const SC_ELEMENT_LIST_SELECTED_BACK: c_uint = 3;
    pub const SC_ELEMENT_SELECTION_TEXT: c_uint = 10;
    pub const SC_ELEMENT_SELECTION_BACK: c_uint = 11;
    pub const SC_ELEMENT_SELECTION_ADDITIONAL_TEXT: c_uint = 12;
    pub const SC_ELEMENT_SELECTION_ADDITIONAL_BACK: c_uint = 13;
    pub const SC_ELEMENT_SELECTION_SECONDARY_TEXT: c_uint = 14;
    pub const SC_ELEMENT_SELECTION_SECONDARY_BACK: c_uint = 15;
    pub const SC_ELEMENT_SELECTION_INACTIVE_TEXT: c_uint = 16;
    pub const SC_ELEMENT_SELECTION_INACTIVE_BACK: c_uint = 17;
    pub const SC_ELEMENT_CARET: c_uint = 40;
    pub const SC_ELEMENT_CARET_ADDITIONAL: c_uint = 41;
    pub const SC_ELEMENT_CARET_LINE_BACK: c_uint = 50;
    pub const SC_ELEMENT_WHITE_SPACE: c_uint = 60;
    pub const SC_ELEMENT_WHITE_SPACE_BACK: c_uint = 61;
    pub const SC_ELEMENT_HOT_SPOT_ACTIVE: c_uint = 70;
    pub const SC_ELEMENT_HOT_SPOT_ACTIVE_BACK: c_uint = 71;
    pub const SC_ELEMENT_FOLD_LINE: c_uint = 80;
    pub const SC_ELEMENT_HIDDEN_LINE: c_uint = 81;
    pub const SCI_SETELEMENTCOLOUR: c_uint = 2753;
    pub const SCI_GETELEMENTCOLOUR: c_uint = 2754;
    pub const SCI_RESETELEMENTCOLOUR: c_uint = 2755;
    pub const SCI_GETELEMENTISSET: c_uint = 2756;
    pub const SCI_GETELEMENTALLOWSTRANSLUCENT: c_uint = 2757;
    pub const SCI_GETELEMENTBASECOLOUR: c_uint = 2758;
    pub const SCI_SETSELFORE: c_uint = 2067;
    pub const SCI_SETSELBACK: c_uint = 2068;
    pub const SCI_GETSELALPHA: c_uint = 2477;
    pub const SCI_SETSELALPHA: c_uint = 2478;
    pub const SCI_GETSELEOLFILLED: c_uint = 2479;
    pub const SCI_SETSELEOLFILLED: c_uint = 2480;
    pub const SC_LAYER_BASE: c_uint = 0;
    pub const SC_LAYER_UNDER_TEXT: c_uint = 1;
    pub const SC_LAYER_OVER_TEXT: c_uint = 2;
    pub const SCI_GETSELECTIONLAYER: c_uint = 2762;
    pub const SCI_SETSELECTIONLAYER: c_uint = 2763;
    pub const SCI_GETCARETLINELAYER: c_uint = 2764;
    pub const SCI_SETCARETLINELAYER: c_uint = 2765;
    pub const SCI_GETCARETLINEHIGHLIGHTSUBLINE: c_uint = 2773;
    pub const SCI_SETCARETLINEHIGHLIGHTSUBLINE: c_uint = 2774;
    pub const SCI_SETCARETFORE: c_uint = 2069;
    pub const SCI_ASSIGNCMDKEY: c_uint = 2070;
    pub const SCI_CLEARCMDKEY: c_uint = 2071;
    pub const SCI_CLEARALLCMDKEYS: c_uint = 2072;
    pub const SCI_SETSTYLINGEX: c_uint = 2073;
    pub const SCI_STYLESETVISIBLE: c_uint = 2074;
    pub const SCI_GETCARETPERIOD: c_uint = 2075;
    pub const SCI_SETCARETPERIOD: c_uint = 2076;
    pub const SCI_SETWORDCHARS: c_uint = 2077;
    pub const SCI_GETWORDCHARS: c_uint = 2646;
    pub const SCI_SETCHARACTERCATEGORYOPTIMIZATION: c_uint = 2720;
    pub const SCI_GETCHARACTERCATEGORYOPTIMIZATION: c_uint = 2721;
    pub const SCI_BEGINUNDOACTION: c_uint = 2078;
    pub const SCI_ENDUNDOACTION: c_uint = 2079;
    pub const INDIC_PLAIN: c_uint = 0;
    pub const INDIC_SQUIGGLE: c_uint = 1;
    pub const INDIC_TT: c_uint = 2;
    pub const INDIC_DIAGONAL: c_uint = 3;
    pub const INDIC_STRIKE: c_uint = 4;
    pub const INDIC_HIDDEN: c_uint = 5;
    pub const INDIC_BOX: c_uint = 6;
    pub const INDIC_ROUNDBOX: c_uint = 7;
    pub const INDIC_STRAIGHTBOX: c_uint = 8;
    pub const INDIC_DASH: c_uint = 9;
    pub const INDIC_DOTS: c_uint = 10;
    pub const INDIC_SQUIGGLELOW: c_uint = 11;
    pub const INDIC_DOTBOX: c_uint = 12;
    pub const INDIC_SQUIGGLEPIXMAP: c_uint = 13;
    pub const INDIC_COMPOSITIONTHICK: c_uint = 14;
    pub const INDIC_COMPOSITIONTHIN: c_uint = 15;
    pub const INDIC_FULLBOX: c_uint = 16;
    pub const INDIC_TEXTFORE: c_uint = 17;
    pub const INDIC_POINT: c_uint = 18;
    pub const INDIC_POINTCHARACTER: c_uint = 19;
    pub const INDIC_GRADIENT: c_uint = 20;
    pub const INDIC_GRADIENTCENTRE: c_uint = 21;
    pub const INDIC_POINT_TOP: c_uint = 22;
    pub const INDIC_EXPLORERLINK: c_uint = 23;
    pub const INDIC_CONTAINER: c_uint = 8;
    pub const INDIC_IME: c_uint = 32;
    pub const INDIC_IME_MAX: c_uint = 35;
    pub const INDIC_MAX: c_uint = 35;
    pub const INDICATOR_CONTAINER: c_uint = 8;
    pub const INDICATOR_IME: c_uint = 32;
    pub const INDICATOR_IME_MAX: c_uint = 35;
    pub const INDICATOR_HISTORY_REVERTED_TO_ORIGIN_INSERTION: c_uint = 36;
    pub const INDICATOR_HISTORY_REVERTED_TO_ORIGIN_DELETION: c_uint = 37;
    pub const INDICATOR_HISTORY_SAVED_INSERTION: c_uint = 38;
    pub const INDICATOR_HISTORY_SAVED_DELETION: c_uint = 39;
    pub const INDICATOR_HISTORY_MODIFIED_INSERTION: c_uint = 40;
    pub const INDICATOR_HISTORY_MODIFIED_DELETION: c_uint = 41;
    pub const INDICATOR_HISTORY_REVERTED_TO_MODIFIED_INSERTION: c_uint = 42;
    pub const INDICATOR_HISTORY_REVERTED_TO_MODIFIED_DELETION: c_uint = 43;
    pub const INDICATOR_MAX: c_uint = 43;
    pub const SCI_INDICSETSTYLE: c_uint = 2080;
    pub const SCI_INDICGETSTYLE: c_uint = 2081;
    pub const SCI_INDICSETFORE: c_uint = 2082;
    pub const SCI_INDICGETFORE: c_uint = 2083;
    pub const SCI_INDICSETUNDER: c_uint = 2510;
    pub const SCI_INDICGETUNDER: c_uint = 2511;
    pub const SCI_INDICSETHOVERSTYLE: c_uint = 2680;
    pub const SCI_INDICGETHOVERSTYLE: c_uint = 2681;
    pub const SCI_INDICSETHOVERFORE: c_uint = 2682;
    pub const SCI_INDICGETHOVERFORE: c_uint = 2683;
    pub const SC_INDICVALUEBIT: c_uint = 0x1000000;
    pub const SC_INDICVALUEMASK: c_uint = 0xFFFFFF;
    pub const SC_INDICFLAG_NONE: c_uint = 0;
    pub const SC_INDICFLAG_VALUEFORE: c_uint = 1;
    pub const SCI_INDICSETFLAGS: c_uint = 2684;
    pub const SCI_INDICGETFLAGS: c_uint = 2685;
    pub const SCI_INDICSETSTROKEWIDTH: c_uint = 2751;
    pub const SCI_INDICGETSTROKEWIDTH: c_uint = 2752;
    pub const SCI_SETWHITESPACEFORE: c_uint = 2084;
    pub const SCI_SETWHITESPACEBACK: c_uint = 2085;
    pub const SCI_SETWHITESPACESIZE: c_uint = 2086;
    pub const SCI_GETWHITESPACESIZE: c_uint = 2087;
    pub const SCI_SETLINESTATE: c_uint = 2092;
    pub const SCI_GETLINESTATE: c_uint = 2093;
    pub const SCI_GETMAXLINESTATE: c_uint = 2094;
    pub const SCI_GETCARETLINEVISIBLE: c_uint = 2095;
    pub const SCI_SETCARETLINEVISIBLE: c_uint = 2096;
    pub const SCI_GETCARETLINEBACK: c_uint = 2097;
    pub const SCI_SETCARETLINEBACK: c_uint = 2098;
    pub const SCI_GETCARETLINEFRAME: c_uint = 2704;
    pub const SCI_SETCARETLINEFRAME: c_uint = 2705;
    pub const SCI_STYLESETCHANGEABLE: c_uint = 2099;
    pub const SCI_AUTOCSHOW: c_uint = 2100;
    pub const SCI_AUTOCCANCEL: c_uint = 2101;
    pub const SCI_AUTOCACTIVE: c_uint = 2102;
    pub const SCI_AUTOCPOSSTART: c_uint = 2103;
    pub const SCI_AUTOCCOMPLETE: c_uint = 2104;
    pub const SCI_AUTOCSTOPS: c_uint = 2105;
    pub const SCI_AUTOCSETSEPARATOR: c_uint = 2106;
    pub const SCI_AUTOCGETSEPARATOR: c_uint = 2107;
    pub const SCI_AUTOCSELECT: c_uint = 2108;
    pub const SCI_AUTOCSETCANCELATSTART: c_uint = 2110;
    pub const SCI_AUTOCGETCANCELATSTART: c_uint = 2111;
    pub const SCI_AUTOCSETFILLUPS: c_uint = 2112;
    pub const SCI_AUTOCSETCHOOSESINGLE: c_uint = 2113;
    pub const SCI_AUTOCGETCHOOSESINGLE: c_uint = 2114;
    pub const SCI_AUTOCSETIGNORECASE: c_uint = 2115;
    pub const SCI_AUTOCGETIGNORECASE: c_uint = 2116;
    pub const SCI_USERLISTSHOW: c_uint = 2117;
    pub const SCI_AUTOCSETAUTOHIDE: c_uint = 2118;
    pub const SCI_AUTOCGETAUTOHIDE: c_uint = 2119;
    pub const SC_AUTOCOMPLETE_NORMAL: c_uint = 0;
    pub const SC_AUTOCOMPLETE_FIXED_SIZE: c_uint = 1;
    pub const SCI_AUTOCSETOPTIONS: c_uint = 2638;
    pub const SCI_AUTOCGETOPTIONS: c_uint = 2639;
    pub const SCI_AUTOCSETDROPRESTOFWORD: c_uint = 2270;
    pub const SCI_AUTOCGETDROPRESTOFWORD: c_uint = 2271;
    pub const SCI_REGISTERIMAGE: c_uint = 2405;
    pub const SCI_CLEARREGISTEREDIMAGES: c_uint = 2408;
    pub const SCI_AUTOCGETTYPESEPARATOR: c_uint = 2285;
    pub const SCI_AUTOCSETTYPESEPARATOR: c_uint = 2286;
    pub const SCI_AUTOCSETMAXWIDTH: c_uint = 2208;
    pub const SCI_AUTOCGETMAXWIDTH: c_uint = 2209;
    pub const SCI_AUTOCSETMAXHEIGHT: c_uint = 2210;
    pub const SCI_AUTOCGETMAXHEIGHT: c_uint = 2211;
    pub const SCI_SETINDENT: c_uint = 2122;
    pub const SCI_GETINDENT: c_uint = 2123;
    pub const SCI_SETUSETABS: c_uint = 2124;
    pub const SCI_GETUSETABS: c_uint = 2125;
    pub const SCI_SETLINEINDENTATION: c_uint = 2126;
    pub const SCI_GETLINEINDENTATION: c_uint = 2127;
    pub const SCI_GETLINEINDENTPOSITION: c_uint = 2128;
    pub const SCI_GETCOLUMN: c_uint = 2129;
    pub const SCI_COUNTCHARACTERS: c_uint = 2633;
    pub const SCI_COUNTCODEUNITS: c_uint = 2715;
    pub const SCI_SETHSCROLLBAR: c_uint = 2130;
    pub const SCI_GETHSCROLLBAR: c_uint = 2131;
    pub const SC_IV_NONE: c_uint = 0;
    pub const SC_IV_REAL: c_uint = 1;
    pub const SC_IV_LOOKFORWARD: c_uint = 2;
    pub const SC_IV_LOOKBOTH: c_uint = 3;
    pub const SCI_SETINDENTATIONGUIDES: c_uint = 2132;
    pub const SCI_GETINDENTATIONGUIDES: c_uint = 2133;
    pub const SCI_SETHIGHLIGHTGUIDE: c_uint = 2134;
    pub const SCI_GETHIGHLIGHTGUIDE: c_uint = 2135;
    pub const SCI_GETLINEENDPOSITION: c_uint = 2136;
    pub const SCI_GETCODEPAGE: c_uint = 2137;
    pub const SCI_GETCARETFORE: c_uint = 2138;
    pub const SCI_GETREADONLY: c_uint = 2140;
    pub const SCI_SETCURRENTPOS: c_uint = 2141;
    pub const SCI_SETSELECTIONSTART: c_uint = 2142;
    pub const SCI_GETSELECTIONSTART: c_uint = 2143;
    pub const SCI_SETSELECTIONEND: c_uint = 2144;
    pub const SCI_GETSELECTIONEND: c_uint = 2145;
    pub const SCI_SETEMPTYSELECTION: c_uint = 2556;
    pub const SCI_SETPRINTMAGNIFICATION: c_uint = 2146;
    pub const SCI_GETPRINTMAGNIFICATION: c_uint = 2147;
    pub const SC_PRINT_NORMAL: c_uint = 0;
    pub const SC_PRINT_INVERTLIGHT: c_uint = 1;
    pub const SC_PRINT_BLACKONWHITE: c_uint = 2;
    pub const SC_PRINT_COLOURONWHITE: c_uint = 3;
    pub const SC_PRINT_COLOURONWHITEDEFAULTBG: c_uint = 4;
    pub const SC_PRINT_SCREENCOLOURS: c_uint = 5;
    pub const SCI_SETPRINTCOLOURMODE: c_uint = 2148;
    pub const SCI_GETPRINTCOLOURMODE: c_uint = 2149;
    pub const SCFIND_NONE: c_uint = 0x0;
    pub const SCFIND_WHOLEWORD: c_uint = 0x2;
    pub const SCFIND_MATCHCASE: c_uint = 0x4;
    pub const SCFIND_WORDSTART: c_uint = 0x00100000;
    pub const SCFIND_REGEXP: c_uint = 0x00200000;
    pub const SCFIND_POSIX: c_uint = 0x00400000;
    pub const SCFIND_CXX11REGEX: c_uint = 0x00800000;

    // Deprecated by Notepad++ 2GB+ support via new scintilla interfaces from 5.2.3 (see https://www.scintilla.org/ScintillaHistory.html)
    // Please use Sci_Position, SCI_GETTEXTRANGEFULL, SCI_FINDTEXTFULL, and SCI_FORMATRANGEFULL and corresponding defines/structs
    // const  SCI_FINDTEXT 2150

    pub const SCI_FINDTEXTFULL: c_uint = 2196;

    // Deprecated by Notepad++ 2GB+ support via new scintilla interfaces from 5.2.3 (see https://www.scintilla.org/ScintillaHistory.html)
    // Please use Sci_Position, SCI_GETTEXTRANGEFULL, SCI_FINDTEXTFULL, and SCI_FORMATRANGEFULL and corresponding defines/structs
    // const  SCI_FORMATRANGE 2151

    pub const SCI_FORMATRANGEFULL: c_uint = 2777;
    pub const SC_CHANGE_HISTORY_DISABLED: c_uint = 0;
    pub const SC_CHANGE_HISTORY_ENABLED: c_uint = 1;
    pub const SC_CHANGE_HISTORY_MARKERS: c_uint = 2;
    pub const SC_CHANGE_HISTORY_INDICATORS: c_uint = 4;
    pub const SCI_SETCHANGEHISTORY: c_uint = 2780;
    pub const SCI_GETCHANGEHISTORY: c_uint = 2781;
    pub const SCI_GETFIRSTVISIBLELINE: c_uint = 2152;
    pub const SCI_GETLINE: c_uint = 2153;
    pub const SCI_GETLINECOUNT: c_uint = 2154;
    pub const SCI_ALLOCATELINES: c_uint = 2089;
    pub const SCI_SETMARGINLEFT: c_uint = 2155;
    pub const SCI_GETMARGINLEFT: c_uint = 2156;
    pub const SCI_SETMARGINRIGHT: c_uint = 2157;
    pub const SCI_GETMARGINRIGHT: c_uint = 2158;
    pub const SCI_GETMODIFY: c_uint = 2159;
    pub const SCI_SETSEL: c_uint = 2160;
    pub const SCI_GETSELTEXT: c_uint = 2161;

    // Deprecated by Notepad++ 2GB+ support via new scintilla interfaces from 5.2.3 (see https://www.scintilla.org/ScintillaHistory.html)
    // Please use Sci_Position, SCI_GETTEXTRANGEFULL, SCI_FINDTEXTFULL, and SCI_FORMATRANGEFULL and corresponding defines/structs
    // const  SCI_GETTEXTRANGE 2162

    pub const SCI_GETTEXTRANGEFULL: c_uint = 2039;
    pub const SCI_HIDESELECTION: c_uint = 2163;
    pub const SCI_GETSELECTIONHIDDEN: c_uint = 2088;
    pub const SCI_POINTXFROMPOSITION: c_uint = 2164;
    pub const SCI_POINTYFROMPOSITION: c_uint = 2165;
    pub const SCI_LINEFROMPOSITION: c_uint = 2166;
    pub const SCI_POSITIONFROMLINE: c_uint = 2167;
    pub const SCI_LINESCROLL: c_uint = 2168;
    pub const SCI_SCROLLCARET: c_uint = 2169;
    pub const SCI_SCROLLRANGE: c_uint = 2569;
    pub const SCI_REPLACESEL: c_uint = 2170;
    pub const SCI_SETREADONLY: c_uint = 2171;
    pub const SCI_NULL: c_uint = 2172;
    pub const SCI_CANPASTE: c_uint = 2173;
    pub const SCI_CANUNDO: c_uint = 2174;
    pub const SCI_EMPTYUNDOBUFFER: c_uint = 2175;
    pub const SCI_UNDO: c_uint = 2176;
    pub const SCI_CUT: c_uint = 2177;
    pub const SCI_COPY: c_uint = 2178;
    pub const SCI_PASTE: c_uint = 2179;
    pub const SCI_CLEAR: c_uint = 2180;
    pub const SCI_SETTEXT: c_uint = 2181;
    pub const SCI_GETTEXT: c_uint = 2182;
    pub const SCI_GETTEXTLENGTH: c_uint = 2183;
    pub const SCI_GETDIRECTFUNCTION: c_uint = 2184;
    pub const SCI_GETDIRECTSTATUSFUNCTION: c_uint = 2772;
    pub const SCI_GETDIRECTPOINTER: c_uint = 2185;
    pub const SCI_SETOVERTYPE: c_uint = 2186;
    pub const SCI_GETOVERTYPE: c_uint = 2187;
    pub const SCI_SETCARETWIDTH: c_uint = 2188;
    pub const SCI_GETCARETWIDTH: c_uint = 2189;
    pub const SCI_SETTARGETSTART: c_uint = 2190;
    pub const SCI_GETTARGETSTART: c_uint = 2191;
    pub const SCI_SETTARGETSTARTVIRTUALSPACE: c_uint = 2728;
    pub const SCI_GETTARGETSTARTVIRTUALSPACE: c_uint = 2729;
    pub const SCI_SETTARGETEND: c_uint = 2192;
    pub const SCI_GETTARGETEND: c_uint = 2193;
    pub const SCI_SETTARGETENDVIRTUALSPACE: c_uint = 2730;
    pub const SCI_GETTARGETENDVIRTUALSPACE: c_uint = 2731;
    pub const SCI_SETTARGETRANGE: c_uint = 2686;
    pub const SCI_GETTARGETTEXT: c_uint = 2687;
    pub const SCI_TARGETFROMSELECTION: c_uint = 2287;
    pub const SCI_TARGETWHOLEDOCUMENT: c_uint = 2690;
    pub const SCI_REPLACETARGET: c_uint = 2194;
    pub const SCI_REPLACETARGETRE: c_uint = 2195;
    pub const SCI_SEARCHINTARGET: c_uint = 2197;
    pub const SCI_SETSEARCHFLAGS: c_uint = 2198;
    pub const SCI_GETSEARCHFLAGS: c_uint = 2199;
    pub const SCI_CALLTIPSHOW: c_uint = 2200;
    pub const SCI_CALLTIPCANCEL: c_uint = 2201;
    pub const SCI_CALLTIPACTIVE: c_uint = 2202;
    pub const SCI_CALLTIPPOSSTART: c_uint = 2203;
    pub const SCI_CALLTIPSETPOSSTART: c_uint = 2214;
    pub const SCI_CALLTIPSETHLT: c_uint = 2204;
    pub const SCI_CALLTIPSETBACK: c_uint = 2205;
    pub const SCI_CALLTIPSETFORE: c_uint = 2206;
    pub const SCI_CALLTIPSETFOREHLT: c_uint = 2207;
    pub const SCI_CALLTIPUSESTYLE: c_uint = 2212;
    pub const SCI_CALLTIPSETPOSITION: c_uint = 2213;
    pub const SCI_VISIBLEFROMDOCLINE: c_uint = 2220;
    pub const SCI_DOCLINEFROMVISIBLE: c_uint = 2221;
    pub const SCI_WRAPCOUNT: c_uint = 2235;
    pub const SC_FOLDLEVELNONE: c_uint = 0x0;
    pub const SC_FOLDLEVELBASE: c_uint = 0x400;
    pub const SC_FOLDLEVELWHITEFLAG: c_uint = 0x1000;
    pub const SC_FOLDLEVELHEADERFLAG: c_uint = 0x2000;
    pub const SC_FOLDLEVELNUMBERMASK: c_uint = 0x0FFF;
    pub const SCI_SETFOLDLEVEL: c_uint = 2222;
    pub const SCI_GETFOLDLEVEL: c_uint = 2223;
    pub const SCI_GETLASTCHILD: c_uint = 2224;
    pub const SCI_GETFOLDPARENT: c_uint = 2225;
    pub const SCI_SHOWLINES: c_uint = 2226;
    pub const SCI_HIDELINES: c_uint = 2227;
    pub const SCI_GETLINEVISIBLE: c_uint = 2228;
    pub const SCI_GETALLLINESVISIBLE: c_uint = 2236;
    pub const SCI_SETFOLDEXPANDED: c_uint = 2229;
    pub const SCI_GETFOLDEXPANDED: c_uint = 2230;
    pub const SCI_TOGGLEFOLD: c_uint = 2231;
    pub const SCI_TOGGLEFOLDSHOWTEXT: c_uint = 2700;
    pub const SC_FOLDDISPLAYTEXT_HIDDEN: c_uint = 0;
    pub const SC_FOLDDISPLAYTEXT_STANDARD: c_uint = 1;
    pub const SC_FOLDDISPLAYTEXT_BOXED: c_uint = 2;
    pub const SCI_FOLDDISPLAYTEXTSETSTYLE: c_uint = 2701;
    pub const SCI_FOLDDISPLAYTEXTGETSTYLE: c_uint = 2707;
    pub const SCI_SETDEFAULTFOLDDISPLAYTEXT: c_uint = 2722;
    pub const SCI_GETDEFAULTFOLDDISPLAYTEXT: c_uint = 2723;
    pub const SC_FOLDACTION_CONTRACT: c_uint = 0;
    pub const SC_FOLDACTION_EXPAND: c_uint = 1;
    pub const SC_FOLDACTION_TOGGLE: c_uint = 2;
    pub const SC_FOLDACTION_CONTRACT_EVERY_LEVEL: c_uint = 4;
    pub const SCI_FOLDLINE: c_uint = 2237;
    pub const SCI_FOLDCHILDREN: c_uint = 2238;
    pub const SCI_EXPANDCHILDREN: c_uint = 2239;
    pub const SCI_FOLDALL: c_uint = 2662;
    pub const SCI_ENSUREVISIBLE: c_uint = 2232;
    pub const SC_AUTOMATICFOLD_NONE: c_uint = 0x0000;
    pub const SC_AUTOMATICFOLD_SHOW: c_uint = 0x0001;
    pub const SC_AUTOMATICFOLD_CLICK: c_uint = 0x0002;
    pub const SC_AUTOMATICFOLD_CHANGE: c_uint = 0x0004;
    pub const SCI_SETAUTOMATICFOLD: c_uint = 2663;
    pub const SCI_GETAUTOMATICFOLD: c_uint = 2664;
    pub const SC_FOLDFLAG_NONE: c_uint = 0x0000;
    pub const SC_FOLDFLAG_LINEBEFORE_EXPANDED: c_uint = 0x0002;
    pub const SC_FOLDFLAG_LINEBEFORE_CONTRACTED: c_uint = 0x0004;
    pub const SC_FOLDFLAG_LINEAFTER_EXPANDED: c_uint = 0x0008;
    pub const SC_FOLDFLAG_LINEAFTER_CONTRACTED: c_uint = 0x0010;
    pub const SC_FOLDFLAG_LEVELNUMBERS: c_uint = 0x0040;
    pub const SC_FOLDFLAG_LINESTATE: c_uint = 0x0080;
    pub const SCI_SETFOLDFLAGS: c_uint = 2233;
    pub const SCI_ENSUREVISIBLEENFORCEPOLICY: c_uint = 2234;
    pub const SCI_SETTABINDENTS: c_uint = 2260;
    pub const SCI_GETTABINDENTS: c_uint = 2261;
    pub const SCI_SETBACKSPACEUNINDENTS: c_uint = 2262;
    pub const SCI_GETBACKSPACEUNINDENTS: c_uint = 2263;
    pub const SC_TIME_FOREVER: c_uint = 10000000;
    pub const SCI_SETMOUSEDWELLTIME: c_uint = 2264;
    pub const SCI_GETMOUSEDWELLTIME: c_uint = 2265;
    pub const SCI_WORDSTARTPOSITION: c_uint = 2266;
    pub const SCI_WORDENDPOSITION: c_uint = 2267;
    pub const SCI_ISRANGEWORD: c_uint = 2691;
    pub const SC_IDLESTYLING_NONE: c_uint = 0;
    pub const SC_IDLESTYLING_TOVISIBLE: c_uint = 1;
    pub const SC_IDLESTYLING_AFTERVISIBLE: c_uint = 2;
    pub const SC_IDLESTYLING_ALL: c_uint = 3;
    pub const SCI_SETIDLESTYLING: c_uint = 2692;
    pub const SCI_GETIDLESTYLING: c_uint = 2693;
    pub const SC_WRAP_NONE: c_uint = 0;
    pub const SC_WRAP_WORD: c_uint = 1;
    pub const SC_WRAP_CHAR: c_uint = 2;
    pub const SC_WRAP_WHITESPACE: c_uint = 3;
    pub const SCI_SETWRAPMODE: c_uint = 2268;
    pub const SCI_GETWRAPMODE: c_uint = 2269;
    pub const SC_WRAPVISUALFLAG_NONE: c_uint = 0x0000;
    pub const SC_WRAPVISUALFLAG_END: c_uint = 0x0001;
    pub const SC_WRAPVISUALFLAG_START: c_uint = 0x0002;
    pub const SC_WRAPVISUALFLAG_MARGIN: c_uint = 0x0004;
    pub const SCI_SETWRAPVISUALFLAGS: c_uint = 2460;
    pub const SCI_GETWRAPVISUALFLAGS: c_uint = 2461;
    pub const SC_WRAPVISUALFLAGLOC_DEFAULT: c_uint = 0x0000;
    pub const SC_WRAPVISUALFLAGLOC_END_BY_TEXT: c_uint = 0x0001;
    pub const SC_WRAPVISUALFLAGLOC_START_BY_TEXT: c_uint = 0x0002;
    pub const SCI_SETWRAPVISUALFLAGSLOCATION: c_uint = 2462;
    pub const SCI_GETWRAPVISUALFLAGSLOCATION: c_uint = 2463;
    pub const SCI_SETWRAPSTARTINDENT: c_uint = 2464;
    pub const SCI_GETWRAPSTARTINDENT: c_uint = 2465;
    pub const SC_WRAPINDENT_FIXED: c_uint = 0;
    pub const SC_WRAPINDENT_SAME: c_uint = 1;
    pub const SC_WRAPINDENT_INDENT: c_uint = 2;
    pub const SC_WRAPINDENT_DEEPINDENT: c_uint = 3;
    pub const SCI_SETWRAPINDENTMODE: c_uint = 2472;
    pub const SCI_GETWRAPINDENTMODE: c_uint = 2473;
    pub const SC_CACHE_NONE: c_uint = 0;
    pub const SC_CACHE_CARET: c_uint = 1;
    pub const SC_CACHE_PAGE: c_uint = 2;
    pub const SC_CACHE_DOCUMENT: c_uint = 3;
    pub const SCI_SETLAYOUTCACHE: c_uint = 2272;
    pub const SCI_GETLAYOUTCACHE: c_uint = 2273;
    pub const SCI_SETSCROLLWIDTH: c_uint = 2274;
    pub const SCI_GETSCROLLWIDTH: c_uint = 2275;
    pub const SCI_SETSCROLLWIDTHTRACKING: c_uint = 2516;
    pub const SCI_GETSCROLLWIDTHTRACKING: c_uint = 2517;
    pub const SCI_TEXTWIDTH: c_uint = 2276;
    pub const SCI_SETENDATLASTLINE: c_uint = 2277;
    pub const SCI_GETENDATLASTLINE: c_uint = 2278;
    pub const SCI_TEXTHEIGHT: c_uint = 2279;
    pub const SCI_SETVSCROLLBAR: c_uint = 2280;
    pub const SCI_GETVSCROLLBAR: c_uint = 2281;
    pub const SCI_APPENDTEXT: c_uint = 2282;
    pub const SC_PHASES_ONE: c_uint = 0;
    pub const SC_PHASES_TWO: c_uint = 1;
    pub const SC_PHASES_MULTIPLE: c_uint = 2;
    pub const SCI_GETPHASESDRAW: c_uint = 2673;
    pub const SCI_SETPHASESDRAW: c_uint = 2674;
    pub const SC_EFF_QUALITY_MASK: c_uint = 0xF;
    pub const SC_EFF_QUALITY_DEFAULT: c_uint = 0;
    pub const SC_EFF_QUALITY_NON_ANTIALIASED: c_uint = 1;
    pub const SC_EFF_QUALITY_ANTIALIASED: c_uint = 2;
    pub const SC_EFF_QUALITY_LCD_OPTIMIZED: c_uint = 3;
    pub const SCI_SETFONTQUALITY: c_uint = 2611;
    pub const SCI_GETFONTQUALITY: c_uint = 2612;
    pub const SCI_SETFIRSTVISIBLELINE: c_uint = 2613;
    pub const SC_MULTIPASTE_ONCE: c_uint = 0;
    pub const SC_MULTIPASTE_EACH: c_uint = 1;
    pub const SCI_SETMULTIPASTE: c_uint = 2614;
    pub const SCI_GETMULTIPASTE: c_uint = 2615;
    pub const SCI_GETTAG: c_uint = 2616;
    pub const SCI_LINESJOIN: c_uint = 2288;
    pub const SCI_LINESSPLIT: c_uint = 2289;
    pub const SCI_SETFOLDMARGINCOLOUR: c_uint = 2290;
    pub const SCI_SETFOLDMARGINHICOLOUR: c_uint = 2291;
    pub const SC_ACCESSIBILITY_DISABLED: c_uint = 0;
    pub const SC_ACCESSIBILITY_ENABLED: c_uint = 1;
    pub const SCI_SETACCESSIBILITY: c_uint = 2702;
    pub const SCI_GETACCESSIBILITY: c_uint = 2703;
    pub const SCI_LINEDOWN: c_uint = 2300;
    pub const SCI_LINEDOWNEXTEND: c_uint = 2301;
    pub const SCI_LINEUP: c_uint = 2302;
    pub const SCI_LINEUPEXTEND: c_uint = 2303;
    pub const SCI_CHARLEFT: c_uint = 2304;
    pub const SCI_CHARLEFTEXTEND: c_uint = 2305;
    pub const SCI_CHARRIGHT: c_uint = 2306;
    pub const SCI_CHARRIGHTEXTEND: c_uint = 2307;
    pub const SCI_WORDLEFT: c_uint = 2308;
    pub const SCI_WORDLEFTEXTEND: c_uint = 2309;
    pub const SCI_WORDRIGHT: c_uint = 2310;
    pub const SCI_WORDRIGHTEXTEND: c_uint = 2311;
    pub const SCI_HOME: c_uint = 2312;
    pub const SCI_HOMEEXTEND: c_uint = 2313;
    pub const SCI_LINEEND: c_uint = 2314;
    pub const SCI_LINEENDEXTEND: c_uint = 2315;
    pub const SCI_DOCUMENTSTART: c_uint = 2316;
    pub const SCI_DOCUMENTSTARTEXTEND: c_uint = 2317;
    pub const SCI_DOCUMENTEND: c_uint = 2318;
    pub const SCI_DOCUMENTENDEXTEND: c_uint = 2319;
    pub const SCI_PAGEUP: c_uint = 2320;
    pub const SCI_PAGEUPEXTEND: c_uint = 2321;
    pub const SCI_PAGEDOWN: c_uint = 2322;
    pub const SCI_PAGEDOWNEXTEND: c_uint = 2323;
    pub const SCI_EDITTOGGLEOVERTYPE: c_uint = 2324;
    pub const SCI_CANCEL: c_uint = 2325;
    pub const SCI_DELETEBACK: c_uint = 2326;
    pub const SCI_TAB: c_uint = 2327;
    pub const SCI_BACKTAB: c_uint = 2328;
    pub const SCI_NEWLINE: c_uint = 2329;
    pub const SCI_FORMFEED: c_uint = 2330;
    pub const SCI_VCHOME: c_uint = 2331;
    pub const SCI_VCHOMEEXTEND: c_uint = 2332;
    pub const SCI_ZOOMIN: c_uint = 2333;
    pub const SCI_ZOOMOUT: c_uint = 2334;
    pub const SCI_DELWORDLEFT: c_uint = 2335;
    pub const SCI_DELWORDRIGHT: c_uint = 2336;
    pub const SCI_DELWORDRIGHTEND: c_uint = 2518;
    pub const SCI_LINECUT: c_uint = 2337;
    pub const SCI_LINEDELETE: c_uint = 2338;
    pub const SCI_LINETRANSPOSE: c_uint = 2339;
    pub const SCI_LINEREVERSE: c_uint = 2354;
    pub const SCI_LINEDUPLICATE: c_uint = 2404;
    pub const SCI_LOWERCASE: c_uint = 2340;
    pub const SCI_UPPERCASE: c_uint = 2341;
    pub const SCI_LINESCROLLDOWN: c_uint = 2342;
    pub const SCI_LINESCROLLUP: c_uint = 2343;
    pub const SCI_DELETEBACKNOTLINE: c_uint = 2344;
    pub const SCI_HOMEDISPLAY: c_uint = 2345;
    pub const SCI_HOMEDISPLAYEXTEND: c_uint = 2346;
    pub const SCI_LINEENDDISPLAY: c_uint = 2347;
    pub const SCI_LINEENDDISPLAYEXTEND: c_uint = 2348;
    pub const SCI_HOMEWRAP: c_uint = 2349;
    pub const SCI_HOMEWRAPEXTEND: c_uint = 2450;
    pub const SCI_LINEENDWRAP: c_uint = 2451;
    pub const SCI_LINEENDWRAPEXTEND: c_uint = 2452;
    pub const SCI_VCHOMEWRAP: c_uint = 2453;
    pub const SCI_VCHOMEWRAPEXTEND: c_uint = 2454;
    pub const SCI_LINECOPY: c_uint = 2455;
    pub const SCI_MOVECARETINSIDEVIEW: c_uint = 2401;
    pub const SCI_LINELENGTH: c_uint = 2350;
    pub const SCI_BRACEHIGHLIGHT: c_uint = 2351;
    pub const SCI_BRACEHIGHLIGHTINDICATOR: c_uint = 2498;
    pub const SCI_BRACEBADLIGHT: c_uint = 2352;
    pub const SCI_BRACEBADLIGHTINDICATOR: c_uint = 2499;
    pub const SCI_BRACEMATCH: c_uint = 2353;
    pub const SCI_BRACEMATCHNEXT: c_uint = 2369;
    pub const SCI_GETVIEWEOL: c_uint = 2355;
    pub const SCI_SETVIEWEOL: c_uint = 2356;
    pub const SCI_GETDOCPOINTER: c_uint = 2357;
    pub const SCI_SETDOCPOINTER: c_uint = 2358;
    pub const SCI_SETMODEVENTMASK: c_uint = 2359;
    pub const EDGE_NONE: c_uint = 0;
    pub const EDGE_LINE: c_uint = 1;
    pub const EDGE_BACKGROUND: c_uint = 2;
    pub const EDGE_MULTILINE: c_uint = 3;
    pub const SCI_GETEDGECOLUMN: c_uint = 2360;
    pub const SCI_SETEDGECOLUMN: c_uint = 2361;
    pub const SCI_GETEDGEMODE: c_uint = 2362;
    pub const SCI_SETEDGEMODE: c_uint = 2363;
    pub const SCI_GETEDGECOLOUR: c_uint = 2364;
    pub const SCI_SETEDGECOLOUR: c_uint = 2365;
    pub const SCI_MULTIEDGEADDLINE: c_uint = 2694;
    pub const SCI_MULTIEDGECLEARALL: c_uint = 2695;
    pub const SCI_GETMULTIEDGECOLUMN: c_uint = 2749;
    pub const SCI_SEARCHANCHOR: c_uint = 2366;
    pub const SCI_SEARCHNEXT: c_uint = 2367;
    pub const SCI_SEARCHPREV: c_uint = 2368;
    pub const SCI_LINESONSCREEN: c_uint = 2370;
    pub const SC_POPUP_NEVER: c_uint = 0;
    pub const SC_POPUP_ALL: c_uint = 1;
    pub const SC_POPUP_TEXT: c_uint = 2;
    pub const SCI_USEPOPUP: c_uint = 2371;
    pub const SCI_SELECTIONISRECTANGLE: c_uint = 2372;
    pub const SCI_SETZOOM: c_uint = 2373;
    pub const SCI_GETZOOM: c_uint = 2374;
    pub const SC_DOCUMENTOPTION_DEFAULT: c_uint = 0;
    pub const SC_DOCUMENTOPTION_STYLES_NONE: c_uint = 0x1;
    pub const SC_DOCUMENTOPTION_TEXT_LARGE: c_uint = 0x100;
    pub const SCI_CREATEDOCUMENT: c_uint = 2375;
    pub const SCI_ADDREFDOCUMENT: c_uint = 2376;
    pub const SCI_RELEASEDOCUMENT: c_uint = 2377;
    pub const SCI_GETDOCUMENTOPTIONS: c_uint = 2379;
    pub const SCI_GETMODEVENTMASK: c_uint = 2378;
    pub const SCI_SETCOMMANDEVENTS: c_uint = 2717;
    pub const SCI_GETCOMMANDEVENTS: c_uint = 2718;
    pub const SCI_SETFOCUS: c_uint = 2380;
    pub const SCI_GETFOCUS: c_uint = 2381;
    pub const SC_STATUS_OK: c_uint = 0;
    pub const SC_STATUS_FAILURE: c_uint = 1;
    pub const SC_STATUS_BADALLOC: c_uint = 2;
    pub const SC_STATUS_WARN_START: c_uint = 1000;
    pub const SC_STATUS_WARN_REGEX: c_uint = 1001;
    pub const SCI_SETSTATUS: c_uint = 2382;
    pub const SCI_GETSTATUS: c_uint = 2383;
    pub const SCI_SETMOUSEDOWNCAPTURES: c_uint = 2384;
    pub const SCI_GETMOUSEDOWNCAPTURES: c_uint = 2385;
    pub const SCI_SETMOUSEWHEELCAPTURES: c_uint = 2696;
    pub const SCI_GETMOUSEWHEELCAPTURES: c_uint = 2697;
    pub const SCI_SETCURSOR: c_uint = 2386;
    pub const SCI_GETCURSOR: c_uint = 2387;
    pub const SCI_SETCONTROLCHARSYMBOL: c_uint = 2388;
    pub const SCI_GETCONTROLCHARSYMBOL: c_uint = 2389;
    pub const SCI_WORDPARTLEFT: c_uint = 2390;
    pub const SCI_WORDPARTLEFTEXTEND: c_uint = 2391;
    pub const SCI_WORDPARTRIGHT: c_uint = 2392;
    pub const SCI_WORDPARTRIGHTEXTEND: c_uint = 2393;
    pub const VISIBLE_SLOP: c_uint = 0x01;
    pub const VISIBLE_STRICT: c_uint = 0x04;
    pub const SCI_SETVISIBLEPOLICY: c_uint = 2394;
    pub const SCI_DELLINELEFT: c_uint = 2395;
    pub const SCI_DELLINERIGHT: c_uint = 2396;
    pub const SCI_SETXOFFSET: c_uint = 2397;
    pub const SCI_GETXOFFSET: c_uint = 2398;
    pub const SCI_CHOOSECARETX: c_uint = 2399;
    pub const SCI_GRABFOCUS: c_uint = 2400;
    pub const CARET_SLOP: c_uint = 0x01;
    pub const CARET_STRICT: c_uint = 0x04;
    pub const CARET_JUMPS: c_uint = 0x10;
    pub const CARET_EVEN: c_uint = 0x08;
    pub const SCI_SETXCARETPOLICY: c_uint = 2402;
    pub const SCI_SETYCARETPOLICY: c_uint = 2403;
    pub const SCI_SETPRINTWRAPMODE: c_uint = 2406;
    pub const SCI_GETPRINTWRAPMODE: c_uint = 2407;
    pub const SCI_SETHOTSPOTACTIVEFORE: c_uint = 2410;
    pub const SCI_GETHOTSPOTACTIVEFORE: c_uint = 2494;
    pub const SCI_SETHOTSPOTACTIVEBACK: c_uint = 2411;
    pub const SCI_GETHOTSPOTACTIVEBACK: c_uint = 2495;
    pub const SCI_SETHOTSPOTACTIVEUNDERLINE: c_uint = 2412;
    pub const SCI_GETHOTSPOTACTIVEUNDERLINE: c_uint = 2496;
    pub const SCI_SETHOTSPOTSINGLELINE: c_uint = 2421;
    pub const SCI_GETHOTSPOTSINGLELINE: c_uint = 2497;
    pub const SCI_PARADOWN: c_uint = 2413;
    pub const SCI_PARADOWNEXTEND: c_uint = 2414;
    pub const SCI_PARAUP: c_uint = 2415;
    pub const SCI_PARAUPEXTEND: c_uint = 2416;
    pub const SCI_POSITIONBEFORE: c_uint = 2417;
    pub const SCI_POSITIONAFTER: c_uint = 2418;
    pub const SCI_POSITIONRELATIVE: c_uint = 2670;
    pub const SCI_POSITIONRELATIVECODEUNITS: c_uint = 2716;
    pub const SCI_COPYRANGE: c_uint = 2419;
    pub const SCI_COPYTEXT: c_uint = 2420;
    pub const SC_SEL_STREAM: c_uint = 0;
    pub const SC_SEL_RECTANGLE: c_uint = 1;
    pub const SC_SEL_LINES: c_uint = 2;
    pub const SC_SEL_THIN: c_uint = 3;
    pub const SCI_SETSELECTIONMODE: c_uint = 2422;
    pub const SCI_GETSELECTIONMODE: c_uint = 2423;
    pub const SCI_GETMOVEEXTENDSSELECTION: c_uint = 2706;
    pub const SCI_GETLINESELSTARTPOSITION: c_uint = 2424;
    pub const SCI_GETLINESELENDPOSITION: c_uint = 2425;
    pub const SCI_LINEDOWNRECTEXTEND: c_uint = 2426;
    pub const SCI_LINEUPRECTEXTEND: c_uint = 2427;
    pub const SCI_CHARLEFTRECTEXTEND: c_uint = 2428;
    pub const SCI_CHARRIGHTRECTEXTEND: c_uint = 2429;
    pub const SCI_HOMERECTEXTEND: c_uint = 2430;
    pub const SCI_VCHOMERECTEXTEND: c_uint = 2431;
    pub const SCI_LINEENDRECTEXTEND: c_uint = 2432;
    pub const SCI_PAGEUPRECTEXTEND: c_uint = 2433;
    pub const SCI_PAGEDOWNRECTEXTEND: c_uint = 2434;
    pub const SCI_STUTTEREDPAGEUP: c_uint = 2435;
    pub const SCI_STUTTEREDPAGEUPEXTEND: c_uint = 2436;
    pub const SCI_STUTTEREDPAGEDOWN: c_uint = 2437;
    pub const SCI_STUTTEREDPAGEDOWNEXTEND: c_uint = 2438;
    pub const SCI_WORDLEFTEND: c_uint = 2439;
    pub const SCI_WORDLEFTENDEXTEND: c_uint = 2440;
    pub const SCI_WORDRIGHTEND: c_uint = 2441;
    pub const SCI_WORDRIGHTENDEXTEND: c_uint = 2442;
    pub const SCI_SETWHITESPACECHARS: c_uint = 2443;
    pub const SCI_GETWHITESPACECHARS: c_uint = 2647;
    pub const SCI_SETPUNCTUATIONCHARS: c_uint = 2648;
    pub const SCI_GETPUNCTUATIONCHARS: c_uint = 2649;
    pub const SCI_SETCHARSDEFAULT: c_uint = 2444;
    pub const SCI_AUTOCGETCURRENT: c_uint = 2445;
    pub const SCI_AUTOCGETCURRENTTEXT: c_uint = 2610;
    pub const SC_CASEINSENSITIVEBEHAVIOUR_RESPECTCASE: c_uint = 0;
    pub const SC_CASEINSENSITIVEBEHAVIOUR_IGNORECASE: c_uint = 1;
    pub const SCI_AUTOCSETCASEINSENSITIVEBEHAVIOUR: c_uint = 2634;
    pub const SCI_AUTOCGETCASEINSENSITIVEBEHAVIOUR: c_uint = 2635;
    pub const SC_MULTIAUTOC_ONCE: c_uint = 0;
    pub const SC_MULTIAUTOC_EACH: c_uint = 1;
    pub const SCI_AUTOCSETMULTI: c_uint = 2636;
    pub const SCI_AUTOCGETMULTI: c_uint = 2637;
    pub const SC_ORDER_PRESORTED: c_uint = 0;
    pub const SC_ORDER_PERFORMSORT: c_uint = 1;
    pub const SC_ORDER_CUSTOM: c_uint = 2;
    pub const SCI_AUTOCSETORDER: c_uint = 2660;
    pub const SCI_AUTOCGETORDER: c_uint = 2661;
    pub const SCI_ALLOCATE: c_uint = 2446;
    pub const SCI_TARGETASUTF8: c_uint = 2447;
    pub const SCI_SETLENGTHFORENCODE: c_uint = 2448;
    pub const SCI_ENCODEDFROMUTF8: c_uint = 2449;
    pub const SCI_FINDCOLUMN: c_uint = 2456;
    pub const SC_CARETSTICKY_OFF: c_uint = 0;
    pub const SC_CARETSTICKY_ON: c_uint = 1;
    pub const SC_CARETSTICKY_WHITESPACE: c_uint = 2;
    pub const SCI_GETCARETSTICKY: c_uint = 2457;
    pub const SCI_SETCARETSTICKY: c_uint = 2458;
    pub const SCI_TOGGLECARETSTICKY: c_uint = 2459;
    pub const SCI_SETPASTECONVERTENDINGS: c_uint = 2467;
    pub const SCI_GETPASTECONVERTENDINGS: c_uint = 2468;
    pub const SCI_REPLACERECTANGULAR: c_uint = 2771;
    pub const SCI_SELECTIONDUPLICATE: c_uint = 2469;
    pub const SCI_SETCARETLINEBACKALPHA: c_uint = 2470;
    pub const SCI_GETCARETLINEBACKALPHA: c_uint = 2471;
    pub const CARETSTYLE_INVISIBLE: c_uint = 0;
    pub const CARETSTYLE_LINE: c_uint = 1;
    pub const CARETSTYLE_BLOCK: c_uint = 2;
    pub const CARETSTYLE_OVERSTRIKE_BAR: c_uint = 0;
    pub const CARETSTYLE_OVERSTRIKE_BLOCK: c_uint = 0x10;
    pub const CARETSTYLE_CURSES: c_uint = 0x20;
    pub const CARETSTYLE_INS_MASK: c_uint = 0xF;
    pub const CARETSTYLE_BLOCK_AFTER: c_uint = 0x100;
    pub const SCI_SETCARETSTYLE: c_uint = 2512;
    pub const SCI_GETCARETSTYLE: c_uint = 2513;
    pub const SCI_SETINDICATORCURRENT: c_uint = 2500;
    pub const SCI_GETINDICATORCURRENT: c_uint = 2501;
    pub const SCI_SETINDICATORVALUE: c_uint = 2502;
    pub const SCI_GETINDICATORVALUE: c_uint = 2503;
    pub const SCI_INDICATORFILLRANGE: c_uint = 2504;
    pub const SCI_INDICATORCLEARRANGE: c_uint = 2505;
    pub const SCI_INDICATORALLONFOR: c_uint = 2506;
    pub const SCI_INDICATORVALUEAT: c_uint = 2507;
    pub const SCI_INDICATORSTART: c_uint = 2508;
    pub const SCI_INDICATOREND: c_uint = 2509;
    pub const SCI_SETPOSITIONCACHE: c_uint = 2514;
    pub const SCI_GETPOSITIONCACHE: c_uint = 2515;
    pub const SCI_SETLAYOUTTHREADS: c_uint = 2775;
    pub const SCI_GETLAYOUTTHREADS: c_uint = 2776;
    pub const SCI_COPYALLOWLINE: c_uint = 2519;
    pub const SCI_GETCHARACTERPOINTER: c_uint = 2520;
    pub const SCI_GETRANGEPOINTER: c_uint = 2643;
    pub const SCI_GETGAPPOSITION: c_uint = 2644;
    pub const SCI_INDICSETALPHA: c_uint = 2523;
    pub const SCI_INDICGETALPHA: c_uint = 2524;
    pub const SCI_INDICSETOUTLINEALPHA: c_uint = 2558;
    pub const SCI_INDICGETOUTLINEALPHA: c_uint = 2559;
    pub const SCI_SETEXTRAASCENT: c_uint = 2525;
    pub const SCI_GETEXTRAASCENT: c_uint = 2526;
    pub const SCI_SETEXTRADESCENT: c_uint = 2527;
    pub const SCI_GETEXTRADESCENT: c_uint = 2528;
    pub const SCI_MARKERSYMBOLDEFINED: c_uint = 2529;
    pub const SCI_MARGINSETTEXT: c_uint = 2530;
    pub const SCI_MARGINGETTEXT: c_uint = 2531;
    pub const SCI_MARGINSETSTYLE: c_uint = 2532;
    pub const SCI_MARGINGETSTYLE: c_uint = 2533;
    pub const SCI_MARGINSETSTYLES: c_uint = 2534;
    pub const SCI_MARGINGETSTYLES: c_uint = 2535;
    pub const SCI_MARGINTEXTCLEARALL: c_uint = 2536;
    pub const SCI_MARGINSETSTYLEOFFSET: c_uint = 2537;
    pub const SCI_MARGINGETSTYLEOFFSET: c_uint = 2538;
    pub const SC_MARGINOPTION_NONE: c_uint = 0;
    pub const SC_MARGINOPTION_SUBLINESELECT: c_uint = 1;
    pub const SCI_SETMARGINOPTIONS: c_uint = 2539;
    pub const SCI_GETMARGINOPTIONS: c_uint = 2557;
    pub const SCI_ANNOTATIONSETTEXT: c_uint = 2540;
    pub const SCI_ANNOTATIONGETTEXT: c_uint = 2541;
    pub const SCI_ANNOTATIONSETSTYLE: c_uint = 2542;
    pub const SCI_ANNOTATIONGETSTYLE: c_uint = 2543;
    pub const SCI_ANNOTATIONSETSTYLES: c_uint = 2544;
    pub const SCI_ANNOTATIONGETSTYLES: c_uint = 2545;
    pub const SCI_ANNOTATIONGETLINES: c_uint = 2546;
    pub const SCI_ANNOTATIONCLEARALL: c_uint = 2547;
    pub const ANNOTATION_HIDDEN: c_uint = 0;
    pub const ANNOTATION_STANDARD: c_uint = 1;
    pub const ANNOTATION_BOXED: c_uint = 2;
    pub const ANNOTATION_INDENTED: c_uint = 3;
    pub const SCI_ANNOTATIONSETVISIBLE: c_uint = 2548;
    pub const SCI_ANNOTATIONGETVISIBLE: c_uint = 2549;
    pub const SCI_ANNOTATIONSETSTYLEOFFSET: c_uint = 2550;
    pub const SCI_ANNOTATIONGETSTYLEOFFSET: c_uint = 2551;
    pub const SCI_RELEASEALLEXTENDEDSTYLES: c_uint = 2552;
    pub const SCI_ALLOCATEEXTENDEDSTYLES: c_uint = 2553;
    pub const UNDO_NONE: c_uint = 0;
    pub const UNDO_MAY_COALESCE: c_uint = 1;
    pub const SCI_ADDUNDOACTION: c_uint = 2560;
    pub const SCI_CHARPOSITIONFROMPOINT: c_uint = 2561;
    pub const SCI_CHARPOSITIONFROMPOINTCLOSE: c_uint = 2562;
    pub const SCI_SETMOUSESELECTIONRECTANGULARSWITCH: c_uint = 2668;
    pub const SCI_GETMOUSESELECTIONRECTANGULARSWITCH: c_uint = 2669;
    pub const SCI_SETMULTIPLESELECTION: c_uint = 2563;
    pub const SCI_GETMULTIPLESELECTION: c_uint = 2564;
    pub const SCI_SETADDITIONALSELECTIONTYPING: c_uint = 2565;
    pub const SCI_GETADDITIONALSELECTIONTYPING: c_uint = 2566;
    pub const SCI_SETADDITIONALCARETSBLINK: c_uint = 2567;
    pub const SCI_GETADDITIONALCARETSBLINK: c_uint = 2568;
    pub const SCI_SETADDITIONALCARETSVISIBLE: c_uint = 2608;
    pub const SCI_GETADDITIONALCARETSVISIBLE: c_uint = 2609;
    pub const SCI_GETSELECTIONS: c_uint = 2570;
    pub const SCI_GETSELECTIONEMPTY: c_uint = 2650;
    pub const SCI_CLEARSELECTIONS: c_uint = 2571;
    pub const SCI_SETSELECTION: c_uint = 2572;
    pub const SCI_ADDSELECTION: c_uint = 2573;
    pub const SCI_DROPSELECTIONN: c_uint = 2671;
    pub const SCI_SETMAINSELECTION: c_uint = 2574;
    pub const SCI_GETMAINSELECTION: c_uint = 2575;
    pub const SCI_SETSELECTIONNCARET: c_uint = 2576;
    pub const SCI_GETSELECTIONNCARET: c_uint = 2577;
    pub const SCI_SETSELECTIONNANCHOR: c_uint = 2578;
    pub const SCI_GETSELECTIONNANCHOR: c_uint = 2579;
    pub const SCI_SETSELECTIONNCARETVIRTUALSPACE: c_uint = 2580;
    pub const SCI_GETSELECTIONNCARETVIRTUALSPACE: c_uint = 2581;
    pub const SCI_SETSELECTIONNANCHORVIRTUALSPACE: c_uint = 2582;
    pub const SCI_GETSELECTIONNANCHORVIRTUALSPACE: c_uint = 2583;
    pub const SCI_SETSELECTIONNSTART: c_uint = 2584;
    pub const SCI_GETSELECTIONNSTART: c_uint = 2585;
    pub const SCI_GETSELECTIONNSTARTVIRTUALSPACE: c_uint = 2726;
    pub const SCI_SETSELECTIONNEND: c_uint = 2586;
    pub const SCI_GETSELECTIONNENDVIRTUALSPACE: c_uint = 2727;
    pub const SCI_GETSELECTIONNEND: c_uint = 2587;
    pub const SCI_SETRECTANGULARSELECTIONCARET: c_uint = 2588;
    pub const SCI_GETRECTANGULARSELECTIONCARET: c_uint = 2589;
    pub const SCI_SETRECTANGULARSELECTIONANCHOR: c_uint = 2590;
    pub const SCI_GETRECTANGULARSELECTIONANCHOR: c_uint = 2591;
    pub const SCI_SETRECTANGULARSELECTIONCARETVIRTUALSPACE: c_uint = 2592;
    pub const SCI_GETRECTANGULARSELECTIONCARETVIRTUALSPACE: c_uint = 2593;
    pub const SCI_SETRECTANGULARSELECTIONANCHORVIRTUALSPACE: c_uint = 2594;
    pub const SCI_GETRECTANGULARSELECTIONANCHORVIRTUALSPACE: c_uint = 2595;
    pub const SCVS_NONE: c_uint = 0;
    pub const SCVS_RECTANGULARSELECTION: c_uint = 1;
    pub const SCVS_USERACCESSIBLE: c_uint = 2;
    pub const SCVS_NOWRAPLINESTART: c_uint = 4;
    pub const SCI_SETVIRTUALSPACEOPTIONS: c_uint = 2596;
    pub const SCI_GETVIRTUALSPACEOPTIONS: c_uint = 2597;
    pub const SCI_SETRECTANGULARSELECTIONMODIFIER: c_uint = 2598;
    pub const SCI_GETRECTANGULARSELECTIONMODIFIER: c_uint = 2599;
    pub const SCI_SETADDITIONALSELFORE: c_uint = 2600;
    pub const SCI_SETADDITIONALSELBACK: c_uint = 2601;
    pub const SCI_SETADDITIONALSELALPHA: c_uint = 2602;
    pub const SCI_GETADDITIONALSELALPHA: c_uint = 2603;
    pub const SCI_SETADDITIONALCARETFORE: c_uint = 2604;
    pub const SCI_GETADDITIONALCARETFORE: c_uint = 2605;
    pub const SCI_ROTATESELECTION: c_uint = 2606;
    pub const SCI_SWAPMAINANCHORCARET: c_uint = 2607;
    pub const SCI_MULTIPLESELECTADDNEXT: c_uint = 2688;
    pub const SCI_MULTIPLESELECTADDEACH: c_uint = 2689;
    pub const SCI_CHANGELEXERSTATE: c_uint = 2617;
    pub const SCI_CONTRACTEDFOLDNEXT: c_uint = 2618;
    pub const SCI_VERTICALCENTRECARET: c_uint = 2619;
    pub const SCI_MOVESELECTEDLINESUP: c_uint = 2620;
    pub const SCI_MOVESELECTEDLINESDOWN: c_uint = 2621;
    pub const SCI_SETIDENTIFIER: c_uint = 2622;
    pub const SCI_GETIDENTIFIER: c_uint = 2623;
    pub const SCI_RGBAIMAGESETWIDTH: c_uint = 2624;
    pub const SCI_RGBAIMAGESETHEIGHT: c_uint = 2625;
    pub const SCI_RGBAIMAGESETSCALE: c_uint = 2651;
    pub const SCI_MARKERDEFINERGBAIMAGE: c_uint = 2626;
    pub const SCI_REGISTERRGBAIMAGE: c_uint = 2627;
    pub const SCI_SCROLLTOSTART: c_uint = 2628;
    pub const SCI_SCROLLTOEND: c_uint = 2629;
    pub const SC_TECHNOLOGY_DEFAULT: c_uint = 0;
    pub const SC_TECHNOLOGY_DIRECTWRITE: c_uint = 1;
    pub const SC_TECHNOLOGY_DIRECTWRITERETAIN: c_uint = 2;
    pub const SC_TECHNOLOGY_DIRECTWRITEDC: c_uint = 3;
    pub const SCI_SETTECHNOLOGY: c_uint = 2630;
    pub const SCI_GETTECHNOLOGY: c_uint = 2631;
    pub const SCI_CREATELOADER: c_uint = 2632;
    pub const SCI_FINDINDICATORSHOW: c_uint = 2640;
    pub const SCI_FINDINDICATORFLASH: c_uint = 2641;
    pub const SCI_FINDINDICATORHIDE: c_uint = 2642;
    pub const SCI_VCHOMEDISPLAY: c_uint = 2652;
    pub const SCI_VCHOMEDISPLAYEXTEND: c_uint = 2653;
    pub const SCI_GETCARETLINEVISIBLEALWAYS: c_uint = 2654;
    pub const SCI_SETCARETLINEVISIBLEALWAYS: c_uint = 2655;
    pub const SC_LINE_END_TYPE_DEFAULT: c_uint = 0;
    pub const SC_LINE_END_TYPE_UNICODE: c_uint = 1;
    pub const SCI_SETLINEENDTYPESALLOWED: c_uint = 2656;
    pub const SCI_GETLINEENDTYPESALLOWED: c_uint = 2657;
    pub const SCI_GETLINEENDTYPESACTIVE: c_uint = 2658;
    pub const SCI_SETREPRESENTATION: c_uint = 2665;
    pub const SCI_GETREPRESENTATION: c_uint = 2666;
    pub const SCI_CLEARREPRESENTATION: c_uint = 2667;
    pub const SCI_CLEARALLREPRESENTATIONS: c_uint = 2770;
    pub const SC_REPRESENTATION_PLAIN: c_uint = 0;
    pub const SC_REPRESENTATION_BLOB: c_uint = 1;
    pub const SC_REPRESENTATION_COLOUR: c_uint = 0x10;
    pub const SCI_SETREPRESENTATIONAPPEARANCE: c_uint = 2766;
    pub const SCI_GETREPRESENTATIONAPPEARANCE: c_uint = 2767;
    pub const SCI_SETREPRESENTATIONCOLOUR: c_uint = 2768;
    pub const SCI_GETREPRESENTATIONCOLOUR: c_uint = 2769;
    pub const SCI_EOLANNOTATIONSETTEXT: c_uint = 2740;
    pub const SCI_EOLANNOTATIONGETTEXT: c_uint = 2741;
    pub const SCI_EOLANNOTATIONSETSTYLE: c_uint = 2742;
    pub const SCI_EOLANNOTATIONGETSTYLE: c_uint = 2743;
    pub const SCI_EOLANNOTATIONCLEARALL: c_uint = 2744;
    pub const EOLANNOTATION_HIDDEN: c_uint = 0x0;
    pub const EOLANNOTATION_STANDARD: c_uint = 0x1;
    pub const EOLANNOTATION_BOXED: c_uint = 0x2;
    pub const EOLANNOTATION_STADIUM: c_uint = 0x100;
    pub const EOLANNOTATION_FLAT_CIRCLE: c_uint = 0x101;
    pub const EOLANNOTATION_ANGLE_CIRCLE: c_uint = 0x102;
    pub const EOLANNOTATION_CIRCLE_FLAT: c_uint = 0x110;
    pub const EOLANNOTATION_FLATS: c_uint = 0x111;
    pub const EOLANNOTATION_ANGLE_FLAT: c_uint = 0x112;
    pub const EOLANNOTATION_CIRCLE_ANGLE: c_uint = 0x120;
    pub const EOLANNOTATION_FLAT_ANGLE: c_uint = 0x121;
    pub const EOLANNOTATION_ANGLES: c_uint = 0x122;
    pub const SCI_EOLANNOTATIONSETVISIBLE: c_uint = 2745;
    pub const SCI_EOLANNOTATIONGETVISIBLE: c_uint = 2746;
    pub const SCI_EOLANNOTATIONSETSTYLEOFFSET: c_uint = 2747;
    pub const SCI_EOLANNOTATIONGETSTYLEOFFSET: c_uint = 2748;
    pub const SC_SUPPORTS_LINE_DRAWS_FINAL: c_uint = 0;
    pub const SC_SUPPORTS_PIXEL_DIVISIONS: c_uint = 1;
    pub const SC_SUPPORTS_FRACTIONAL_STROKE_WIDTH: c_uint = 2;
    pub const SC_SUPPORTS_TRANSLUCENT_STROKE: c_uint = 3;
    pub const SC_SUPPORTS_PIXEL_MODIFICATION: c_uint = 4;
    pub const SC_SUPPORTS_THREAD_SAFE_MEASURE_WIDTHS: c_uint = 5;
    pub const SCI_SUPPORTSFEATURE: c_uint = 2750;
    pub const SC_LINECHARACTERINDEX_NONE: c_uint = 0;
    pub const SC_LINECHARACTERINDEX_UTF32: c_uint = 1;
    pub const SC_LINECHARACTERINDEX_UTF16: c_uint = 2;
    pub const SCI_GETLINECHARACTERINDEX: c_uint = 2710;
    pub const SCI_ALLOCATELINECHARACTERINDEX: c_uint = 2711;
    pub const SCI_RELEASELINECHARACTERINDEX: c_uint = 2712;
    pub const SCI_LINEFROMINDEXPOSITION: c_uint = 2713;
    pub const SCI_INDEXPOSITIONFROMLINE: c_uint = 2714;
    pub const SCI_STARTRECORD: c_uint = 3001;
    pub const SCI_STOPRECORD: c_uint = 3002;
    pub const SCI_GETLEXER: c_uint = 4002;
    pub const SCI_COLOURISE: c_uint = 4003;
    pub const SCI_SETPROPERTY: c_uint = 4004;
    pub const KEYWORDSET_MAX: c_uint = 30;
    pub const SCI_SETKEYWORDS: c_uint = 4005;
    pub const SCI_GETPROPERTY: c_uint = 4008;
    pub const SCI_GETPROPERTYEXPANDED: c_uint = 4009;
    pub const SCI_GETPROPERTYINT: c_uint = 4010;
    pub const SCI_GETLEXERLANGUAGE: c_uint = 4012;
    pub const SCI_PRIVATELEXERCALL: c_uint = 4013;
    pub const SCI_PROPERTYNAMES: c_uint = 4014;
    pub const SC_TYPE_BOOLEAN: c_uint = 0;
    pub const SC_TYPE_INTEGER: c_uint = 1;
    pub const SC_TYPE_STRING: c_uint = 2;
    pub const SCI_PROPERTYTYPE: c_uint = 4015;
    pub const SCI_DESCRIBEPROPERTY: c_uint = 4016;
    pub const SCI_DESCRIBEKEYWORDSETS: c_uint = 4017;
    pub const SCI_GETLINEENDTYPESSUPPORTED: c_uint = 4018;
    pub const SCI_ALLOCATESUBSTYLES: c_uint = 4020;
    pub const SCI_GETSUBSTYLESSTART: c_uint = 4021;
    pub const SCI_GETSUBSTYLESLENGTH: c_uint = 4022;
    pub const SCI_GETSTYLEFROMSUBSTYLE: c_uint = 4027;
    pub const SCI_GETPRIMARYSTYLEFROMSTYLE: c_uint = 4028;
    pub const SCI_FREESUBSTYLES: c_uint = 4023;
    pub const SCI_SETIDENTIFIERS: c_uint = 4024;
    pub const SCI_DISTANCETOSECONDARYSTYLES: c_uint = 4025;
    pub const SCI_GETSUBSTYLEBASES: c_uint = 4026;
    pub const SCI_GETNAMEDSTYLES: c_uint = 4029;
    pub const SCI_NAMEOFSTYLE: c_uint = 4030;
    pub const SCI_TAGSOFSTYLE: c_uint = 4031;
    pub const SCI_DESCRIPTIONOFSTYLE: c_uint = 4032;
    pub const SCI_SETILEXER: c_uint = 4033;
    pub const SC_MOD_NONE: c_uint = 0x0;
    pub const SC_MOD_INSERTTEXT: c_uint = 0x1;
    pub const SC_MOD_DELETETEXT: c_uint = 0x2;
    pub const SC_MOD_CHANGESTYLE: c_uint = 0x4;
    pub const SC_MOD_CHANGEFOLD: c_uint = 0x8;
    pub const SC_PERFORMED_USER: c_uint = 0x10;
    pub const SC_PERFORMED_UNDO: c_uint = 0x20;
    pub const SC_PERFORMED_REDO: c_uint = 0x40;
    pub const SC_MULTISTEPUNDOREDO: c_uint = 0x80;
    pub const SC_LASTSTEPINUNDOREDO: c_uint = 0x100;
    pub const SC_MOD_CHANGEMARKER: c_uint = 0x200;
    pub const SC_MOD_BEFOREINSERT: c_uint = 0x400;
    pub const SC_MOD_BEFOREDELETE: c_uint = 0x800;
    pub const SC_MULTILINEUNDOREDO: c_uint = 0x1000;
    pub const SC_STARTACTION: c_uint = 0x2000;
    pub const SC_MOD_CHANGEINDICATOR: c_uint = 0x4000;
    pub const SC_MOD_CHANGELINESTATE: c_uint = 0x8000;
    pub const SC_MOD_CHANGEMARGIN: c_uint = 0x10000;
    pub const SC_MOD_CHANGEANNOTATION: c_uint = 0x20000;
    pub const SC_MOD_CONTAINER: c_uint = 0x40000;
    pub const SC_MOD_LEXERSTATE: c_uint = 0x80000;
    pub const SC_MOD_INSERTCHECK: c_uint = 0x100000;
    pub const SC_MOD_CHANGETABSTOPS: c_uint = 0x200000;
    pub const SC_MOD_CHANGEEOLANNOTATION: c_uint = 0x400000;
    pub const SC_MODEVENTMASKALL: c_uint = 0x7FFFFF;
    pub const SC_UPDATE_NONE: c_uint = 0x0;
    pub const SC_UPDATE_CONTENT: c_uint = 0x1;
    pub const SC_UPDATE_SELECTION: c_uint = 0x2;
    pub const SC_UPDATE_V_SCROLL: c_uint = 0x4;
    pub const SC_UPDATE_H_SCROLL: c_uint = 0x8;
    pub const SCEN_CHANGE: c_uint = 768;
    pub const SCEN_SETFOCUS: c_uint = 512;
    pub const SCEN_KILLFOCUS: c_uint = 256;
    pub const SCK_DOWN: c_uint = 300;
    pub const SCK_UP: c_uint = 301;
    pub const SCK_LEFT: c_uint = 302;
    pub const SCK_RIGHT: c_uint = 303;
    pub const SCK_HOME: c_uint = 304;
    pub const SCK_END: c_uint = 305;
    pub const SCK_PRIOR: c_uint = 306;
    pub const SCK_NEXT: c_uint = 307;
    pub const SCK_DELETE: c_uint = 308;
    pub const SCK_INSERT: c_uint = 309;
    pub const SCK_ESCAPE: c_uint = 7;
    pub const SCK_BACK: c_uint = 8;
    pub const SCK_TAB: c_uint = 9;
    pub const SCK_RETURN: c_uint = 13;
    pub const SCK_ADD: c_uint = 310;
    pub const SCK_SUBTRACT: c_uint = 311;
    pub const SCK_DIVIDE: c_uint = 312;
    pub const SCK_WIN: c_uint = 313;
    pub const SCK_RWIN: c_uint = 314;
    pub const SCK_MENU: c_uint = 315;
    pub const SCMOD_NORM: c_uint = 0;
    pub const SCMOD_SHIFT: c_uint = 1;
    pub const SCMOD_CTRL: c_uint = 2;
    pub const SCMOD_ALT: c_uint = 4;
    pub const SCMOD_SUPER: c_uint = 8;
    pub const SCMOD_META: c_uint = 16;
    pub const SC_AC_FILLUP: c_uint = 1;
    pub const SC_AC_DOUBLECLICK: c_uint = 2;
    pub const SC_AC_TAB: c_uint = 3;
    pub const SC_AC_NEWLINE: c_uint = 4;
    pub const SC_AC_COMMAND: c_uint = 5;
    pub const SC_CHARACTERSOURCE_DIRECT_INPUT: c_uint = 0;
    pub const SC_CHARACTERSOURCE_TENTATIVE_INPUT: c_uint = 1;
    pub const SC_CHARACTERSOURCE_IME_RESULT: c_uint = 2;
    pub const SCN_STYLENEEDED: c_uint = 2000;
    pub const SCN_CHARADDED: c_uint = 2001;
    pub const SCN_SAVEPOINTREACHED: c_uint = 2002;
    pub const SCN_SAVEPOINTLEFT: c_uint = 2003;
    pub const SCN_MODIFYATTEMPTRO: c_uint = 2004;
    pub const SCN_KEY: c_uint = 2005;
    pub const SCN_DOUBLECLICK: c_uint = 2006;
    pub const SCN_UPDATEUI: c_uint = 2007;
    pub const SCN_MODIFIED: c_uint = 2008;
    pub const SCN_MACRORECORD: c_uint = 2009;
    pub const SCN_MARGINCLICK: c_uint = 2010;
    pub const SCN_NEEDSHOWN: c_uint = 2011;
    pub const SCN_PAINTED: c_uint = 2013;
    pub const SCN_USERLISTSELECTION: c_uint = 2014;
    pub const SCN_URIDROPPED: c_uint = 2015;
    pub const SCN_DWELLSTART: c_uint = 2016;
    pub const SCN_DWELLEND: c_uint = 2017;
    pub const SCN_ZOOM: c_uint = 2018;
    pub const SCN_HOTSPOTCLICK: c_uint = 2019;
    pub const SCN_HOTSPOTDOUBLECLICK: c_uint = 2020;
    pub const SCN_CALLTIPCLICK: c_uint = 2021;
    pub const SCN_AUTOCSELECTION: c_uint = 2022;
    pub const SCN_INDICATORCLICK: c_uint = 2023;
    pub const SCN_INDICATORRELEASE: c_uint = 2024;
    pub const SCN_AUTOCCANCELLED: c_uint = 2025;
    pub const SCN_AUTOCCHARDELETED: c_uint = 2026;
    pub const SCN_HOTSPOTRELEASECLICK: c_uint = 2027;
    pub const SCN_FOCUSIN: c_uint = 2028;
    pub const SCN_FOCUSOUT: c_uint = 2029;
    pub const SCN_AUTOCCOMPLETED: c_uint = 2030;
    pub const SCN_MARGINRIGHTCLICK: c_uint = 2031;
    pub const SCN_AUTOCSELECTIONCHANGE: c_uint = 2032;

    pub const SC_BIDIRECTIONAL_DISABLED: c_uint = 0;
    pub const SC_BIDIRECTIONAL_L2R: c_uint = 1;
    pub const SC_BIDIRECTIONAL_R2L: c_uint = 2;
    pub const SCI_GETBIDIRECTIONAL: c_uint = 2708;
    pub const SCI_SETBIDIRECTIONAL: c_uint = 2709;

    pub const SC_SEARCHRESULT_LINEBUFFERMAXLENGTH: c_uint = 2048;
    pub const SCI_GETBOOSTREGEXERRMSG: c_uint = 5000;
    pub const SCN_FOLDINGSTATECHANGED: c_uint = 2081;

    /*
    /* These structures are defined to be exactly the same shape as the Win32
    * CHARRANGE, TEXTRANGE, FINDTEXTEX, FORMATRANGE, and NMHDR structs.
    * So older code that treats Scintilla as a RichEdit will work. */
    /*
    * Deprecated by Notepad++ 2GB+ support via new scintilla interfaces from 5.2.3 (see https://www.scintilla.org/ScintillaHistory.html)
    * Please use Sci_Position, SCI_GETTEXTRANGEFULL, SCI_FINDTEXTFULL, and SCI_FORMATRANGEFULL and corresponding defines/structs
    *
    struct Sci_CharacterRange {
        Sci_PositionCR cpMin;
        Sci_PositionCR cpMax;
    };
    */
    struct Sci_CharacterRangeFull {
        Sci_Position cpMin;
        Sci_Position cpMax;
    };
    /*
    * Deprecated by Notepad++ 2GB+ support via new scintilla interfaces from 5.2.3 (see https://www.scintilla.org/ScintillaHistory.html)
    * Please use Sci_Position, SCI_GETTEXTRANGEFULL, SCI_FINDTEXTFULL, and SCI_FORMATRANGEFULL and corresponding defines/structs
    *
    struct Sci_TextRange {
        struct Sci_CharacterRange chrg;
        char *lpstrText;
    };
    */
    struct Sci_TextRangeFull {
        struct Sci_CharacterRangeFull chrg;
        char *lpstrText;
    };
    /*
    * Deprecated by Notepad++ 2GB+ support via new scintilla interfaces from 5.2.3 (see https://www.scintilla.org/ScintillaHistory.html)
    * Please use Sci_Position, SCI_GETTEXTRANGEFULL, SCI_FINDTEXTFULL, and SCI_FORMATRANGEFULL and corresponding defines/structs
    *
    struct Sci_TextToFind {
        struct Sci_CharacterRange chrg;
        const char *lpstrText;
        struct Sci_CharacterRange chrgText;
    };
    */
    struct Sci_TextToFindFull {
        struct Sci_CharacterRangeFull chrg;
        const char *lpstrText;
        struct Sci_CharacterRangeFull chrgText;
    };

    typedef void *Sci_SurfaceID;

    struct Sci_Rectangle {
        int left;
        int top;
        int right;
        int bottom;
    };

    /* This structure is used in printing and requires some of the graphics types
    * from Platform.h.  Not needed by most client code. */
    /*
    * Deprecated by Notepad++ 2GB+ support via new scintilla interfaces from 5.2.3 (see https://www.scintilla.org/ScintillaHistory.html)
    * Please use Sci_Position, SCI_GETTEXTRANGEFULL, SCI_FINDTEXTFULL, and SCI_FORMATRANGEFULL and corresponding defines/structs
    *
    struct Sci_RangeToFormat {
        Sci_SurfaceID hdc;
        Sci_SurfaceID hdcTarget;
        struct Sci_Rectangle rc;
        struct Sci_Rectangle rcPage;
        struct Sci_CharacterRange chrg;
    };
    */
    struct Sci_RangeToFormatFull {
        Sci_SurfaceID hdc;
        Sci_SurfaceID hdcTarget;
        struct Sci_Rectangle rc;
        struct Sci_Rectangle rcPage;
        struct Sci_CharacterRangeFull chrg;
    };

    #ifndef __cplusplus
    /* For the GTK+ platform, g-ir-scanner needs to have these typedefs. This
    * is not required in C++ code and actually seems to break ScintillaEditPy */
    typedef struct Sci_NotifyHeader Sci_NotifyHeader;
    typedef struct SCNotification SCNotification;
    #endif

    struct Sci_NotifyHeader {
        /* Compatible with Windows NMHDR.
        * hwndFrom is really an environment specific window handle or pointer
        * but most clients of Scintilla.h do not have this type visible. */
        void *hwndFrom;
        uptr_t idFrom;
        unsigned int code;
    };

    struct SCNotification {
        Sci_NotifyHeader nmhdr;
        Sci_Position position;
        /* SCN_STYLENEEDED, SCN_DOUBLECLICK, SCN_MODIFIED, SCN_MARGINCLICK, */
        /* SCN_NEEDSHOWN, SCN_DWELLSTART, SCN_DWELLEND, SCN_CALLTIPCLICK, */
        /* SCN_HOTSPOTCLICK, SCN_HOTSPOTDOUBLECLICK, SCN_HOTSPOTRELEASECLICK, */
        /* SCN_INDICATORCLICK, SCN_INDICATORRELEASE, */
        /* SCN_USERLISTSELECTION, SCN_AUTOCSELECTION */

        int ch;
        /* SCN_CHARADDED, SCN_KEY, SCN_AUTOCCOMPLETED, SCN_AUTOCSELECTION, */
        /* SCN_USERLISTSELECTION */
        int modifiers;
        /* SCN_KEY, SCN_DOUBLECLICK, SCN_HOTSPOTCLICK, SCN_HOTSPOTDOUBLECLICK, */
        /* SCN_HOTSPOTRELEASECLICK, SCN_INDICATORCLICK, SCN_INDICATORRELEASE, */

        int modificationType;	/* SCN_MODIFIED */
        const char *text;
        /* SCN_MODIFIED, SCN_USERLISTSELECTION, SCN_AUTOCSELECTION, SCN_URIDROPPED */

        Sci_Position length;		/* SCN_MODIFIED */
        Sci_Position linesAdded;	/* SCN_MODIFIED */
        int message;	/* SCN_MACRORECORD */
        uptr_t wParam;	/* SCN_MACRORECORD */
        sptr_t lParam;	/* SCN_MACRORECORD */
        Sci_Position line;		/* SCN_MODIFIED */
        int foldLevelNow;	/* SCN_MODIFIED */
        int foldLevelPrev;	/* SCN_MODIFIED */
        int margin;		/* SCN_MARGINCLICK */
        int listType;	/* SCN_USERLISTSELECTION */
        int x;			/* SCN_DWELLSTART, SCN_DWELLEND */
        int y;		/* SCN_DWELLSTART, SCN_DWELLEND */
        int token;		/* SCN_MODIFIED with SC_MOD_CONTAINER */
        Sci_Position annotationLinesAdded;	/* SCN_MODIFIED with SC_MOD_CHANGEANNOTATION */
        int updated;	/* SCN_UPDATEUI */
        int listCompletionMethod;
        /* SCN_AUTOCSELECTION, SCN_AUTOCCOMPLETED, SCN_USERLISTSELECTION, */
        int characterSource;	/* SCN_CHARADDED */
    };

    #include <vector>
    struct SearchResultMarkingLine { // each line could have several segments if user want to see only 1 found line which contains several results
        std::vector<std::pair<intptr_t, intptr_t>> _segmentPostions; // a vector of pair of start & end of occurrence for colourizing
    };

    struct SearchResultMarkings {
        intptr_t _length;
        SearchResultMarkingLine *_markings;
    };*/
}
