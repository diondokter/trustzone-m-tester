#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rtt_target::rprintln;

const WRITE_THING_HASH: u32 = hash("write_thing");
const READ_THING_HASH: u32 = hash("read_thing");

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();

    rprintln!("Hello world!");

    let write_thing = unsafe { find_vector::<extern "C" fn(val: u32)>(WRITE_THING_HASH).unwrap() };
    let read_thing = unsafe { find_vector::<extern "C" fn() -> u32>(READ_THING_HASH).unwrap() };

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

const fn hash(name: &str) -> u32 {
    crc::Crc::<u32>::new(&crc::CRC_32_CKSUM).checksum(name.as_bytes())
}

unsafe fn find_vector<F>(name_hash: u32) -> Option<F> {
    extern "C" {
        static _NS_VECTORS: u32;
    }

    let mut ns_vectors_ptr = &_NS_VECTORS as *const u32 as *const (u32, u32);

    loop {
        let (vector, vector_hash) = *ns_vectors_ptr;

        if vector == 0 && vector_hash == 0 {
            // We've reached the end
            return None;
        }

        if vector_hash == name_hash {
            // We've found the vector we've been looking for
            return Some(core::mem::transmute_copy(&vector));
        }

        ns_vectors_ptr = ns_vectors_ptr.offset(1);
    }
}
