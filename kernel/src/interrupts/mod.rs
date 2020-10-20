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
    irq_add_handle(KEYBOARD + IRQ0, Box::new(keyboard));
    irq_add_handle(COM1 + IRQ0, Box::new(com1));
    irq_enable_raw(KEYBOARD, KEYBOARD + IRQ0);
    irq_enable_raw(COM1, COM1 + IRQ0);
    crate::drivers::serial::COM1.lock().init();
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
        IRQ0..=63 => irq_handle(tf.trap_num as u8),
        _ => panic!("Unhandled interrupt {:x} {:#x?}", tf.trap_num, tf),
    }
}

pub fn irq_handle(irq: u8) {
    let mut lapic = unsafe { XApic::new(phys_to_virt(LAPIC_ADDR)) };
    lapic.eoi();
    let table = IRQ_TABLE.lock();
    match &table[irq as usize] {
        Some(f) => f(),
        None => panic!("unhandled external IRQ number: {}", irq),
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

fn com1() {
    let c = crate::drivers::serial::COM1.lock().receive();
    crate::drivers::serial::serial_put(c);
}

fn keyboard() {
    use pc_keyboard::{DecodedKey, KeyCode};
    if let Some(key) = crate::board::keyboard::receive() {
        match key {
            DecodedKey::Unicode(c) => print!("{}", c),
            DecodedKey::RawKey(code) => {
                let s = match code {
                    KeyCode::ArrowUp => "\u{1b}[A",
                    KeyCode::ArrowDown => "\u{1b}[B",
                    KeyCode::ArrowRight => "\u{1b}[C",
                    KeyCode::ArrowLeft => "\u{1b}[D",
                    _ => "",
                };
                for c in s.bytes() {
                    print!("{}", c);
                }
            }
        }
    }
}

fn init_ioapic() {
    unsafe {
        for ioapic in AcpiTable::get_ioapic() {
            println!("Ioapic found: {:#x?}", ioapic);
            let mut ip = IoApic::new(phys_to_virt(ioapic.address as usize));
            ip.disable_all();
            let mut lapic = XApic::new(phys_to_virt(ioapic.address as usize));
            lapic.cpu_init();
        }
    }
    let mut ip = unsafe { IoApic::new(phys_to_virt(IOAPIC_ADDR)) };
    ip.disable_all();
}

/// Add a handle to IRQ table. Return the specified irq or an allocated irq on success
pub fn irq_add_handle(irq: u8, handle: InterruptHandle) -> Option<u8> {
    debug!("IRQ add handle {:#x?}", irq);
    let mut table = IRQ_TABLE.lock();
    // allocate a valid irq number
    if irq == 0 {
        let mut id = 0x20;
        while id < table.len() {
            if table[id].is_none() {
                table[id] = Some(handle);
                return Some(id as u8);
            }
            id += 1;
        }
        return None;
    }
    match table[irq as usize] {
        Some(_) => None,
        None => {
            table[irq as usize] = Some(handle);
            Some(irq)
        }
    }
}

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

