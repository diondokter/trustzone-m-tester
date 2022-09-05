#![no_std]
#![no_main]

extern crate trustzone_m_nonsecure_rt;

static mut THING: u32 = 0;

#[link_section = ".vectors"]
#[used]
static WRITE_THING_VECTOR: (extern "C" fn(val: u32), u32) = (write_thing, 
    crc::Crc::<u32>::new(&crc::CRC_32_CKSUM).checksum("write_thing".as_bytes()),
);

#[link_section = ".text.exported"]
pub extern "C" fn write_thing(val: u32) {
    unsafe {
        THING = val;
    }
}

#[link_section = ".vectors"]
#[used]
static READ_THING_VECTOR: (extern "C" fn() -> u32, u32) = (
    read_thing,
    crc::Crc::<u32>::new(&crc::CRC_32_CKSUM).checksum("read_thing".as_bytes()),
);

#[link_section = ".text.exported"]
pub extern "C" fn read_thing() -> u32 {
    unsafe { THING }
}

const fn adler32(string: &str) -> u32 {
    const MOD_ADLER: u32 = 65521;

    let data = string.as_bytes();

    let mut a = 1;
    let mut b = 0;

    let mut index = 0;

    while index < data.len() {
        a = (a + data[index] as u32) % MOD_ADLER;
        b = (b + a) % MOD_ADLER;
        index += 1;
    }

    (b << 16) | a
}

/// Called when our code panics.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    cortex_m::asm::udf();
}
