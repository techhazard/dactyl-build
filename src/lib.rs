#![feature(asm,core_intrinsics)]
extern crate core;
use core::intrinsics::volatile_store;

#[macro_export]
macro_rules! GPIOC_PDOR   {() => (0x400FF080 as *mut u32);} // GPIOC_PDOR - page 1334,1335
#[macro_export]
macro_rules! WDOG_UNLOCK  {() => (0x4005200E as *mut u16);} // Watchdog Unlock register
#[macro_export]
macro_rules! WDOG_STCTRLH {() => (0x40052000 as *mut u16);} // Watchdog Status and Control Register High
#[macro_export]
macro_rules! GPIO_CONFIG  {() => (0x40048038 as *mut u32);}
#[macro_export]
macro_rules! PORTC_PCR5   {() => (0x4004B014 as *mut u32);} // PORTC_PCR5 - page 223/227
#[macro_export]
macro_rules! GPIOC_PDDR   {() => (0x400FF094 as *mut u32);} // GPIOC_PDDR - page 1334,1337
#[macro_export]
macro_rules! GPIOC_PDOR   {() => (0x400FF080 as *mut u32);} // GPIOC_PDOR - page 1334,1335

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

pub fn delay(ms: i32) {
    for _ in 0..ms * 250 {
        unsafe {
            asm!("NOP");
        }
    }
}


pub enum Key {
    Char(char),
    CapsLock,
    Esc,
    Tab,
    Enter,
    Return,
    Backspace,
    Menu, //rightmousethingy
    Function,
    Modifier,
    Layer(u32),
}

pub enum Side {
    Left,
    Right
}

pub enum Modifier {
    Alt(Side),
    Altgr(Side),
    Shift(Side),
    Ctrl(Side),
    Super(Side),
    Meta(Side),
    Hyper(Side),
}

pub struct KeyCombo {
    key: Key,
    modifiers: Vec<Modifier>,/* one of: shift ctrl alt altgr super meta hyper*/
}

/// Designates the position in the keygrid.
/// A grid can be at most 256 keys wide
/// and 256 keys tall
/// grid_id is the id of the grid the key is in,
/// e.g. 0 for alphanum, 1 for numpad
pub struct KeyPosition {
    column: u8,
    row: u8,
    grid_id: u8,
}

pub fn find_keys() -> Option<Vec<KeyPosition>> {
    let keys : Vec<KeyPosition> = Vec::new();
    loop {
        // loop over grids
        // loop over keys in a grid like this
        // https://en.wikipedia.org/wiki/Computer_keyboard#/media/File:FunctionalCircuitDiagramOfKeyboardNumPadScanningProcedure-small.gif
        break;
    }
    None
}

pub fn handle_keypress(keys: Vec<KeyCombo>) -> KeyCombo {
    // get position(s) in grid that are passed
    // map each postition to a keypress
    // return KeyCombo
    unimplemented!();
}

fn detect_keypress() -> Option<Vec<KeyCombo>> {
    let keys : Vec<KeyCombo>;
    if let Some(keypositions) = find_keys() {
        keypositions.iter().map(|&pos| map_position_to_key(pos)).collect()
    } else {
        None
    }
}
pub fn run() -> ! {
    loop {
        if let Some(keycombo) = detect_keypress() {
            handle_keypress(keycombo);
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

/// the dactyl has four grids of keys
/// on each side the dactyl has one grid for the alphanumerics
/// and one grid for the thumb keys.
