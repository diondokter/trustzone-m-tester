#![no_std]
#![no_main]

extern crate trustzone_m_nonsecure_rt;

static mut THING: u32 = 0;

#[link_section = ".text.exported"]
#[no_mangle]
pub extern "C" fn write_thing(val: u32) {
    unsafe {
        THING = val;
    }
}

#[link_section = ".text.exported"]
#[no_mangle]
pub extern "C" fn read_thing() -> u32 {
    unsafe {
        THING
    }
}

/// Called when our code panics.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    cortex_m::interrupt::disable();
    cortex_m::asm::udf();
}
