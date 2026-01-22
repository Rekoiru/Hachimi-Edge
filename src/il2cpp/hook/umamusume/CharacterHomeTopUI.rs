use crate::il2cpp::{api::il2cpp_field_get_value_object, symbols::get_field_from_name, types::*};

static mut CARD_ROOT_BUTTON_FIELD: *mut FieldInfo = std::ptr::null_mut();
pub fn get_cardRootButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    unsafe { 
        if CARD_ROOT_BUTTON_FIELD.is_null() {
            error!("CARD_ROOT_BUTTON_FIELD is null!");
            return std::ptr::null_mut();
        }
        il2cpp_field_get_value_object(CARD_ROOT_BUTTON_FIELD, this) 
    }
}

static mut SUPPORT_CARD_ROOT_BUTTON_FIELD: *mut FieldInfo = std::ptr::null_mut();
pub fn get_supportCardRootButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    unsafe { 
        if SUPPORT_CARD_ROOT_BUTTON_FIELD.is_null() {
            error!("SUPPORT_CARD_ROOT_BUTTON_FIELD is null!");
            return std::ptr::null_mut();
        }
        il2cpp_field_get_value_object(SUPPORT_CARD_ROOT_BUTTON_FIELD, this) 
    }
}

static mut TRAINED_CHARA_ROOT_BUTTON_FIELD: *mut FieldInfo = std::ptr::null_mut();
pub fn get_trainedCharaRootButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    unsafe { 
        if TRAINED_CHARA_ROOT_BUTTON_FIELD.is_null() {
            error!("TRAINED_CHARA_ROOT_BUTTON_FIELD is null!");
            return std::ptr::null_mut();
        }
        il2cpp_field_get_value_object(TRAINED_CHARA_ROOT_BUTTON_FIELD, this) 
    }
}

static mut CHARACTER_CARD_CATALOG_BUTTON_FIELD: *mut FieldInfo = std::ptr::null_mut();
pub fn get_characterCardCatalogButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    unsafe { 
        if CHARACTER_CARD_CATALOG_BUTTON_FIELD.is_null() {
            error!("CHARACTER_CARD_CATALOG_BUTTON_FIELD is null!");
            return std::ptr::null_mut();
        }
        il2cpp_field_get_value_object(CHARACTER_CARD_CATALOG_BUTTON_FIELD, this) 
    }
}

static mut CARD_LV_UP_BUTTON_FIELD: *mut FieldInfo = std::ptr::null_mut();
pub fn get_cardLvUpButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    unsafe { 
        if CARD_LV_UP_BUTTON_FIELD.is_null() {
            error!("CARD_LV_UP_BUTTON_FIELD is null!");
            return std::ptr::null_mut();
        }
        il2cpp_field_get_value_object(CARD_LV_UP_BUTTON_FIELD, this) 
    }
}

static mut HINT_LV_UP_BUTTON_FIELD: *mut FieldInfo = std::ptr::null_mut();
pub fn get_hintLvUpButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    unsafe { 
        if HINT_LV_UP_BUTTON_FIELD.is_null() {
            error!("HINT_LV_UP_BUTTON_FIELD is null!");
            return std::ptr::null_mut();
        }
        il2cpp_field_get_value_object(HINT_LV_UP_BUTTON_FIELD, this) 
    }
}

static mut CARD_LIMIT_BREAK_BUTTON_FIELD: *mut FieldInfo = std::ptr::null_mut();
pub fn get_cardLimitBreakButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    unsafe { 
        if CARD_LIMIT_BREAK_BUTTON_FIELD.is_null() {
            error!("CARD_LIMIT_BREAK_BUTTON_FIELD is null!");
            return std::ptr::null_mut();
        }
        il2cpp_field_get_value_object(CARD_LIMIT_BREAK_BUTTON_FIELD, this) 
    }
}

static mut PIECE_EXCHANGE_BUTTON_FIELD: *mut FieldInfo = std::ptr::null_mut();
pub fn get_pieceExchangeButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    unsafe { 
        if PIECE_EXCHANGE_BUTTON_FIELD.is_null() {
            error!("PIECE_EXCHANGE_BUTTON_FIELD is null!");
            return std::ptr::null_mut();
        }
        il2cpp_field_get_value_object(PIECE_EXCHANGE_BUTTON_FIELD, this) 
    }
}

static mut SUPPORT_EDIT_BUTTON_FIELD: *mut FieldInfo = std::ptr::null_mut();
pub fn get_supportEditButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    unsafe { 
        if SUPPORT_EDIT_BUTTON_FIELD.is_null() {
            error!("SUPPORT_EDIT_BUTTON_FIELD is null!");
            return std::ptr::null_mut();
        }
        il2cpp_field_get_value_object(SUPPORT_EDIT_BUTTON_FIELD, this) 
    }
}

static mut SUPPORT_SELL_BUTTON_FIELD: *mut FieldInfo = std::ptr::null_mut();
pub fn get_supportSellButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    unsafe { 
        if SUPPORT_SELL_BUTTON_FIELD.is_null() {
            error!("SUPPORT_SELL_BUTTON_FIELD is null!");
            return std::ptr::null_mut();
        }
        il2cpp_field_get_value_object(SUPPORT_SELL_BUTTON_FIELD, this) 
    }
}

static mut SUPPORT_LIST_BUTTON_FIELD: *mut FieldInfo = std::ptr::null_mut();
pub fn get_supportListButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    unsafe { 
        if SUPPORT_LIST_BUTTON_FIELD.is_null() {
            error!("SUPPORT_LIST_BUTTON_FIELD is null!");
            return std::ptr::null_mut();
        }
        il2cpp_field_get_value_object(SUPPORT_LIST_BUTTON_FIELD, this) 
    }
}

static mut TRAINED_LIST_BUTTON_FIELD: *mut FieldInfo = std::ptr::null_mut();
pub fn get_trainedListButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    unsafe { 
        if TRAINED_LIST_BUTTON_FIELD.is_null() {
            error!("TRAINED_LIST_BUTTON_FIELD is null!");
            return std::ptr::null_mut();
        }
        il2cpp_field_get_value_object(TRAINED_LIST_BUTTON_FIELD, this) 
    }
}

static mut NEW_TEAM_EDIT_BUTTON_FIELD: *mut FieldInfo = std::ptr::null_mut();
pub fn get_newTeamEditButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    unsafe { 
        if NEW_TEAM_EDIT_BUTTON_FIELD.is_null() {
            error!("NEW_TEAM_EDIT_BUTTON_FIELD is null!");
            return std::ptr::null_mut();
        }
        il2cpp_field_get_value_object(NEW_TEAM_EDIT_BUTTON_FIELD, this) 
    }
}

static mut TRANSFER_BUTTON_FIELD: *mut FieldInfo = std::ptr::null_mut();
pub fn get_transferButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    unsafe { 
        if TRANSFER_BUTTON_FIELD.is_null() {
            error!("TRANSFER_BUTTON_FIELD is null!");
            return std::ptr::null_mut();
        }
        il2cpp_field_get_value_object(TRANSFER_BUTTON_FIELD, this) 
    }
}

pub fn init(umamusume: *const Il2CppImage) {
    if let Ok(klass) = crate::il2cpp::symbols::get_class(umamusume, c"Gallop", c"CharacterHomeTopUI") {
        unsafe {
            let ch = klass;
            CARD_ROOT_BUTTON_FIELD = get_field_from_name(ch, c"_cardRootButton");
            SUPPORT_CARD_ROOT_BUTTON_FIELD = get_field_from_name(ch, c"_supportCardRootButton");
            TRAINED_CHARA_ROOT_BUTTON_FIELD = get_field_from_name(ch, c"_trainedCharaRootButton");
            CHARACTER_CARD_CATALOG_BUTTON_FIELD = get_field_from_name(ch, c"_characterCardCatalogButton");
            CARD_LV_UP_BUTTON_FIELD = get_field_from_name(ch, c"_cardLvUpButton");
            HINT_LV_UP_BUTTON_FIELD = get_field_from_name(ch, c"_hintLvUpButton");
            CARD_LIMIT_BREAK_BUTTON_FIELD = get_field_from_name(ch, c"_cardLimitBreakButton");
            PIECE_EXCHANGE_BUTTON_FIELD = get_field_from_name(ch, c"_pieceExchangeButton");
            SUPPORT_EDIT_BUTTON_FIELD = get_field_from_name(ch, c"_supportEditButton");
            SUPPORT_SELL_BUTTON_FIELD = get_field_from_name(ch, c"_supportSellButton");
            SUPPORT_LIST_BUTTON_FIELD = get_field_from_name(ch, c"_supportListButton");
            TRAINED_LIST_BUTTON_FIELD = get_field_from_name(ch, c"_trainedListButton");
            NEW_TEAM_EDIT_BUTTON_FIELD = get_field_from_name(ch, c"_newTeamEditButton");
            TRANSFER_BUTTON_FIELD = get_field_from_name(ch, c"_transferButton");
        }
    }
}
