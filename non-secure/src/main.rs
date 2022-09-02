#![no_std]
#![no_main]

extern crate trustzone_m_nonsecure_rt;

static mut THING: u32 = 0;

#[link_section = ".ns_vector.len"]
#[no_mangle]
static mut VT_LEN: usize = 0;

#[link_section = ".ns_vectors.ptrs"]
#[no_mangle]
static mut VT: [(u32, u32); 8] = [(0, 0); 8];

#[link_section = ".ctor.init"]
#[used]
static CTORS_INIT: extern "C" fn() = ctors_init;
extern "C" fn ctors_init() {
    unsafe {
        VT_LEN = 0;
        VT.iter_mut().for_each(|v| *v = (0, 0));
    }

    extern "C" {
        static _sctors: unsafe extern "C" fn();
        static _ectors: unsafe extern "C" fn();
    }

    let ctors = unsafe {
        core::slice::from_raw_parts(
            &_sctors as *const unsafe extern "C" fn(),
            (&_ectors as *const unsafe extern "C" fn())
                .offset_from(&_sctors as *const unsafe extern "C" fn()) as usize,
        )
    };

    // Skip the first ctor, because that's this init function
    for ctor in ctors.iter().skip(1).take(unsafe { VT.len() }) {
        unsafe { ctor() };
    }
}





#[link_section = ".ctor"]
#[used]
static WRITE_THING_CTOR: unsafe extern "C" fn() = write_thing_ctor;
unsafe extern "C" fn write_thing_ctor() {
    *VT.get_unchecked_mut(VT_LEN) = (
        write_thing as *const u32 as u32,
        "write_thing"
            .chars()
            .map(|c| c as u32)
            .fold(u32::MAX, |l, r| l ^ r),
    );

    VT_LEN += 1;
}

#[link_section = ".text.exported"]
pub extern "C" fn write_thing(val: u32) {
    unsafe {
        THING = val;
    }
}





#[link_section = ".ctor"]
#[used]
static READ_THING_CTOR: unsafe extern "C" fn() = read_thing_ctor;
unsafe extern "C" fn read_thing_ctor() {
    *VT.get_unchecked_mut(VT_LEN) = (
        read_thing as *const u32 as u32,
        "read_thing"
            .chars()
            .map(|c| c as u32)
            .fold(u32::MAX, |l, r| l ^ r),
    );

    VT_LEN += 1;
}

// TODO: Invastigate doing this instead of having the constructors
#[used]
static READ_THING: (extern "C" fn() -> u32, u32) = (read_thing, adler32("read_thing"));

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
