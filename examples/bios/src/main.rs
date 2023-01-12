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

mod allocator;
mod boot;
mod exceptions;
mod global;
mod handlers;
mod misc;
mod rand;
mod stdout;
mod thread;

use crate::misc::get_system_info;
use crate::thread::Thread;
use core::ffi::CStr;
use psx::hw::cop0::IntSrc;
use psx::hw::{cop0, irq, Register};

fn main() {
    // This main loop doesn't do anything useful yet, it's only used to test
    // functionality that would be exposed to executables if the BIOS could load
    // them
    println!("Starting main BIOS loop");
    let version_str = unsafe { CStr::from_ptr(get_system_info(2) as *const i8) };
    let bios_date = get_system_info(0);
    println!("{:?}", version_str);
    println!("{:x?}", bios_date);
    cop0::Status::new()
        .enable_interrupts()
        .unmask_interrupt(IntSrc::Hardware)
        .use_boot_vectors(false)
        .store();
    irq::Status::skip_load().ack_all().store();
    irq::Mask::new().enable_all().store();

    let mut t = Thread::new(task).unwrap();
    let mut counter = 5;
    loop {
        println!("hello from main thread");
        if counter != 0 {
            counter -= 1;
            t.resume();
        }
    }
}

extern "C" fn task() -> ! {
    let mut counter = 10;
    loop {
        println!("hello from task thread");
        if counter != 0 {
            counter -= 1;
            Thread::resume_main();
        }
    }
}
