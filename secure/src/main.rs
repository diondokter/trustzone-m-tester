#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rtt_target::rprintln;

#[link(name = "non_secure")]
extern "C" {
    fn write_thing(val: u32);
}

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_target::rtt_init_default!();

    rprintln!("Hello world!");

    unsafe {
        write_thing(5);
    }

    loop {
        cortex_m::asm::bkpt();
    }
}

#[cortex_m_rt::exception]
unsafe fn HardFault(frame: &cortex_m_rt::ExceptionFrame) -> ! {
    // rprintln!("{:?}", frame);
    cortex_m::peripheral::SCB::sys_reset();
}

/// Called when our code panics.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_m::interrupt::disable();
    // rprintln!("{}", info);
    cortex_m::asm::udf();
}
