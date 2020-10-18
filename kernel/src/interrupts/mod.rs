use crate::board::acpi_table::AcpiTable;
use crate::memory::phys_to_virt;
use alloc::boxed::Box;
use alloc::vec::Vec;
use apic::{IoApic, LocalApic, XApic};
use lazy_static::*;
use spin::Mutex;
use trapframe;
use trapframe::TrapFrame;
use x86_64::instructions::interrupts;
use x86_64::registers::control::{Cr4, Cr4Flags};

const TABLE_SIZE: usize = 256;
const LAPIC_ADDR: usize = 0xfee0_0000;
const IOAPIC_ADDR: usize = 0xfec0_0000;
pub type InterruptHandle = Box<dyn Fn() + Send + Sync>;

lazy_static! {
    static ref IRQ_TABLE: Mutex<Vec<Option<InterruptHandle>>> = Default::default();
}

pub fn init() {
    unsafe {
        trapframe::init();
    }
    init_ioapic();
    init_irq_table();
    irq_enable_raw(KEYBOARD, KEYBOARD + IRQ0);
    irq_enable_raw(COM1, COM1 + IRQ0);

    unsafe {
        // enable global page
        Cr4::update(|f| f.insert(Cr4Flags::PAGE_GLOBAL));
    }
    interrupts::enable();
    test!("Init Interrupts");
}

fn init_irq_table() {
    let mut table = IRQ_TABLE.lock();
    for _ in 0..TABLE_SIZE {
        table.push(None);
    }
}

fn irq_enable_raw(irq: u8, vector: u8) {
    println!("irq_enable_raw: irq={:#x?}, vector={:#x?}", irq, vector);
    let mut ioapic = unsafe { IoApic::new(phys_to_virt(IOAPIC_ADDR)) };
    ioapic.set_irq_vector(irq, vector);
    ioapic.enable(irq, 0)
}

#[no_mangle]
pub extern "C" fn trap_handler(tf: &mut TrapFrame) {
    // debug!("Interrupt: {:#x} @ CPU{}", tf.trap_num, 0); // TODO 0 should replace in multi-core case
    match tf.trap_num as u8 {
        DOUBLE_FAULT => double_fault(tf),
        PAGE_FAULT => page_fault(tf),
        BREAKPOINT => breakpoint(),
        _ => panic!("Unhandled interrupt {:x} {:#x?}", tf.trap_num, tf),
    }
}

fn breakpoint() {
    panic!("\nEXCEPTION: BREAKPOINT");
}

fn double_fault(tf: &TrapFrame) {
    panic!("\nEXCEPTION: Double Fault\n{:#x?}", tf);
}

fn page_fault(tf: &mut TrapFrame) {
    panic!("\nEXCEPTION: Page Fault\n{:#x?}", tf);
}

fn init_ioapic() {
    unsafe {
        for ioapic in AcpiTable::get_ioapic() {
            println!("Ioapic found: {:#x?}", ioapic);
            let mut ip = IoApic::new(phys_to_virt(ioapic.address as usize));
            ip.disable_all();
            let mut lapic = XApic::new(phys_to_virt(ioapic.address as usize));
            lapic.cpu_init();
            let mut lapic =  XApic::new(phys_to_virt(LAPIC_ADDR));
            lapic.eoi();
        }
    }
    let mut ip = unsafe { IoApic::new(phys_to_virt(IOAPIC_ADDR)) };
    ip.disable_all();
}

#[allow(dead_code)]
// Reference: https://wiki.osdev.org/Exceptions
//const DivideError: u8 = 0;
//const Debug: u8 = 1;
//const NonMaskableInterrupt: u8 = 2;
const BREAKPOINT: u8 = 3;
//const Overflow: u8 = 4;
//const BoundRangeExceeded: u8 = 5;
//const InvalidOpcode: u8 = 6;
//const DeviceNotAvailable: u8 = 7;
const DOUBLE_FAULT: u8 = 8;
//const CoprocessorSegmentOverrun: u8 = 9;
//const InvalidTSS: u8 = 10;
//const SegmentNotPresent: u8 = 11;
//const StackSegmentFault: u8 = 12;
//const GeneralProtectionFault: u8 = 13;
const PAGE_FAULT: u8 = 14;
//const FloatingPointException: u8 = 16;
//const AlignmentCheck: u8 = 17;
//const MachineCheck: u8 = 18;
//const SIMDFloatingPointException: u8 = 19;
//const VirtualizationException: u8 = 20;
//const SecurityException: u8 = 30;

pub(crate) const IRQ0: u8 = 32;

// IRQ
pub const TIMER: u8 = 0;
pub const KEYBOARD: u8 = 1;
//const COM2: u8 = 3;
pub const COM1: u8 = 4;
//const IDE: u8 = 14;
//const Error: u8 = 19;
//const Spurious: u8 = 31;

