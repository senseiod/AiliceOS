use crate::drivers::block::BlockDriver;
use crate::drivers::provider::Provider;
use crate::drivers::{DeviceType, Driver};
use crate::drivers::{BLK_DRIVERS, DRIVERS};
use alloc::{
    string::{String, ToString},
    sync::Arc,
};
use isomorphic_drivers::block::ahci::{AHCI, BLOCK_SIZE};
use spin::Mutex;

pub struct AHCIDriver(Mutex<AHCI<Provider>>);

impl Driver for AHCIDriver {
    fn try_handle_interrupt(&self, _irq: Option<usize>) -> bool {
        false
    }

    fn device_type(&self) -> DeviceType {
        DeviceType::Block
    }

    fn get_id(&self) -> String {
        "ahci".to_string()
    }

    fn as_block(&self) -> Option<&dyn BlockDriver> {
        Some(self)
    }
}

impl BlockDriver for AHCIDriver {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) -> bool {
        let mut driver = self.0.lock();
        driver.read_block(block_id, buf);
        true
    }

    fn write_block(&self, block_id: usize, buf: &[u8]) -> bool {
        if buf.len() < BLOCK_SIZE {
            return false;
        }
        let mut driver = self.0.lock();
        driver.write_block(block_id, buf);
        true
    }
}

pub fn init(_irq: Option<usize>, header: usize, size: usize) -> Option<Arc<AHCIDriver>> {
    if let Some(ahci) = AHCI::new(header, size) {
        let driver = Arc::new(AHCIDriver(Mutex::new(ahci)));
        DRIVERS.write().push(driver.clone());
        BLK_DRIVERS.write().push(driver.clone());
        Some(driver)
    } else {
        None
    }
}
