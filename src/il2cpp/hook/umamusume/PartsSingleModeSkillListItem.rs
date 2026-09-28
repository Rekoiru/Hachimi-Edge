use crate::{
    core::{Hachimi, game::Region},
    il2cpp::{
        hook::UnityEngine_UI::Text,
        sql::TextDataQuery,
        symbols::{
            get_field_from_name,
            get_field_object_value,
            get_method_addr},
        types::*}
};

// SkillListItem
static mut NAMETEXT_FIELD: *mut FieldInfo = 0 as _;
pub fn get__nameText(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { NAMETEXT_FIELD })
}
static mut DESCTEXT_FIELD: *mut FieldInfo = 0 as _;
pub fn get__descText(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { DESCTEXT_FIELD })
}

// PartsSingleModeSkillListItem.Info
static mut get_IsDrawDesc_addr: usize = 0;
impl_addr_wrapper_fn!(get_IsDrawDesc, get_IsDrawDesc_addr, bool, this: *mut Il2CppObject);
static mut get_IsDrawNeedSkillPoint_addr: usize = 0;
impl_addr_wrapper_fn!(get_IsDrawNeedSkillPoint, get_IsDrawNeedSkillPoint_addr, bool, this: *mut Il2CppObject);

fn UpdateItemCommon(this: *mut Il2CppObject, skill_info: *mut Il2CppObject, orig_fn_cb: impl FnOnce()) {
    let name = get__nameText(this);
    let desc = get__descText(this);

    // Name should always exist, but let's be sure.
    if !name.is_null() {
        Text::set_horizontalOverflow(name, 0);
        Text::set_resizeTextForBestFit(name, true);
    }

    if get_IsDrawDesc(skill_info) && !desc.is_null() {
        Text::set_horizontalOverflow(desc, 0);
        Text::set_resizeTextForBestFit(desc, true);
        Text::set_resizeTextMinSize(desc, 14);
        Text::set_resizeTextMaxSize(desc, 30);
    }

    TextDataQuery::with_skill_learning_query(|| {
        orig_fn_cb();
    });

    if let Some(mult) = Hachimi::instance().localized_data.load().config.skill_list_item_desc_font_size_multiplier {
        let desc_text = get__descText(this);
        let font_size = Text::get_fontSize(desc_text);
        Text::set_fontSize(desc_text, (font_size as f32 * mult).round() as i32);
    }
}

type UpdateItemJpFn = extern "C" fn(this: *mut Il2CppObject, skill_info: *mut Il2CppObject, is_plate_effect_enable: bool, adjuster_data: *mut Il2CppObject, resource_hash: i32, on_click_button: *mut Il2CppObject);
extern "C" fn UpdateItemJp(this: *mut Il2CppObject, skill_info: *mut Il2CppObject, is_plate_effect_enable: bool, adjuster_data: *mut Il2CppObject, resource_hash: i32, on_click_button: *mut Il2CppObject) {
    UpdateItemCommon(this, skill_info, || {
        get_orig_fn!(UpdateItemJp, UpdateItemJpFn)(this, skill_info, is_plate_effect_enable, adjuster_data, resource_hash, on_click_button);
    });
}

type UpdateItemOtherFn = extern "C" fn(this: *mut Il2CppObject, skill_info: *mut Il2CppObject, is_plate_effect_enable: bool);
extern "C" fn UpdateItemOther(this: *mut Il2CppObject, skill_info: *mut Il2CppObject, is_plate_effect_enable: bool) {
    UpdateItemCommon(this, skill_info, || {
        get_orig_fn!(UpdateItemOther, UpdateItemOtherFn)(this, skill_info, is_plate_effect_enable);
    });
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, PartsSingleModeSkillListItem);
    find_nested_class_or_return!(PartsSingleModeSkillListItem, Info);

    if Hachimi::instance().game.region == Region::Japan {
        let UpdateItem_addr = get_method_addr(PartsSingleModeSkillListItem, c"UpdateItem", 5);
        new_hook!(UpdateItem_addr, UpdateItemJp);
    } else {
        let UpdateItem_addr = get_method_addr(PartsSingleModeSkillListItem, c"UpdateItem", 3);
        new_hook!(UpdateItem_addr, UpdateItemOther);
    }

    unsafe {
        // PartsSingleModeSkillListItem
        NAMETEXT_FIELD = get_field_from_name(PartsSingleModeSkillListItem, c"_nameText");
        DESCTEXT_FIELD = get_field_from_name(PartsSingleModeSkillListItem, c"_descText");

        // PartsSingleModeSkillListItem.Info
        get_IsDrawDesc_addr = get_method_addr(Info, c"get_IsDrawDesc", 0);
        get_IsDrawNeedSkillPoint_addr = get_method_addr(Info, c"get_IsDrawNeedSkillPoint", 0);
    }
}