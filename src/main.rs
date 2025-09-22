use rusb::{Context, UsbContext};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

mod ds_listener;
use ds_listener::ds_listen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    let listener_thread_tx = tx.clone();
    let listener = thread::spawn(|| {
        if let Err(e) = usb_listener(listener_thread_tx) {
            eprintln!("USB thread error: {}", e);
        }
    });

    loop {
        while let Ok(msg) = rx.try_recv() {
            println!("{:?}", msg);
        }
    }

    Ok(())
}

fn usb_listener(thread_tx: Sender<String>) -> Result<(), Box<dyn std::error::Error>> {
    let context = Context::new()?;

    for device in context.devices()?.iter() {
        let desc = device.device_descriptor()?;

        if desc.vendor_id() == 0x54c && desc.product_id() == 0x05c4 {
            println!("PS4 controller detected");
            return ds_listen(device, thread_tx);
        }
    }

    Ok(())
}
