use trapframe;
use trapframe::TrapFrame;

pub fn init() {
    unsafe {
        trapframe::init();
    }
    println!("Init Interrupts");
}

#[no_mangle]
pub extern "C" fn trap_handler(tf: &mut TrapFrame) {
    panic!("Unsupport interrupt {:x} {:#?}", tf.trap_num, tf);
}
