use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use rusb::{Context, UsbContext};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

mod ds_listener;
use ds_listener::ds_listen;

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

fn draw(frame: &mut Frame, messages: &[String]) {
    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Percentage(30),
            Constraint::Percentage(15),
        ])
        .split(frame.area());

    let middle_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(horizontal_chunks[1]);

    let text = if messages.is_empty() {
        "Press q to quit".to_string()
    } else {
        messages.join("\n")
    };

    let left_container = Block::default().borders(Borders::LEFT | Borders::BOTTOM | Borders::TOP);
    let left_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(horizontal_chunks[0]);
    let left_top = Block::default().borders(Borders::LEFT | Borders::TOP);
    frame.render_widget(left_top, left_vertical[0]);
    let left_bottom = Block::default().borders(Borders::LEFT | Borders::BOTTOM | Borders::RIGHT);
    frame.render_widget(left_bottom, left_vertical[1]);
    frame.render_widget(left_container, horizontal_chunks[0]);

    let center_container = Block::default().borders(Borders::TOP | Borders::BOTTOM);
    let center_paragraph = Paragraph::new(text.clone())
        .block(center_container)
        .wrap(Wrap { trim: true });
    frame.render_widget(center_paragraph, middle_vertical[0]);

    let right_container = Block::default().borders(Borders::RIGHT | Borders::BOTTOM | Borders::TOP);
    let right_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(horizontal_chunks[2]);
    let right_top = Block::default().borders(Borders::RIGHT | Borders::TOP);
    frame.render_widget(right_top, right_vertical[0]);
    let right_bottom = Block::default().borders(Borders::LEFT | Borders::BOTTOM | Borders::RIGHT);
    frame.render_widget(right_bottom, right_vertical[1]);
    frame.render_widget(right_container, horizontal_chunks[2]);
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
