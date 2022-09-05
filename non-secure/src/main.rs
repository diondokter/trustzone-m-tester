#![no_std]
#![no_main]

extern crate trustzone_m_nonsecure_rt;

static mut THING: u32 = 0;

#[link_section = ".vectors"]
#[used]
static WRITE_THING_VECTOR: (extern "C" fn(val: u32), u32) = (write_thing, hash("write_thing"));

#[link_section = ".text.exported"]
pub extern "C" fn write_thing(val: u32) {
    unsafe {
        THING = val;
    }
}

#[link_section = ".vectors"]
#[used]
static READ_THING_VECTOR: (extern "C" fn() -> u32, u32) = (read_thing, hash("read_thing"));

#[link_section = ".text.exported"]
pub extern "C" fn read_thing() -> u32 {
    unsafe { THING }
}

/// Called when our code panics.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    cortex_m::asm::udf();
}

const fn hash(name: &str) -> u32 {
    crc::Crc::<u32>::new(&crc::CRC_32_CKSUM).checksum(name.as_bytes())
}
