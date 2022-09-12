#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rtt_target::rprintln;

include!(concat!(env!("OUT_DIR"), "/trustzone_bindings.rs"));

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();

    rprintln!("Hello world!");

    rprintln!("Calling 'write_thing' with 5");
    trustzone_bindings::write_thing(5);
    rprintln!("Read call: {}", trustzone_bindings::read_thing());
    rprintln!("Calling 'write_thing' with 10");
    trustzone_bindings::write_thing(10);
    rprintln!("Read call: {}", trustzone_bindings::read_thing());

    loop {
        cortex_m::asm::bkpt();
    }
}

#[cortex_m_rt::exception]
unsafe fn HardFault(frame: &cortex_m_rt::ExceptionFrame) -> ! {
    rprintln!("{:?}", frame);
    cortex_m::peripheral::SCB::sys_reset();
}

/// Called when our code panics.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_m::interrupt::disable();
    rprintln!("{}", info);
    cortex_m::asm::udf();
}
