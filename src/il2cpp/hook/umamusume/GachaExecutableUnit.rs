use crate::il2cpp::{symbols::get_method_addr, types::*};

static mut GET_DRAW_COUNT_ADDR: usize = 0;
static mut GET_IS_DAILY_ADDR: usize = 0;
static mut GET_IS_PAID_ADDR: usize = 0;
static mut GET_IS_FREE_ADDR: usize = 0;

fn call_method<T>(this: *mut Il2CppObject, addr: usize, default: T) -> T {
    if this.is_null() || addr == 0 {
        return default;
    }
    unsafe {
        let func: extern "C" fn(*mut Il2CppObject) -> T = std::mem::transmute(addr);
        func(this)
    }
}

pub fn get_DrawCount(this: *mut Il2CppObject) -> i32 {
    call_method(this, unsafe { GET_DRAW_COUNT_ADDR }, 0)
}

pub fn get_IsDaily(this: *mut Il2CppObject) -> bool {
    call_method(this, unsafe { GET_IS_DAILY_ADDR }, false)
}

pub fn get_IsPaid(this: *mut Il2CppObject) -> bool {
    call_method(this, unsafe { GET_IS_PAID_ADDR }, false)
}

pub fn get_IsFree(this: *mut Il2CppObject) -> bool {
    call_method(this, unsafe { GET_IS_FREE_ADDR }, false)
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