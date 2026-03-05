use crate::il2cpp::{hook::UnityEngine_UI::Text, sql::{self, TextDataQuery}, symbols::{get_field_from_name, get_field_object_value, get_method_addr}, types::*};

static mut NAMETEXT_FIELD: *mut FieldInfo = 0 as _;
fn get__nameText(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { NAMETEXT_FIELD })
}
static mut DESCTEXT_FIELD: *mut FieldInfo = 0 as _;
fn get__descText(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { DESCTEXT_FIELD })
}

type UpdateCurrentFn = extern "C" fn(this: *mut Il2CppObject);
extern "C" fn UpdateCurrent(this: *mut Il2CppObject) {
    let name = get__nameText(this);
    let desc = get__descText(this);

    if !name.is_null() {
        Text::set_horizontalOverflow(name, 0);
        Text::set_resizeTextForBestFit(name, true);
    }
    if !desc.is_null() {
        Text::set_horizontalOverflow(desc, 0);
        Text::set_resizeTextForBestFit(desc, true);
        Text::set_resizeTextMinSize(desc, 14); // sensible default
        Text::set_resizeTextMaxSize(desc, 30);
    }

    sql::TextDataQuery::with_skill_learning_query(|| {
        get_orig_fn!(UpdateCurrent, UpdateCurrentFn)(this);
    });
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, PartsSingleModeSkillLearningListItem);

    let UpdateCurrent_addr = get_method_addr(PartsSingleModeSkillLearningListItem, c"UpdateCurrent", 0);

    new_hook!(UpdateCurrent_addr, UpdateCurrent);

    unsafe {
        NAMETEXT_FIELD = get_field_from_name(PartsSingleModeSkillLearningListItem, c"_nameText");
        DESCTEXT_FIELD = get_field_from_name(PartsSingleModeSkillLearningListItem, c"_descriptionText");
    }
}