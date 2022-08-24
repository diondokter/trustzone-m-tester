#![no_std]
#![no_main]

static mut THING: u32 = 0;

#[no_mangle]
pub unsafe extern "C" fn write_thing(val: u32) {
    THING = val;
}

#[no_mangle]
pub unsafe extern "C" fn read_thing() -> u32 {
    THING
}

/// Called when our code panics.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    cortex_m::interrupt::disable();
    cortex_m::asm::udf();
}
