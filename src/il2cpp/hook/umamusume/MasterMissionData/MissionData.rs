use crate::il2cpp::{symbols::get_method_addr, types::*};

type GetNameFn = extern "C" fn(this: *mut Il2CppObject) -> *mut Il2CppString;
extern "C" fn get_Name(this: *mut Il2CppObject) -> *mut Il2CppString {
    let text = get_orig_fn!(get_Name, GetNameFn)(this);
    text
}

pub fn init(MasterMissionData: *mut Il2CppClass) {
    find_nested_class_or_return!(MasterMissionData, MissionData);

    let get_Name_addr = get_method_addr(MissionData, c"get_Name", 0);

    new_hook!(get_Name_addr, get_Name);
}