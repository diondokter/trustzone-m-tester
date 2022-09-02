#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rtt_target::rprintln;

extern "C" {
    static _NS_CTOR_INIT: u32;
    static mut _NS_VT_LEN: u32;
    static _NS_VT: u32;
}

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();

    let _ns_ctor_init = unsafe { *(&_NS_CTOR_INIT as *const _ as *const extern "C" fn()) };
    let _ns_vt_len = unsafe { &mut _NS_VT_LEN as *mut u32 as *mut usize };

    rprintln!("Hello world!");

    rprintln!("VT_LEN: {:?}", unsafe { _ns_vt_len.read_volatile() });
    rprintln!("CTOR_INIT: {:p}", _ns_ctor_init);

    _ns_ctor_init();

    rprintln!("VT_LEN: {:?}", unsafe { _ns_vt_len.read_volatile() });

    let _ns_vt = unsafe { &_NS_VT as *const u32 as *const (u32, u32) };

    rprintln!("VT: {:p}", _ns_vt);

    let ns_vt = unsafe {
        core::slice::from_raw_parts(
            _ns_vt,
            _ns_vt_len.read_volatile(),
        )
    };

    rprintln!("VT: {:X?}", ns_vt);

    let read_thing: extern "C" fn() -> u32 = ns_vt.iter().find(|(_ptr, hash)| {
        *hash
            == "read_thing"
                .chars()
                .map(|c| c as u32)
                .fold(u32::MAX, |l, r| l ^ r)
    }).map(|(ptr, _)| unsafe { core::mem::transmute(*ptr) }).unwrap();

    rprintln!("read_thing: {:p}", read_thing);

    let write_thing: extern "C" fn(u32) = ns_vt.iter().find(|(_ptr, hash)| {
        *hash
            == "write_thing"
                .chars()
                .map(|c| c as u32)
                .fold(u32::MAX, |l, r| l ^ r)
    }).map(|(ptr, _)| unsafe { core::mem::transmute(*ptr) }).unwrap();

    rprintln!("write_thing: {:p}", write_thing);

    rprintln!("Calling 'write_thing' with 5");
    write_thing(5);
    rprintln!("Read call: {}", read_thing());
    rprintln!("Calling 'write_thing' with 10");
    write_thing(10);
    rprintln!("Read call: {}", read_thing());

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
