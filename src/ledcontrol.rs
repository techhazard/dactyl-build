use core::intrinsics::volatile_store;

pub fn led_on() {
    unsafe {
        volatile_store(GPIOC_PDOR!(), 0x20);
    }
}

pub fn led_off() {
    unsafe {
        volatile_store(GPIOC_PDOR!(), 0x0);
    }
}
