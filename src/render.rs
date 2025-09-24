use ratatui::style::{Color, Style};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn draw(frame: &mut Frame, messages: &[String]) {
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

    draw_left_side(frame, messages, &horizontal_chunks);

    draw_center(frame, messages, &middle_vertical);

    draw_right_side(frame, messages, &horizontal_chunks);
}

fn draw_button(
    frame: &mut Frame,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    border_style: Style,
) -> Rect {
    let button = Rect::new(x, y, width, height);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style);
    frame.render_widget(block, button);
    button
}

fn button_border_style(messages: &[String], key: &str) -> Style {
    if messages.iter().any(|s| s == key) {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    }
}

fn draw_left_side(frame: &mut Frame, messages: &[String], horizontal_chunks: &std::rc::Rc<[Rect]>) {
    let left_container = Block::default().borders(Borders::LEFT | Borders::BOTTOM | Borders::TOP);
    let left_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(horizontal_chunks[0]);
    let left_top = Block::default().borders(Borders::LEFT | Borders::TOP);

    let left_inner_area = left_top.inner(left_vertical[0]);
    frame.render_widget(left_top, left_vertical[0]);

    let l2_button = draw_button(
        frame,
        left_inner_area.x + 5,
        left_inner_area.y,
        15,
        3,
        button_border_style(messages, "L2"),
    );

    let l1_button = draw_button(
        frame,
        left_inner_area.x + 5,
        left_inner_area.y + l2_button.height,
        15,
        2,
        button_border_style(messages, "L1"),
    );

    let up_button = draw_button(
        frame,
        left_inner_area.x + 10,
        left_inner_area.y + l2_button.height + l1_button.height,
        5,
        3,
        button_border_style(messages, "up"),
    );

    let left_button = draw_button(
        frame,
        left_inner_area.x + 5,
        left_inner_area.y + l2_button.height + l1_button.height + up_button.height,
        5,
        3,
        button_border_style(messages, "left"),
    );

    let right_button = draw_button(
        frame,
        left_inner_area.x + 15,
        left_inner_area.y + l2_button.height + l1_button.height + up_button.height,
        5,
        3,
        button_border_style(messages, "right"),
    );

    let down_button = draw_button(
        frame,
        left_inner_area.x + 10,
        left_inner_area.y
            + l2_button.height
            + l1_button.height
            + up_button.height
            + right_button.height,
        5,
        3,
        button_border_style(messages, "down"),
    );

    let left_bottom = Block::default().borders(Borders::LEFT | Borders::BOTTOM | Borders::RIGHT);
    frame.render_widget(left_bottom, left_vertical[1]);
    frame.render_widget(left_container, horizontal_chunks[0]);
}

fn draw_right_side(
    frame: &mut Frame,
    messages: &[String],
    horizontal_chunks: &std::rc::Rc<[Rect]>,
) {
    let right_container = Block::default().borders(Borders::RIGHT | Borders::BOTTOM | Borders::TOP);
    let right_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(horizontal_chunks[2]);
    let right_top = Block::default().borders(Borders::RIGHT | Borders::TOP);

    let right_inner_area = right_top.inner(right_vertical[0]);
    frame.render_widget(right_top, right_vertical[0]);

    let r2_button = draw_button(
        frame,
        right_inner_area.x + 5,
        right_inner_area.y,
        15,
        3,
        button_border_style(messages, "R2"),
    );

    let r1_button = draw_button(
        frame,
        right_inner_area.x + 5,
        right_inner_area.y + r2_button.height,
        15,
        2,
        button_border_style(messages, "R1"),
    );

    let triangle_button = draw_button(
        frame,
        right_inner_area.x + 10,
        right_inner_area.y + r2_button.height + r1_button.height,
        5,
        3,
        button_border_style(messages, "triangle"),
    );

    let square_button = draw_button(
        frame,
        right_inner_area.x + 5,
        right_inner_area.y + r2_button.height + r1_button.height + triangle_button.height,
        5,
        3,
        button_border_style(messages, "square"),
    );

    let circle_button = draw_button(
        frame,
        right_inner_area.x + 15,
        right_inner_area.y + r2_button.height + r1_button.height + triangle_button.height,
        5,
        3,
        button_border_style(messages, "circle"),
    );

    let cross_button = draw_button(
        frame,
        right_inner_area.x + 10,
        right_inner_area.y
            + r2_button.height
            + r1_button.height
            + triangle_button.height
            + circle_button.height,
        5,
        3,
        button_border_style(messages, "cross"),
    );

    let right_bottom = Block::default().borders(Borders::LEFT | Borders::BOTTOM | Borders::RIGHT);
    frame.render_widget(right_bottom, right_vertical[1]);
    frame.render_widget(right_container, horizontal_chunks[2]);
}

fn draw_center(frame: &mut Frame, messages: &[String], middle_vertical: &std::rc::Rc<[Rect]>) {
    let center_container = Block::default().borders(Borders::TOP | Borders::BOTTOM);

    let center_vertical_top = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(middle_vertical[0]);
    let center_top = Block::default().borders(Borders::BOTTOM | Borders::TOP);

    let center_inner_top_area = center_top.inner(center_vertical_top[0]);
    frame.render_widget(center_top, center_vertical_top[0]);

    let share_button = draw_button(
        frame,
        center_inner_top_area.x,
        center_inner_top_area.y,
        5,
        4,
        button_border_style(messages, "share"),
    );

    let options_button = draw_button(
        frame,
        center_inner_top_area.x + center_inner_top_area.width - 5,
        center_inner_top_area.y,
        5,
        4,
        button_border_style(messages, "options"),
    );

    let center_vertical_bottom = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(middle_vertical[1]);
    let center_bottom = Block::default();

    let center_inner_bottom_area = center_bottom.inner(center_vertical_top[1]);
    frame.render_widget(center_bottom, center_vertical_top[1]);

    let l_joystick_button = draw_button(
        frame,
        center_inner_bottom_area.x,
        center_inner_bottom_area.y,
        8,
        5,
        button_border_style(messages, "left_joystick"),
    );

    let r_joystick_button = draw_button(
        frame,
        center_inner_bottom_area.x + center_inner_bottom_area.width - 8,
        center_inner_bottom_area.y,
        8,
        5,
        button_border_style(messages, "right_joystick"),
    );

    frame.render_widget(center_container, middle_vertical[0]);
}
