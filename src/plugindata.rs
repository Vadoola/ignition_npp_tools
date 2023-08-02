use crate::def::{function_item_text, FuncItem, NppData, ShortcutKey};
use crate::functions;

pub static mut NPP_DATA: Option<NppData> = None;

/*static mut SHORT_KEY_F10: ShortcutKey = ShortcutKey {
    _isCtrl: false,
    _isAlt: false,
    _isShift: false,
    _key: 121,
};

static mut SHORT_KEY_CTRL_F10: ShortcutKey = ShortcutKey {
    _isCtrl: true,
    _isAlt: false,
    _isShift: false,
    _key: 121,
};*/

pub fn FuncItem_MovePipes() -> FuncItem {
    FuncItem {
        _itemName: function_item_text("Move Pipes"),
        _pFunc: functions::move_objects,
        _cmdID: 0,
        _init2Check: false,
        //_pShKey: unsafe { &mut SHORT_KEY_F10 as *mut ShortcutKey as usize },
        _pShKey: 0,
    }
}

pub fn getNppHandle() -> &'static mut NppData {
    unsafe {
        match NPP_DATA {
            Some(ref mut x) => &mut *x,
            None => panic!(),
        }
    }
}
