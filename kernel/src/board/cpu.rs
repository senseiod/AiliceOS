use core::sync::atomic::{spin_loop_hint, AtomicBool, Ordering};
use raw_cpuid::CpuId;
use x86_64::registers::control::{Cr0, Cr0Flags, Cr4, Cr4Flags};

pub static AP_CAN_INIT: AtomicBool = AtomicBool::new(false);

pub fn init_cpu() {
    let cpu_id = CpuId::new()
        .get_feature_info()
        .unwrap()
        .initial_local_apic_id() as usize;
    println!("I'm from {} cpu", cpu_id);
    if cpu_id != 0 {
        while !AP_CAN_INIT.load(Ordering::Relaxed) {
            spin_loop_hint();
        }
        println!("I'm Two!");
    }
    let cpuid = CpuId::new();
    if let Some(vendor_info) = cpuid.get_vendor_info() {
        println!("CPU {}", vendor_info);
    }

    unsafe {
        Cr4::update(|cr4| {
            // enable fxsave/fxrstor
            cr4.insert(Cr4Flags::OSFXSR);
            // sse
            cr4.insert(Cr4Flags::OSXMMEXCPT_ENABLE);
        });
        Cr0::update(|cr0| {
            // enable fpu
            cr0.remove(Cr0Flags::EMULATE_COPROCESSOR);
            cr0.insert(Cr0Flags::MONITOR_COPROCESSOR);
        });
    }

    // wake up other CPUs
    AP_CAN_INIT.store(true, Ordering::Relaxed);
    test!("Init CPU and FPU");
    test!("init CPU");
}
