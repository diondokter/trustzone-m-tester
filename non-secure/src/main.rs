#![no_std]
#![no_main]

use trustzone_m_macros::secure_callable;

mod other_private_thing;
pub mod other_public_thing;

include!(concat!(env!("OUT_DIR"), "/trustzone_bindings.rs"));

static mut THING: u32 = 0;

#[secure_callable]
pub extern "C" fn write_thing(val: u32) {
    unsafe {
        THING = val + trustzone_bindings::return_5();
    }
}

#[secure_callable]
pub extern "C" fn read_thing() -> u32 {
    unsafe { THING }
}

/// Called when our code panics.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    cortex_m::asm::udf();
}
