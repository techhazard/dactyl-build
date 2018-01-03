#[derive(Copy, Clone)]
pub struct Milliseconds(pub i32);

impl From<Milliseconds> for i32 {
    fn from(ms: Milliseconds) -> i32 {
        ms.0
    }
}

///  Please do not assume this sleeps for
///  the set aumount of time.
///  The clockspeed might've changed,
///  or something else might not go as expected
pub fn rough_sleep(ms: Milliseconds) {
    let milliseconds: i32 = i32::from(ms);

    for _ in 0..milliseconds * 250 {
        unsafe {
            asm!("NOP");
        }
    }
}
