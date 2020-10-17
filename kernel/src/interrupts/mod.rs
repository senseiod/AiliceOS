use trapframe;
use trapframe::TrapFrame;
use x86_64::instructions::interrupts;
use x86_64::registers::control::{Cr4, Cr4Flags};

pub fn init() {
    unsafe {
        trapframe::init();
    }
    unsafe {
        // enable global page
        Cr4::update(|f| f.insert(Cr4Flags::PAGE_GLOBAL));
    }
    interrupts::enable();
    test!("Init Interrupts");
}

#[no_mangle]
pub extern "C" fn trap_handler(tf: &mut TrapFrame) {
    panic!("Unsupport interrupt {:x} {:#?}", tf.trap_num, tf);
}
