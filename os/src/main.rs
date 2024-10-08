#![no_std]
#![no_main]

mod lang_items;
use core::arch::global_asm;
mod console;
mod batch;
mod sync;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));


mod sbi;
#[no_mangle]
pub fn rust_main() -> ! {
	clear_bss();
	println!("Hello world!");
	panic!("Shutdown machine!");
}

fn clear_bss() {
	extern "C" {
		fn sbss();
		fn ebss();
	}
	(sbss as usize..ebss as usize).for_each(|a| {
		unsafe { (a as *mut u8).write_volatile(0) }
	});
}
