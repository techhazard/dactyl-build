#![feature(asm, core_intrinsics)]
extern crate core;

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

macro_rules! magic {() => (unimplemented!())}

macro_rules! UnimplementedType {
    () => { UnimplementedType::None };
}

pub mod ledcontrol;
pub mod timecontrol;
mod settings;
#[cfg(lighting)]
mod lighting;
mod internal_command;
use internal_command::InternalCommand;
mod devices;

type List<T> = Vec<T>;

// for all types thet I still have to think about
enum UnimplementedType {
    None,
}

// don't I just send keycodes to the PC?
enum Key {
    Char(u32),
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

enum Side {
    Left,
    Right,
}

struct KeyCode(u32);

enum Modifier {
    Alt(Side),
    Altgr(Side),
    Shift(Side),
    Ctrl(Side),
    Super(Side),
    Meta(Side),
    Hyper(Side),
}

enum KeySequence {
    KeyPresses(KeyCombo),
    Internal(InternalCommand),
    Nothing,
}

struct KeyCombo {
    key: List<Key>,
    /// one of: shift ctrl alt altgr super meta hyper
    modifiers: List<Modifier>,
}

/// Designates the position in the keygrid.
/// A grid can be at most 256 keys wide
/// and 256 keys tall
/// grid_id is the id of the grid the key is in,
/// e.g. 0 for alphanum, 1 for numpad
struct KeyPosition {
    column: u8,
    row: u8,
    grid_id: u8,
}

/// Handle the keypress and send it to the receiving device
/// side effect: send out stuff to device
fn handle_keypresses(keys: KeyCombo, ref settings: &settings::Settings) {
    unimplemented!();
    // send pressed keys to host
}

/// Somehow detect keypresses from the key matrix
fn detect_keypress() -> KeySequence {
    magic!();
}

fn execute_internal_command(command: InternalCommand, ref mut settings: &mut settings::Settings) {
    unimplemented!();
}

/// Start the firmware
///
/// Does not return (designated by "!" )
///
/// Example
///     use dactyl_firmware as firmware;
///
///     firmware::run();
pub fn run() -> ! {
    // first we initialise some variables
    let mut settings: settings::Settings = settings::load_settings();

    // if something changes (e.g. device connected),
    // change settings.device to None so it re-checks
    if let None = settings.devices {
        settings.devices = devices::detect()
    }

    loop {
        use KeySequence::*;
        match detect_keypress() {
            Internal(command) => execute_internal_command(command, &mut settings),
            KeyPresses(keycombo) => handle_keypresses(keycombo, &settings),
            Nothing => {}
        };
    }
}

fn map_positions_to_keys(positions: List<KeyPosition>) -> List<KeyCombo> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

// the dactyl has four grids of keys
// on each side the dactyl has one grid for the alphanumerics
// and one grid for the thumb keys.
//
// TODO:
// - replace vec with nostd compatible vec (on stack)
//
// Idea:
// handle state like react+redux
// TODO
// - check if redux lib exists and is nostd compatible
