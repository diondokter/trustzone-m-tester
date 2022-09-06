#![no_std]
#![no_main]

extern crate trustzone_m_nonsecure_rt;

use trustzone_m_macros::secure_callable;

static mut THING: u32 = 0;

#[secure_callable]
pub extern "C" fn write_thing(val: u32) {
    unsafe {
        THING = val;
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
