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
    
    let mut ds_layout: Vec<(&str, bool)> = vec![
        ("cross", false), ("square", false), ("triangle", false), ("circle", false),
        ("down", false), ("left", false), ("up", false), ("right", false),
        ("L2", false), ("L1", false), ("R2", false), ("R1", false),
        ("Share", false), ("Options", false), ("L_joystick", false), ("R_joystick", false)
    ];
    
    loop {
        match ds_handle.read_interrupt(ds_endpoint, &mut buf, Duration::from_millis(1000)) {
            Ok(len) => {
                //println!("Received {} bytes: {:02x?}", len, &buf[..len]);
                let button: &u8 = &buf[..len][5];
                let special: &u8 = &buf[..len][6];


                println!("{:?}", ds_layout);

                ds_button(button, &mut ds_layout);            
                ds_special(special, &mut ds_layout);                    
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

fn ds_button(button: &u8, ds_layout: &mut Vec<(&str, bool)>) {
    match button {
        // Action buttons
        40 => ds_layout[0].1 = true, // Cross
        24 =>  ds_layout[1].1 = true, // Square
        136 => ds_layout[2].1 = true, // Triangle
        72 => ds_layout[3].1 = true, // Circle
        // Dpad
        04 => ds_layout[4].1 = true, // Down
        06 => ds_layout[5].1 = true, // Left
        00 => ds_layout[6].1 = true, // Up
        02 => ds_layout[7].1 = true, // Right
        _ => (),
    }
}

fn ds_special(input: &u8, ds_layout: &mut Vec<(&str, bool)>) {
    match input {
        0x04 => ds_layout[8].1 = true, // L2
        0x01 => ds_layout[9].1 = true, // L1
        0x08 => ds_layout[10].1 = true, // R2
        0x02 => ds_layout[11].1 = true, // R1
        0x10 => ds_layout[12].1 = true, // Share
        0x20 => ds_layout[13].1 = true, // Options
        0x40 => ds_layout[14].1 = true, // Left joystick
        0x80 => ds_layout[15].1 = true, // Right joystick
        _ => (),
    }
}
