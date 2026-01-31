use crate::il2cpp::{api::il2cpp_field_get_value_object, symbols::get_field_from_name, types::*};

static mut CARD_ROOT_BUTTON_FIELD: *mut FieldInfo = std::ptr::null_mut();
static mut SUPPORT_CARD_ROOT_BUTTON_FIELD: *mut FieldInfo = std::ptr::null_mut();
static mut TRAINED_CHARA_ROOT_BUTTON_FIELD: *mut FieldInfo = std::ptr::null_mut();
static mut CHARACTER_CARD_CATALOG_BUTTON_FIELD: *mut FieldInfo = std::ptr::null_mut();
static mut CARD_LV_UP_BUTTON_FIELD: *mut FieldInfo = std::ptr::null_mut();
static mut HINT_LV_UP_BUTTON_FIELD: *mut FieldInfo = std::ptr::null_mut();
static mut CARD_LIMIT_BREAK_BUTTON_FIELD: *mut FieldInfo = std::ptr::null_mut();
static mut PIECE_EXCHANGE_BUTTON_FIELD: *mut FieldInfo = std::ptr::null_mut();
static mut SUPPORT_EDIT_BUTTON_FIELD: *mut FieldInfo = std::ptr::null_mut();
static mut SUPPORT_SELL_BUTTON_FIELD: *mut FieldInfo = std::ptr::null_mut();
static mut SUPPORT_LIST_BUTTON_FIELD: *mut FieldInfo = std::ptr::null_mut();
static mut TRAINED_LIST_BUTTON_FIELD: *mut FieldInfo = std::ptr::null_mut();
static mut NEW_TEAM_EDIT_BUTTON_FIELD: *mut FieldInfo = std::ptr::null_mut();
static mut TRANSFER_BUTTON_FIELD: *mut FieldInfo = std::ptr::null_mut();

// helper function for getting field values
fn get_field_value(this: *mut Il2CppObject, field: *mut FieldInfo) -> *mut Il2CppObject {
    if this.is_null() || field.is_null() {
        return std::ptr::null_mut();
    }
    unsafe { il2cpp_field_get_value_object(field, this) }
}

// public getter functions
pub fn get_cardRootButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_value(this, unsafe { CARD_ROOT_BUTTON_FIELD })
}

pub fn get_supportCardRootButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_value(this, unsafe { SUPPORT_CARD_ROOT_BUTTON_FIELD })
}

pub fn get_trainedCharaRootButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_value(this, unsafe { TRAINED_CHARA_ROOT_BUTTON_FIELD })
}

pub fn get_characterCardCatalogButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_value(this, unsafe { CHARACTER_CARD_CATALOG_BUTTON_FIELD })
}

pub fn get_cardLvUpButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_value(this, unsafe { CARD_LV_UP_BUTTON_FIELD })
}

pub fn get_hintLvUpButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_value(this, unsafe { HINT_LV_UP_BUTTON_FIELD })
}

pub fn get_cardLimitBreakButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_value(this, unsafe { CARD_LIMIT_BREAK_BUTTON_FIELD })
}

pub fn get_pieceExchangeButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_value(this, unsafe { PIECE_EXCHANGE_BUTTON_FIELD })
}

pub fn get_supportEditButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_value(this, unsafe { SUPPORT_EDIT_BUTTON_FIELD })
}

pub fn get_supportSellButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_value(this, unsafe { SUPPORT_SELL_BUTTON_FIELD })
}

pub fn get_supportListButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_value(this, unsafe { SUPPORT_LIST_BUTTON_FIELD })
}

pub fn get_trainedListButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_value(this, unsafe { TRAINED_LIST_BUTTON_FIELD })
}

pub fn get_newTeamEditButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_value(this, unsafe { NEW_TEAM_EDIT_BUTTON_FIELD })
}

pub fn get_transferButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_value(this, unsafe { TRANSFER_BUTTON_FIELD })
}

pub fn init(umamusume: *const Il2CppImage) {
    if let Ok(klass) = crate::il2cpp::symbols::get_class(umamusume, c"Gallop", c"CharacterHomeTopUI") {
        unsafe {
            CARD_ROOT_BUTTON_FIELD = get_field_from_name(klass, c"_cardRootButton");
            SUPPORT_CARD_ROOT_BUTTON_FIELD = get_field_from_name(klass, c"_supportCardRootButton");
            TRAINED_CHARA_ROOT_BUTTON_FIELD = get_field_from_name(klass, c"_trainedCharaRootButton");
            CHARACTER_CARD_CATALOG_BUTTON_FIELD = get_field_from_name(klass, c"_characterCardCatalogButton");
            CARD_LV_UP_BUTTON_FIELD = get_field_from_name(klass, c"_cardLvUpButton");
            HINT_LV_UP_BUTTON_FIELD = get_field_from_name(klass, c"_hintLvUpButton");
            CARD_LIMIT_BREAK_BUTTON_FIELD = get_field_from_name(klass, c"_cardLimitBreakButton");
            PIECE_EXCHANGE_BUTTON_FIELD = get_field_from_name(klass, c"_pieceExchangeButton");
            SUPPORT_EDIT_BUTTON_FIELD = get_field_from_name(klass, c"_supportEditButton");
            SUPPORT_SELL_BUTTON_FIELD = get_field_from_name(klass, c"_supportSellButton");
            SUPPORT_LIST_BUTTON_FIELD = get_field_from_name(klass, c"_supportListButton");
            TRAINED_LIST_BUTTON_FIELD = get_field_from_name(klass, c"_trainedListButton");
            NEW_TEAM_EDIT_BUTTON_FIELD = get_field_from_name(klass, c"_newTeamEditButton");
            TRANSFER_BUTTON_FIELD = get_field_from_name(klass, c"_transferButton");
        }
    }
}