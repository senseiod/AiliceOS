use trapframe;
use trapframe::TrapFrame;
use x86_64::instructions::interrupts;

pub fn init() {
    unsafe {
        trapframe::init();
    }
    interrupts::enable();
    test!("Init Interrupts");
}

#[no_mangle]
pub extern "C" fn trap_handler(tf: &mut TrapFrame) {
    panic!("Unsupport interrupt {:x} {:#?}", tf.trap_num, tf);
}
