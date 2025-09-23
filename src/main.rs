use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use rusb::{Context, UsbContext};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

mod ds_listener;
use ds_listener::ds_listen;

mod render;
use render::draw;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    let listener_thread_tx = tx.clone();
    thread::spawn(|| {
        if let Err(e) = usb_listener(listener_thread_tx) {
            eprintln!("USB thread error: {}", e);
        }
    });

    let mut terminal = ratatui::init();
    let mut messages = Vec::new();

    loop {
        while let Ok(msg) = rx.try_recv() {
            messages.push(msg);
            if messages.len() > 20 {
                messages.remove(0);
            }
        }

        terminal.draw(|frame| draw(frame, &messages))?;

        if event::poll(Duration::from_millis(50))? {
            if handle_events()? {
                break;
            }
        }
    }

    ratatui::restore();
    Ok(())
}

fn handle_events() -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => return Ok(true),
            _ => {}
        },
        _ => {}
    }
    Ok(false)
}

fn usb_listener(thread_tx: Sender<String>) -> Result<(), Box<dyn std::error::Error>> {
    let context = Context::new()?;

    for device in context.devices()?.iter() {
        let desc = device.device_descriptor()?;

        if desc.vendor_id() == 0x54c && desc.product_id() == 0x05c4 {
            return ds_listen(device, thread_tx);
        }
    }

    Ok(())
}
