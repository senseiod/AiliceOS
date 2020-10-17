use raw_cpuid::CpuId;
use core::sync::atomic::AtomicBool;

pub static AP_CAN_INIT: AtomicBool = AtomicBool::new(false);

pub fn early_init_cpu() {
    let cpu_id = CpuId::new().get_feature_info().unwrap().initial_local_apic_id() as usize;
    println!("I'm from {} cpu", cpu_id);
    if cpu_id != 0 {
        println!("I'm Two!");
    }
    let cpuid = CpuId::new();
    if let Some(vendor_info) = cpuid.get_vendor_info() {
        println!("CPU {}", vendor_info);
    }
    test!("Early init CPU");
}
