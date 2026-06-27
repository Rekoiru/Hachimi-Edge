use crate::{
    core::{Hachimi, game::Region},
    il2cpp::{hook::UnityEngine_UI::Text, symbols::{get_field_from_name, get_field_object_value, get_method_addr}, types::*}
};

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

type SetupJpFn = extern "C" fn(this: *mut Il2CppObject, p1: *mut Il2CppObject, p2: bool, p3: bool, p4: *mut Il2CppObject, p5: bool, p6: bool, p7: bool, p8: *mut Il2CppString, p9: *mut Il2CppObject, p10: *mut Il2CppObject);
extern "C" fn SetupJp(this: *mut Il2CppObject, p1: *mut Il2CppObject, p2: bool, p3: bool, p4: *mut Il2CppObject, p5: bool, p6: bool, p7: bool, p8: *mut Il2CppString, p9: *mut Il2CppObject, p10: *mut Il2CppObject) {
    get_orig_fn!(SetupJp, SetupJpFn)(this, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10);
    fix_story_text(this);
}

type SetupOtherFn = extern "C" fn(this: *mut Il2CppObject, p1: *mut Il2CppObject, p2: bool, p3: bool, p4: *mut Il2CppObject, p5: bool, p6: bool);
extern "C" fn SetupOther(this: *mut Il2CppObject, p1: *mut Il2CppObject, p2: bool, p3: bool, p4: *mut Il2CppObject, p5: bool, p6: bool) {
    get_orig_fn!(SetupOther, SetupOtherFn)(this, p1, p2, p3, p4, p5, p6);
    fix_story_text(this);
}

pub fn init(image: *const Il2CppImage) {
    get_class_or_return!(image, Gallop, DialogSupportCardDetail);

    if Hachimi::instance().game.region == Region::Japan {
        let Setup_addr = get_method_addr(DialogSupportCardDetail, c"Setup", 10);
        new_hook!(Setup_addr, SetupJp);
    }
    else {
        let Setup_addr = get_method_addr(DialogSupportCardDetail, c"Setup", 6);
        new_hook!(Setup_addr, SetupOther);
    }

    unsafe {
        STORYTEXT_FIELD = get_field_from_name(DialogSupportCardDetail, c"_storyText");
    }
}
