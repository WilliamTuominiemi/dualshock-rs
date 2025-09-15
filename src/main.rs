use rusb::{Context, Device, DeviceHandle, UsbContext, Direction, TransferType};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

fn ds_listen<T: UsbContext>(device: Device<T>) -> Result<(), Box<dyn std::error::Error>> {
    let config_desc = device.config_descriptor(0)?;
                
    let mut ds_endpoint = 0x00;

    for interface in config_desc.interfaces() {
        for interface_desc in interface.descriptors() {
            for endpoint_desc in interface_desc.endpoint_descriptors() {
                if endpoint_desc.transfer_type() ==  TransferType::Interrupt &&
                   endpoint_desc.direction() == Direction::In {
                    ds_endpoint = endpoint_desc.address();
                }
            }
        }
    }

    let mut ds_handle = device.open()?;

    let _ = ds_handle.claim_interface(0);

    let mut buf = [0u8; 64];
    
    loop {
        match ds_handle.read_interrupt(ds_endpoint, &mut buf, Duration::from_millis(1000)) {
            Ok(len) => {
                //println!("Received {} bytes: {:02x?}", len, &buf[..len]);
                let button: &u8 = &buf[..len][5];

                //println!("{:?}", button);
                ds_button(button);            
                                    
            }
            Err(rusb::Error::Timeout) => {
                continue;
            }
            Err(e) => {
                eprintln!("Error reading device: {}", e);
                break;
            }
        }
    }
    
    Ok(())
}

fn ds_button(button: &u8) {
    match button {
        40 => println!("X pressed"),
        24 => println!("Square pressed"),
        136 => println!("Triangle pressed"),
        72 => println!("Circle pressed"),
        _ => (),
    }
}
