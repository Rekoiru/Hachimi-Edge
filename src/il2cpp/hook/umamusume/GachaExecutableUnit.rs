use crate::il2cpp::{symbols::get_method_addr, types::*};

static mut GET_DRAW_COUNT_ADDR: usize = 0;
static mut GET_IS_DAILY_ADDR: usize = 0;
static mut GET_IS_PAID_ADDR: usize = 0;
static mut GET_IS_FREE_ADDR: usize = 0;

pub fn get_DrawCount(this: *mut Il2CppObject) -> i32 {
    if this.is_null() { return 0; }
    unsafe {
        if GET_DRAW_COUNT_ADDR == 0 { return 0; }
        let func: extern "C" fn(*mut Il2CppObject) -> i32 = std::mem::transmute(GET_DRAW_COUNT_ADDR);
        func(this)
    }
}

pub fn get_IsDaily(this: *mut Il2CppObject) -> bool {
    if this.is_null() { return false; }
    unsafe {
        if GET_IS_DAILY_ADDR == 0 { return false; }
        let func: extern "C" fn(*mut Il2CppObject) -> bool = std::mem::transmute(GET_IS_DAILY_ADDR);
        func(this)
    }
}

pub fn get_IsPaid(this: *mut Il2CppObject) -> bool {
    if this.is_null() { return false; }
    unsafe {
        if GET_IS_PAID_ADDR == 0 { return false; }
        let func: extern "C" fn(*mut Il2CppObject) -> bool = std::mem::transmute(GET_IS_PAID_ADDR);
        func(this)
    }
}

pub fn get_IsFree(this: *mut Il2CppObject) -> bool {
    if this.is_null() { return false; }
    unsafe {
        if GET_IS_FREE_ADDR == 0 { return false; }
        let func: extern "C" fn(*mut Il2CppObject) -> bool = std::mem::transmute(GET_IS_FREE_ADDR);
        func(this)
    }
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, GachaExecutableUnit);
    unsafe {
        GET_DRAW_COUNT_ADDR = get_method_addr(GachaExecutableUnit, c"get_DrawCount", 0);
        GET_IS_DAILY_ADDR = get_method_addr(GachaExecutableUnit, c"get_IsDaily", 0);
        GET_IS_PAID_ADDR = get_method_addr(GachaExecutableUnit, c"get_IsPaid", 0);
        GET_IS_FREE_ADDR = get_method_addr(GachaExecutableUnit, c"get_IsFree", 0);
    }
}
