use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Gauge, List, ListItem, Paragraph},
};
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 || args[1] != "сделай красиво" {
        eprintln!("🔧 dvizhok v0.1.0");
        eprintln!("Использование: dvizhok \"сделай красиво\"");
        std::process::exit(1);
    }

    // ── Инициализация терминала ──
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    // ── Восстановление терминала ──
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("💥 АААА: {err:?}");
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press
                && (key.code == KeyCode::Char('q') || key.code == KeyCode::Esc)
            {
                return Ok(());
            }
        }
    }
}

/// ──────────────────────────────────────────────────────
///  ГЛАВНЫЙ РЕНДЕР — всё, абсолютно всё, это АААА
/// ──────────────────────────────────────────────────────
fn ui(frame: &mut Frame) {
    let area = frame.area();

    // Разбиваем экран на секции
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // заголовок
            Constraint::Length(3), // полоска HP/MP
            Constraint::Min(5),    // основной контент
            Constraint::Length(6), // инвентарь
            Constraint::Length(3), // чат / лог
            Constraint::Length(1), // нижняя строка
        ])
        .split(area);

    // ── 1) ЗАГОЛОВОК ──
    let title = Paragraph::new(Line::from(vec![
        Span::styled(
            "АААА",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(
            "АААААААА",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(
            "АААА",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(Color::Cyan)),
    )
    .style(Style::default().bg(Color::DarkGray));
    frame.render_widget(title, chunks[0]);

    // ── 2) HP / MP / XP ──
    let hp_bar = Gauge::default()
        .gauge_style(Style::default().fg(Color::Red))
        .label(Span::styled("АААА", Style::default().fg(Color::White)))
        .ratio(0.73);

    let mp_bar = Gauge::default()
        .gauge_style(Style::default().fg(Color::Blue))
        .label(Span::styled("АААААА", Style::default().fg(Color::White)))
        .ratio(0.41);

    let xp_bar = Gauge::default()
        .gauge_style(Style::default().fg(Color::Green))
        .label(Span::styled("АААА", Style::default().fg(Color::White)))
        .ratio(0.89);

    let bars = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ])
        .split(chunks[1]);

    frame.render_widget(hp_bar, bars[0]);
    frame.render_widget(mp_bar, bars[1]);
    frame.render_widget(xp_bar, bars[2]);

    // ── 3) ОСНОВНОЙ КОНТЕНТ — «карта» ──
    let map_lines: Vec<Line> = vec![
        Line::from("АААА  АААААААА  АААА  АААА  АААААААА  АААА  АААА"),
        Line::from("АААА  АА        АААА  АААА  АА        АААА  АААА"),
        Line::from("АААА  АААААААА  АААА  АААА  АААААААА  АААА  АААА"),
        Line::from("АААА        АА  АААА  АААА  АА        АААА  АААА"),
        Line::from("АААА  АААААААА  АААА  АААА  АААААААА  АААА  АААА"),
        Line::from("АААА  АА        АААА  АААА  АА        АААА  АААА"),
        Line::from("АААА  АААААААА  АААА  АААА  АААААААА  АААА  АААА"),
        Line::from(""),
        Line::from(Span::styled(
            "▶ АААААААА  АААА  АААААААА  АААА  АААА",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )),
    ];

    let map = Paragraph::new(map_lines)
        .block(
            Block::default()
                .title(Span::styled(
                    "АААААААА",
                    Style::default().fg(Color::Magenta),
                ))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Magenta)),
        )
        .style(Style::default().bg(Color::Black));
    frame.render_widget(map, chunks[2]);

    // ── 4) ИНВЕНТАРЬ ──
    let items: Vec<ListItem> = vec![
        ListItem::new(Line::from(vec![
            Span::styled("🗡  ", Style::default()),
            Span::styled("АААА", Style::default().fg(Color::Red)),
            Span::raw("  "),
            Span::styled("АААААААА: +АА", Style::default().fg(Color::Gray)),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("🛡  ", Style::default()),
            Span::styled("АААА", Style::default().fg(Color::Blue)),
            Span::raw("  "),
            Span::styled("АААААААА: +АА", Style::default().fg(Color::Gray)),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("🧪  ", Style::default()),
            Span::styled("АААА", Style::default().fg(Color::Green)),
            Span::raw("  "),
            Span::styled("АААААА: АААА", Style::default().fg(Color::Gray)),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("💎  ", Style::default()),
            Span::styled("АААААААА", Style::default().fg(Color::Yellow)),
            Span::raw("  "),
            Span::styled("АААА: 9999", Style::default().fg(Color::Gray)),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("📜  ", Style::default()),
            Span::styled("АААААА", Style::default().fg(Color::Cyan)),
            Span::raw("  "),
            Span::styled("АААААААА", Style::default().fg(Color::Gray)),
        ])),
    ];

    let inventory = List::new(items).block(
        Block::default()
            .title(Span::styled(
                "АААААААААААА",
                Style::default().fg(Color::Yellow),
            ))
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(Color::Yellow)),
    );
    frame.render_widget(inventory, chunks[3]);

    // ── 5) ЧАТ / ЛОГ ──
    let chat_lines: Vec<Line> = vec![
        Line::from(Span::styled(
            "[АА:АА:АА] АААА: АААААААААААААААААААА!",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            "[АА:АА:АА] АААААА: АААА АААА АААААААА...",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            "[АА:АА:АА] АААА: АААААА!!! АААААААА!!!",
            Style::default().fg(Color::Red),
        )),
    ];

    let chat = Paragraph::new(chat_lines).block(
        Block::default()
            .title(Span::styled("АААА", Style::default().fg(Color::Cyan)))
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(Style::default().fg(Color::Cyan)),
    );
    frame.render_widget(chat, chunks[4]);

    // ── 6) НИЖНЯЯ СТРОКА ──
    let bottom = Paragraph::new(Line::from(Span::styled(
        " АААА: АААА | АААА: АААА | АААА: АААААААА  [q] АААААААААА",
        Style::default().fg(Color::White).bg(Color::DarkGray),
    )));
    frame.render_widget(bottom, chunks[5]);
}
