#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
#![feature(asm_const)]
#![feature(naked_functions)]
#![feature(core_intrinsics)]
#![feature(const_mut_refs)]
#![feature(const_ptr_as_ref)]
#![feature(const_option)]
#![feature(inline_const)]

use core::arch::asm;
use core::ffi::CStr;
use psx::constants::KB;
use psx::hw::cop0;
use psx::hw::cop0::IntSrc;
use psx::hw::irq;
use psx::hw::Register;
use psx::sys::kernel::*;

mod allocator;
mod boot;
mod exceptions;
mod global;
mod misc;
mod rand;
mod stdout;
mod thread;

use crate::allocator::{free, init_heap, malloc};
use crate::misc::get_system_info;
use crate::rand::{rand, srand};
use crate::stdout::printf;
use crate::thread::{change_thread, close_thread, open_thread, MAIN_THREAD};

fn main() {
    // This main loop doesn't do anything useful yet, it's only used to test
    // functionality that would be exposed to executables if the BIOS could load
    // them
    println!("Starting main BIOS loop");
    let version_str = unsafe { CStr::from_ptr(get_system_info(2) as *const i8) };
    let bios_date = get_system_info(0);
    println!("{:?}", version_str);
    println!("{:x?}", bios_date);
    let mut sr = cop0::Status::new();
    sr.enable_interrupts()
        .unmask_interrupt(IntSrc::Hardware)
        .use_boot_vectors(false)
        .store();
    irq::Status::skip_load().ack_all().store();
    let mut mask = irq::Mask::new();
    println!("{:#x?}", cop0::Status::new());

    static mut THREAD_STACK: [u8; KB] = [0; KB];
    let t = open_thread(
        task_thread as extern "C" fn() as u32,
        unsafe { &mut THREAD_STACK[KB - 8] } as *mut u8 as u32,
        0,
    );
    extern "C" fn task_thread() {
        loop {
            println!("hello from task thread");
            change_thread(MAIN_THREAD);
        }
    }
    loop {
        println!("hello from main thread");
        change_thread(t);
    }
}

// These sets of four instructions are written to the BIOS fn vectors
#[naked]
unsafe extern "C" fn a0_fn_vec() {
    asm! {
        ".set noreorder
         la $10, fn_handler
         jr $10
         or $8, $0, 0xA0
         .set reorder",
        options(noreturn)
    }
}
#[naked]
unsafe extern "C" fn b0_fn_vec() {
    asm! {
        ".set noreorder
         la $10, fn_handler
         jr $10
         or $8, $0, 0xB0
         .set reorder",
        options(noreturn)
    }
}
#[naked]
unsafe extern "C" fn c0_fn_vec() {
    asm! {
        ".set noreorder
         la $10, fn_handler
         jr $10
         or $8, $0, 0xC0
         .set reorder",
        options(noreturn)
    }
}

// The handler called by the three BIOS fn vectors.
#[no_mangle]
extern "C" fn fn_handler() -> u32 {
    // Bind a register's value to an identifier
    macro_rules! reg {
        (let $var:ident = $reg:tt) => {
            reg!(let $var: u32 = $reg);
        };
        (let $var:ident: $size:ty = $reg:tt) => {
            let $var: $size;
            unsafe {
                asm! { "", out($reg) $var }
            }
        };
    }

    reg!(let fn_ty: u8 = "$8");
    reg!(let fn_num: u8 = "$9");
    // TODO: Consider switching to the table of function pointers approached
    // used by other BIOS implementations
    match (fn_num, fn_ty) {
        (INIT_HEAP_NUM, INIT_HEAP_TY) => {
            reg!(let addr = "$4");
            reg!(let len: usize = "$5");
            init_heap(addr as *mut u8, len)
        },
        (MALLOC_NUM, MALLOC_TY) => {
            reg!(let len: usize = "$4");
            malloc(len) as u32
        },
        (FREE_NUM, FREE_TY) => {
            reg!(let ptr = "$4");
            free(ptr as *mut u8)
        },
        (SRAND_NUM, SRAND_TY) => {
            reg!(let seed = "$4");
            srand(seed)
        },
        (RAND_NUM, RAND_TY) => rand(),
        (PRINTF_NUM, PRINTF_TY) => {
            reg!(let fmt_ptr = "$4");
            reg!(let arg0 = "$5");
            reg!(let arg1 = "$6");
            reg!(let arg2 = "$7");
            // SAFETY: Let's hope the user passed in a null-terminated string
            let fmt_str = unsafe { CStr::from_ptr(fmt_ptr as *const i8) };
            printf(fmt_str, arg0, arg1, arg2)
        },
        (GET_SYSTEM_INFO_NUM, GET_SYSTEM_INFO_TY) => {
            reg!(let idx: u8 = "$4");
            get_system_info(idx)
        },
        (OPEN_THREAD_NUM, OPEN_THREAD_TY) => {
            reg!(let pc: u32 = "$4");
            reg!(let sp: u32 = "$5");
            reg!(let gp: u32 = "$6");
            open_thread(pc, sp, gp)
        },
        (CHANGE_THREAD_NUM, CHANGE_THREAD_TY) => {
            reg!(let handle: u32 = "$4");
            change_thread(handle)
        },
        (CLOSE_THREAD_NUM, CLOSE_THREAD_TY) => {
            reg!(let handle: u32 = "$4");
            close_thread(handle)
        },
        (STD_OUT_PUTCHAR_NUM, STD_OUT_PUTCHAR_TY) => {
            // Emulators usually implement debug output by checking that PC reaches
            // 0x8000_00B0 with $9 set to 0x3D so the BIOS just needs to return to the
            // caller in this case.
            0
        },
        _ => {
            println!("Called unimplemented function {:x}({:x})", fn_ty, fn_num);
            u32::MAX
        },
    }
}
