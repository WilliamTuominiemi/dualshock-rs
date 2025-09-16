use rusb::{Context, Device, UsbContext, Direction, TransferType};
use std::time::Duration;

#[derive(Debug)]
#[derive(Clone)]
struct Input {
    name: String,
    code: u8,
    pressed: bool,
}

impl Input {
    fn press(&mut self) {
        self.pressed = true;
    }
}

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

    let ds_handle = device.open()?;

    let _ = ds_handle.claim_interface(0);

    let mut buf = [0u8; 64];
    
    let mut ds_layout = vec![
        // Action buttons
        Input {name: String::from("cross"), code: 40, pressed: false},
        Input {name: String::from("square"), code: 24, pressed: false},
        Input {name: String::from("triangle"), code: 136, pressed: false},
        Input {name: String::from("circle"), code: 72, pressed: false},
        // D-pad
        Input {name: String::from("down"), code: 04, pressed: false},
        Input {name: String::from("left"), code: 06, pressed: false},
        Input {name: String::from("up"), code: 00, pressed: false},
        Input {name: String::from("right"), code: 02, pressed: false},
        // Triggers
        Input {name: String::from("L2"), code: 0x04, pressed: false},
        Input {name: String::from("L1"), code: 0x01, pressed: false},
        Input {name: String::from("R2"), code: 0x08, pressed: false},
        Input {name: String::from("R1"), code: 0x02, pressed: false},
        // Flat buttons 
        Input {name: String::from("options"), code: 0x10, pressed: false},
        Input {name: String::from("share"), code: 0x20, pressed: false},
        // Joysticks
        Input {name: String::from("left_joystick"), code: 0x40, pressed: false},
        Input {name: String::from("right_joystick"), code: 0x80, pressed: false},
    ];
    
    loop {
        match ds_handle.read_interrupt(ds_endpoint, &mut buf, Duration::from_millis(1000)) {
            Ok(len) => {
                let button: &u8 = &buf[..len][5];
                let special: &u8 = &buf[..len][6];

                ds_button(button, special, &mut ds_layout);            
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

fn ds_button(button: &u8, special: &u8, ds_layout: &mut Vec<Input>) {
    for input in ds_layout.iter_mut() {
        if !input.pressed && (*button == input.code || *special == input.code) {
            input.press();
            println!("{:?}", input);
        }
    }
}

