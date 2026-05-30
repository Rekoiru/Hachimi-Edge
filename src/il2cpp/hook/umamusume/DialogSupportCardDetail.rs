use crate::il2cpp::{hook::UnityEngine_UI::Text, symbols::{get_field_from_name, get_field_object_value, get_method_addr}, types::*};

static mut STORYTEXT_FIELD: *mut FieldInfo = 0 as _;
fn get__storyText(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { STORYTEXT_FIELD })
}

fn fix_story_text(this: *mut Il2CppObject) {
    let story_text = get__storyText(this);
    if !story_text.is_null() {
        Text::set_supportRichText(story_text, true);
    }
}

type SetupFn = extern "C" fn(this: *mut Il2CppObject, p1: *mut Il2CppObject, p2: bool, p3: bool, p4: *mut Il2CppObject, p5: bool, p6: bool, p7: bool, p8: *mut Il2CppString, p9: *mut Il2CppObject, p10: *mut Il2CppObject);
extern "C" fn Setup(this: *mut Il2CppObject, p1: *mut Il2CppObject, p2: bool, p3: bool, p4: *mut Il2CppObject, p5: bool, p6: bool, p7: bool, p8: *mut Il2CppString, p9: *mut Il2CppObject, p10: *mut Il2CppObject) {
    get_orig_fn!(Setup, SetupFn)(this, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10);
    fix_story_text(this);
}

pub fn init(image: *const Il2CppImage) {
    get_class_or_return!(image, Gallop, DialogSupportCardDetail);

    let Setup_addr = get_method_addr(DialogSupportCardDetail, c"Setup", 10);
    if Setup_addr != 0 {
        new_hook!(Setup_addr, Setup);
    }

    unsafe {
        STORYTEXT_FIELD = get_field_from_name(DialogSupportCardDetail, c"_storyText");
    }
}
