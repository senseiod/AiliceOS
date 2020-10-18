pub use acpi::{
    interrupt::{InterruptModel, InterruptSourceOverride, IoApic, Polarity, TriggerMode},
    Acpi,
};
use alloc::vec::Vec;
use lazy_static::*;
use spin::Mutex;
use acpi::{parse_rsdp, AcpiHandler, PhysicalMapping};
use crate::memory::{PAGE_SIZE, phys_to_virt};
use core::ptr::NonNull;
use bootloader::BootInfo;

pub struct AcpiTable {
    inner: Acpi,
}

static mut CONFIG: Config = Config {
    acpi_rsdp: 0,
    smbios: 0,
};

/// Configuration of HAL.
pub struct Config {
    pub acpi_rsdp: u64,
    pub smbios: u64,
}

lazy_static! {
    static ref ACPI_TABLE: Mutex<Option<AcpiTable>> = Mutex::default();
}

impl AcpiTable {
    fn initialize_check() {
                let mut table = ACPI_TABLE.lock();
                if table.is_none() {
                    *table = get_acpi_table().map(|x| AcpiTable { inner: x });
                }
    }
    pub fn invalidate() {
        *ACPI_TABLE.lock() = None;
    }
    pub fn get_ioapic() -> Vec<IoApic> {
        Self::initialize_check();
        let table = ACPI_TABLE.lock();
        match &*table {
            None => Vec::default(),
            Some(table) => match table.inner.interrupt_model.as_ref().unwrap() {
                InterruptModel::Apic(apic) => {
                    apic.io_apics.iter().map(|x| IoApic { ..*x }).collect()
                }
                _ => Vec::default(),
            },
        }
    }
    pub fn get_interrupt_source_overrides() -> Vec<InterruptSourceOverride> {
        Self::initialize_check();
        let table = ACPI_TABLE.lock();
        match &*table {
            None => Vec::default(),
            Some(table) => match table.inner.interrupt_model.as_ref().unwrap() {
                InterruptModel::Apic(apic) => apic
                    .interrupt_source_overrides
                    .iter()
                    .map(|x| InterruptSourceOverride {
                        polarity: Self::clone_polarity(&x.polarity),
                        trigger_mode: Self::clone_trigger_mode(&x.trigger_mode),
                        ..*x
                    })
                    .collect(),
                _ => Vec::default(),
            },
        }
    }
    fn clone_polarity(x: &Polarity) -> Polarity {
        match x {
            Polarity::SameAsBus => Polarity::SameAsBus,
            Polarity::ActiveHigh => Polarity::ActiveHigh,
            Polarity::ActiveLow => Polarity::ActiveLow,
        }
    }
    fn clone_trigger_mode(x: &TriggerMode) -> TriggerMode {
        match x {
            TriggerMode::SameAsBus => TriggerMode::SameAsBus,
            TriggerMode::Edge => TriggerMode::Edge,
            TriggerMode::Level => TriggerMode::Level,
        }
    }
}

/// Get physical address of `acpi_rsdp` and `smbios` on x86_64.
pub fn pc_firmware_tables() -> (u64, u64) {
    unsafe { (CONFIG.acpi_rsdp, CONFIG.smbios) }
}

pub fn get_acpi_addr(boot_info: &BootInfo) {
    unsafe {
        CONFIG = Config {
            acpi_rsdp: boot_info.acpi2_rsdp_addr,
            smbios: boot_info.smbios_addr,
        };
    }
}

pub fn get_acpi_table() -> Option<Acpi> {
    let mut handler = AcpiHelper {};
    println!("Get ACPI address :{:#x?}\n Smbios address: {:#x?}", pc_firmware_tables().0, pc_firmware_tables().1);
    match unsafe { parse_rsdp(&mut handler, pc_firmware_tables().0 as usize) } {
        Ok(table) => Some(table),
        Err(info) => {
            warn!("get_acpi_table error: {:#x?}", info);
            None
        }
    }
}

/// Build ACPI Table
struct AcpiHelper {}
impl AcpiHandler for AcpiHelper {
    unsafe fn map_physical_region<T>(
        &mut self,
        physical_address: usize,
        size: usize,
    ) -> PhysicalMapping<T> {
        #[allow(non_snake_case)]
            let OFFSET = 0;
        let page_start = physical_address / PAGE_SIZE;
        let page_end = (physical_address + size + PAGE_SIZE - 1) / PAGE_SIZE;
        PhysicalMapping::<T> {
            physical_start: physical_address,
            virtual_start: NonNull::new_unchecked(phys_to_virt(physical_address + OFFSET) as *mut T),
            mapped_length: size,
            region_length: PAGE_SIZE * (page_end - page_start),
        }
    }
    fn unmap_physical_region<T>(&mut self, _region: PhysicalMapping<T>) {}
}