use rusb::{Context, Device, DeviceHandle, UsbContext};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = Context::new()?;

    for device in context.devices()?.iter() {
        let desc = device.device_descriptor()?;

        println!("Vendor: {:04x}, Product: {:04x}", 
            desc.vendor_id(), 
            desc.product_id());

        if desc.vendor_id() == 0x54c && desc.product_id() == 0x05c4 {
            println!("PS4 controller detected");
        }
    }

    Ok(())
}
