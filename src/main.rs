use rusb::{Context, UsbContext};
use std::thread;

mod ds_listener;
use ds_listener::ds_listen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = thread::spawn(|| {
        if let Err(e) = usb_listener() {
            eprintln!("USB thread error: {}", e);
        }
    });

    listener.join().unwrap();

    Ok(())
}

fn usb_listener() -> Result<(), Box<dyn std::error::Error>> {
    let context = Context::new()?;

    for device in context.devices()?.iter() {
        let desc = device.device_descriptor()?;

        if desc.vendor_id() == 0x54c && desc.product_id() == 0x05c4 {
            println!("PS4 controller detected");
            return ds_listen(device);
        }
    }

    Ok(())
}
