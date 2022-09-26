#![no_std]
#![no_main]

use trustzone_m_macros::secure_callable;

mod other_private_thing;
pub mod other_public_thing;

// include!(concat!(env!("OUT_DIR"), "/trustzone_bindings.rs"));

pub mod trustzone_bindings {
    #[inline(never)]
    pub extern "C" fn return_5() -> u32 {
        const HASH: u32 = 3146409456u32;
        let fn_ptr = unsafe { super::find_nsc_vector::<extern "C" fn() -> u32>(HASH).unwrap() };
        fn_ptr()
    }
}
#[allow(dead_code)]
#[inline(never)]
unsafe fn find_ns_vector<F>(name_hash: u32) -> Option<F> {
    extern "C" {
        static _NS_VECTORS: u32;
    }
    let mut ns_vectors_ptr = &_NS_VECTORS as *const u32 as *const (u32, u32);
    loop {
        let (vector, vector_hash) = *ns_vectors_ptr;
        if vector == 0 && vector_hash == 0 {
            return None;
        }
        if vector_hash == name_hash {
            return Some(core::mem::transmute_copy(&vector));
        }
        ns_vectors_ptr = ns_vectors_ptr.offset(1);
    }
}
#[allow(dead_code)]
#[inline(never)]
unsafe fn find_nsc_vector<F>(name_hash: u32) -> Option<F> {
    extern "C" {
        static _NSC_VECTORS: u32;
    }
    let mut nsc_vectors_ptr = &_NSC_VECTORS as *const u32 as *const (u32, u32);
    loop {
        let (vector, vector_hash) = *nsc_vectors_ptr;
        if vector == 0 && vector_hash == 0 {
            return None;
        }
        if vector_hash == name_hash {
            return Some(core::mem::transmute_copy(&vector));
        }
        nsc_vectors_ptr = nsc_vectors_ptr.offset(1);
    }
}

static mut THING: u32 = 0;

#[secure_callable]
pub extern "C" fn write_thing(val: u32) {
    unsafe {
        THING = val + trustzone_bindings::return_5();
    }
}

#[secure_callable]
pub extern "C" fn read_thing() -> u32 {
    unsafe { THING }
}

/// Called when our code panics.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    cortex_m::asm::bkpt();
    cortex_m::asm::udf();
}
