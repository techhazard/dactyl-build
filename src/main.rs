#![feature(lang_items, core_intrinsics, asm, start, use_extern_macros)]
#![no_std]
#![crate_type = "staticlib"]

use core::intrinsics::volatile_store;

extern crate dactyl_firmware;
use dactyl_firmware as firmware;
use firmware::{ledcontrol, timecontrol};

//#[lang="eh_personality"]
//extern "C" fn eh_personality() {}
// #[lang="panic_fmt"]
// #[no_mangle]
// pub extern "C" fn rust_begin_unwind(_fmt: &core::fmt::Arguments,
//                                     _file_line: &(&'static str, usize))
//                                     -> ! {
//     loop {}
// }

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() -> () {
    loop {}
}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr1() -> () {
    loop {}
}

extern "C" {
    static mut _sflashdata: u32;
    static mut _sdata: u32;
    static mut _edata: u32;
    static mut _sbss: u32;
    static mut _ebss: u32;
    fn _estack();
}

#[link_section = ".vectors"]
#[allow(non_upper_case_globals)]
#[no_mangle]
pub static ISRVectors: [Option<unsafe extern "C" fn()>; 16] = [
    Some(_estack),        // Stack pointer
    Some(startup),        // Reset
    Some(isr_nmi),        // NMI
    Some(isr_hardfault),  // Hard Fault
    Some(isr_mmfault),    /* CM3 Memory Management Fault */
    Some(isr_busfault),   /* CM3 Bus Fault */
    Some(isr_usagefault), /* CM3 Usage Fault */
    Some(isr_reserved_1), /* Reserved - Used as NXP Checksum */
    None,                 // Reserved
    None,                 // Reserved
    None,                 // Reserved
    Some(isr_svcall),     // SVCall
    Some(isr_debugmon),   /* Reserved for debug */
    None,                 // Reserved
    Some(isr_pendsv),     // PendSV
    Some(isr_systick),    /* SysTick */
];

#[link_section = ".flashconfig"]
#[allow(non_upper_case_globals)]
#[no_mangle]
pub static flashconfigbytes: [usize; 4] = [0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFE];

pub unsafe extern "C" fn startup() {
    let mut src: *mut u32 = &mut _sflashdata;
    let mut dest: *mut u32 = &mut _sdata;

    volatile_store(firmware::WDOG_UNLOCK!(), 0xC520);
    volatile_store(firmware::WDOG_UNLOCK!(), 0xD928);
    volatile_store(firmware::WDOG_STCTRLH!(), 0x01D2);

    while dest < &mut _edata as *mut u32 {
        *dest = *src;
        dest = ((dest as u32) + 4) as *mut u32;
        src = ((src as u32) + 4) as *mut u32;
    }

    dest = &mut _sbss as *mut u32;

    while dest < &mut _edata as *mut u32 {
        *dest = 0;
        dest = ((dest as u32) + 4) as *mut u32;
    }

    // Enable system clock on all GPIO ports - page 254
    *firmware::GPIO_CONFIG!() = 0x00043F82; // 0b1000011111110000010
                                            // Configure the led pin
    *firmware::PORTC_PCR5!() = 0x00000143; // Enables GPIO | DSE | PULL_ENABLE | PULL_SELECT - page 227
                                           // Set the led pin to output
    *firmware::GPIOC_PDDR!() = 0x20; // pin 5 on port c

    rust_loop();
}
fn long() {
    let long_delay = timecontrol::Milliseconds(700);
    ledcontrol::led_on();
    timecontrol::rough_sleep(long_delay);
    ledcontrol::led_off();
    timecontrol::rough_sleep(long_delay);
}

fn short() {
    let short_delay = timecontrol::Milliseconds(300);
    ledcontrol::led_on();
    timecontrol::rough_sleep(short_delay);
    ledcontrol::led_off();
    timecontrol::rough_sleep(short_delay);
}
fn signal_start() {
    short();
    short();
    short();
    long();
}

fn rust_loop() {
    signal_start();
    firmware::run();
}

#[start]
fn lang_start(_: isize, _: *const *const u8) -> isize {
    unsafe {
        startup();
    }
    0
}

pub unsafe extern "C" fn isr_nmi() {
    loop {}
}
pub unsafe extern "C" fn isr_hardfault() {
    loop {}
}
pub unsafe extern "C" fn isr_mmfault() {
    loop {}
}
pub unsafe extern "C" fn isr_busfault() {
    loop {}
}
pub unsafe extern "C" fn isr_usagefault() {
    loop {}
}
pub unsafe extern "C" fn isr_reserved_1() {
    loop {}
}
pub unsafe extern "C" fn isr_svcall() {
    loop {}
}
pub unsafe extern "C" fn isr_debugmon() {
    loop {}
}
pub unsafe extern "C" fn isr_pendsv() {
    loop {}
}
pub unsafe extern "C" fn isr_systick() {
    loop {}
}
