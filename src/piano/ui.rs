use crate::piano::App;
#[allow(unused_imports)]
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle, Points},
    widgets::{
        Axis, BarChart, Block, Borders, Chart, Dataset, Gauge, LineGauge, List, ListItem,
        Paragraph, Row, Sparkline, Table, Tabs, Wrap, BorderType,
    },
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);
    f.render_widget(tabs, chunks[0]);
    match app.tabs.index {
        0 => draw_first_tab(f, app, chunks[1]),
        1 => draw_second_tab(f, app, chunks[1]),
        _ => {}
    };
}

fn draw_first_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(9),
                Constraint::Min(8),
                Constraint::Length(7),
            ]
            .as_ref(),
        )
        .split(area);
    draw_welcome(f, app, chunks[0]);
    draw_music_list(f, app, chunks[1]);
    draw_music_gauge(f, app, chunks[2]);
}

/// Draw the welcome area
fn draw_welcome<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .margin(1)
        .split(area);
    
    // Draw the welcome text
    let text = vec![
        Spans::from("Welcome to the Virtual Piano written with Rust!"),
        Spans::from("Type \'<-\' and \'->\' to change between Music Mode and Piano Mode."),
        Spans::from("Type \'h\' to get help."),
    ];
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Welcome",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);

    // Draw the sparkline
    let sparkline = Sparkline::default()
        .block(Block::default())
        .style(Style::default().fg(Color::Green))
        .data(&app.sparkline.points)
        .bar_set(if app.enhanced_graphics {
            symbols::bar::NINE_LEVELS
        } else {
            symbols::bar::THREE_LEVELS
        });
    f.render_widget(sparkline, chunks[1]);
}

fn draw_music_list<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{    
    let constraints = vec![Constraint::Percentage(50), Constraint::Percentage(50)]; 
    let chunks = Layout::default()
        .constraints(constraints)
        .direction(Direction::Horizontal)
        .split(area);
    {
        let chunks = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[0]);
        {
            let chunks = Layout::default()
                .constraints([Constraint::Percentage(100)].as_ref())
                .direction(Direction::Horizontal)
                .split(chunks[0]);

            let tasks: Vec<ListItem> = app
                .music_list
                .items
                .iter()
                .map(|i| ListItem::new(vec![Spans::from(Span::raw(i.name()))]))
                .collect();

            let tasks = List::new(tasks)
                .block(Block::default().borders(Borders::ALL).title(Span::styled(
                    "Music List",
                    Style::default()
                        .fg(Color::LightCyan)
                        .add_modifier(Modifier::BOLD),
                )))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol("> ");
            f.render_stateful_widget(tasks, chunks[0], &mut app.music_list.state);
        }

        // Draw Music Waves
        let barchart = BarChart::default()
            .block(Block::default().borders(Borders::ALL).title(Span::styled(
                "Music Waves",
                Style::default()
                    .fg(Color::LightRed)
                    .add_modifier(Modifier::BOLD),
            )))
            .data(&app.barchart)
            .bar_width(3)
            .bar_gap(2)
            .bar_set(if app.enhanced_graphics {
                symbols::bar::NINE_LEVELS
            } else {
                symbols::bar::THREE_LEVELS
            })
            .value_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Green)
                    .add_modifier(Modifier::ITALIC),
            )
            .label_style(Style::default().fg(Color::Red))
            .bar_style(Style::default().fg(Color::Blue));
        f.render_widget(barchart, chunks[1]);
    }
    if app.show_chart {
        let x_labels = vec![
            Span::styled(
                format!("{}", app.signals.window[0]),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!(
                "{}",
                (app.signals.window[0] + app.signals.window[1]) / 2.0
            )),
            Span::styled(
                format!("{}", app.signals.window[1]),
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ];
        let datasets = vec![
            Dataset::default()
                .name("high pitch")
                .marker(symbols::Marker::Dot)
                .style(Style::default().fg(Color::Cyan))
                .data(&app.signals.sin1.points),
            Dataset::default()
                .name("low pitch")
                .marker(if app.enhanced_graphics {
                    symbols::Marker::Braille
                } else {
                    symbols::Marker::Dot
                })
                .style(Style::default().fg(Color::Yellow))
                .data(&app.signals.sin2.points),
        ];
        let chart = Chart::new(datasets)
            .block(
                Block::default()
                    .title(Span::styled(
                        "Music Status",
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ))
                    .borders(Borders::ALL),
            )
            .x_axis(
                Axis::default()
                    .title("Time")
                    .style(Style::default().fg(Color::Gray))
                    .bounds(app.signals.window)
                    .labels(x_labels),
            )
            .y_axis(
                Axis::default()
                    .title("Pitch")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([-20.0, 20.0])
                    .labels(vec![
                        Span::styled("-20", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw("0"),
                        Span::styled("20", Style::default().add_modifier(Modifier::BOLD)),
                    ]),
            );
        f.render_widget(chart, chunks[1]);
    } else {
        // Draw logo
        // _  .-')                  .-')     .-') _    
        // ( \( -O )                ( OO ).  (  OO) )   
        // ,------.   ,--. ,--.   (_)---\_) /     '._  
        // |   /`. '  |  | |  |   /    _ |  |'--...__) 
        // |  /  | |  |  | | .-') \  :` `.  '--.  .--' 
        // |  |_.' |  |  |_|( OO ) '..`''.)    |  |    
        // |  .  '.'  |  | | `-' /.-._)   \    |  |    
        // |  |\  \  ('  '-'(_.-' \       /    |  |    
        // `--' '--'   `-----'     `-----'     `--'
        
        // PPPPPPPPPPPPPPPPP     iiii                                                       
        // P::::::::::::::::P   i::::i                                                      
        // P::::::PPPPPP:::::P   iiii                                                       
        // PP:::::P     P:::::P                                                             
        // P::::P     P:::::Piiiiiii   aaaaaaaaaaaaa   nnnn  nnnnnnnn       ooooooooooo   
        // P::::P     P:::::Pi:::::i   a::::::::::::a  n:::nn::::::::nn   oo:::::::::::oo 
        // P::::PPPPPP:::::P  i::::i   aaaaaaaaa:::::a n::::::::::::::nn o:::::::::::::::o
        // P:::::::::::::PP   i::::i            a::::a nn:::::::::::::::no:::::ooooo:::::o
        // P::::PPPPPPPPP     i::::i     aaaaaaa:::::a   n:::::nnnn:::::no::::o     o::::o
        // P::::P             i::::i   aa::::::::::::a   n::::n    n::::no::::o     o::::o
        // P::::P             i::::i  a::::aaaa::::::a   n::::n    n::::no::::o     o::::o
        // P::::P             i::::i a::::a    a:::::a   n::::n    n::::no::::o     o::::o
        // PP::::::PP         i::::::ia::::a    a:::::a   n::::n    n::::no:::::ooooo:::::o
        // P::::::::P         i::::::ia:::::aaaa::::::a   n::::n    n::::no:::::::::::::::o
        // P::::::::P         i::::::i a::::::::::aa:::a  n::::n    n::::n oo:::::::::::oo 
        // PPPPPPPPPP         iiiiiiii  aaaaaaaaaa  aaaa  nnnnnn    nnnnnn   ooooooooooo   
                                                                                                                                                            
        let logo = vec![
            Spans::from("_  .-')                  .-')     .-') _    "),
            Spans::from("( \\( -O )                ( OO ).  (  OO) )   "),
            Spans::from(",------.   ,--. ,--.   (_)---\\_) /     '._  "),
            Spans::from("|   /`. '  |  | |  |   /    _ |  |'--...__) "),
            Spans::from("|  /  | |  |  | | .-') \\  :` `.  '--.  .--' "),
            Spans::from("|  |_.' |  |  |_|( OO ) '..`''.)    |  |    "),
            Spans::from("|  .  '.'  |  | | `-' /.-._)   \\    |  |    "),
            Spans::from("|  |\\  \\  ('  '-'(_.-' \\       /    |  |    "),
            Spans::from("`--' '--'   `-----'     `-----'     `--'    "),
            Spans::from(""),
            Spans::from("PPPPPPPPPPPPPPPPP     iiii"),
            Spans::from("P::::::::::::::::P   i::::i"),
            Spans::from("P::::::PPPPPP:::::P   iiii"),
            Spans::from("PP:::::P     P:::::P"),
            Spans::from("P::::P     P:::::P   iiiiiii    aaaaaaaaaaaaa   nnnn  nnnnnnnn        ooooooooooo"),
            Spans::from("P::::P     P:::::P   i:::::i    a::::::::::::a  n:::nn::::::::nn    oo:::::::::::oo"),
            Spans::from("P::::PPPPPP:::::P     i::::i    aaaaaaaaa:::::a n::::::::::::::nn  o:::::::::::::::o"),
            Spans::from("P:::::::::::::PP      i::::i             a::::a nn:::::::::::::::n o:::::ooooo:::::o"),
            Spans::from("P::::PPPPPPPPP        i::::i      aaaaaaa:::::a   n:::::nnnn:::::n o::::o     o::::o"),
            Spans::from("P::::P                i::::i    aa::::::::::::a   n::::n    n::::n o::::o     o::::o"),
            Spans::from("P::::P                i::::i   a::::aaaa::::::a   n::::n    n::::n o::::o     o::::o"),
            Spans::from("P::::P                i::::i  a::::a    a:::::a   n::::n    n::::n o::::o     o::::o"),
            Spans::from("PP::::::PP           i::::::i a::::a    a:::::a   n::::n    n::::n o:::::ooooo:::::o"),
            Spans::from("P::::::::P           i::::::i a:::::aaaa::::::a   n::::n    n::::n o:::::::::::::::o"),
            Spans::from("P::::::::P           i::::::i  a::::::::::aa:::a  n::::n    n::::n  oo:::::::::::oo"),
            Spans::from("PPPPPPPPPP           iiiiiiii   aaaaaaaaaa  aaaa  nnnnnn    nnnnnn    ooooooooooo"),
        ];
        let block = Block::default().borders(Borders::ALL).title(Span::styled(
            "Logo",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ));
        let logo = Paragraph::new(logo).block(block).wrap(Wrap { trim: true });
        f.render_widget(logo, chunks[1]);
    }
}

fn draw_music_gauge<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{   
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .direction(Direction::Vertical)
        .split(area);
    let mut progress: f64 = 0.2;
    let current_music = match app.cur_music {
        Some(music_index) => {
            let interval = app.audio_player.progress();
            progress = interval / app.music_list.items[music_index].duration.as_micros() as f64;
            app.music_list.items[music_index].name()
        },
        None => "No Music",
    };
    if progress > 0.99 {
        if progress > 1.0 {
            app.audio_player.refresh_progress();
        }
        progress = 1.0;
    }
    let gauge = Gauge::default()
        .block(Block::default().title(current_music))
        .gauge_style(
            Style::default()
                .fg(Color::Blue)
                .bg(Color::Black)
                .add_modifier(Modifier::ITALIC),
        )
        .ratio(progress);
    f.render_widget(gauge, chunks[0]);

    let volume = app.audio_player.get_music_volume() as f64;
    let line_gauge = LineGauge::default()
    .block(Block::default().title("Volume"))
    .gauge_style(Style::default().fg(Color::Cyan))
    .line_set(symbols::line::THICK)
    .ratio(volume);
    f.render_widget(line_gauge, chunks[1]);
}

fn draw_second_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .direction(Direction::Vertical)
        .split(area);
    
    let mut texts = Vec::new();
    texts.push(vec!["Esc", "K+", "K-", "LS", "RS", "L0+", "L0-", "R0+", "R0-", "LV+", "LV-", "RV+", "RV-", "Play", "Rec", "Stop"]);
    texts.push(vec!["~", "1", "2", "3", "4", "5", "6", "7", "1+", "2+", "3+", "4+", "5+", "Backspace", "4++", "5++", "6++", "4+", "5+", "6+", "7+"]);
    texts.push(vec!["Tab", "1-", "2-", "3-", "4-", "5-", "6-", "7-", "1", "2", "3", "4", "5", "6", "1++", "2++", "3++", "7", "1+", "2+", "3+"]);
    texts.push(vec!["Caps", "1--", "2--", "3--", "4--", "5--", "6--", "7--", "1-", "2-", "3-", "4-", "Enter", "4", "5", "6"]);
    texts.push(vec!["Shift", "1---", "2---", "3---", "4---", "5---", "6---", "7---", "1--", "2--", "3--", "Shift", "4-", "1", "2", "3", "7-"]);
    texts.push(vec!["Ctrl", "Win", "Alt", "Space", "Alt", "Win", "App", "Ctrl", "1-", "2-", "3-", "5-", "6-"]);
    let chunks_keybord = Layout::default()
        .constraints(
            [
                Constraint::Percentage(16),
                Constraint::Percentage(16),
                Constraint::Percentage(16),
                Constraint::Percentage(16),
                Constraint::Percentage(16),
                Constraint::Percentage(16),
                Constraint::Percentage(4),
            ]
            .as_ref()
        )
        .direction(Direction::Vertical)
        .split(chunks[0]);
    
    let chunks_keybord_0 = Layout::default()
        .constraints(
            [
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
            ]
            .as_ref()
        )
        .direction(Direction::Horizontal)
        .split(chunks_keybord[0]);
    
    let mut blocks = Vec::new();
    for _ in 0..(chunks_keybord_0.len() - 5) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .border_type(BorderType::Rounded)
            .style(Style::default());
        blocks.push(block);
    }
    let mut text_index = 0usize;
    for i in 0..chunks_keybord_0.len() {
        if i == 1 || i == 10 || i == 18 || i == 19 || i == 20 || i ==21 || i == 22 {
            continue;
        }
        // let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
        let text = texts[0][text_index];
        let block = blocks.pop().unwrap();
        let text = Paragraph::new(text).block(block).wrap(Wrap {trim: true});
        text_index += 1;
        f.render_widget(text, chunks_keybord_0[i]);
    }

    let chunks_keybord_1 = Layout::default()
        .constraints(
            [
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(9),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(11),
            ]
            .as_ref()
        )
        .direction(Direction::Horizontal)
        .split(chunks_keybord[1]);
    
    for _ in 0..(chunks_keybord_1.len() - 1) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .border_type(BorderType::Rounded)
            .style(Style::default());
        blocks.push(block);
    }

    text_index = 0usize;
    for i in 0..chunks_keybord_1.len() {
        if i == 21 {
            continue;
        }
        let block = blocks.pop().unwrap();
        let text = texts[1][text_index];
        let text = Paragraph::new(text).block(block).wrap(Wrap {trim: true});
        text_index += 1;
        f.render_widget(text, chunks_keybord_1[i]);
    }

    let chunks_keybord_2 = Layout::default()
        .constraints(
            [
                Constraint::Percentage(7),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(7),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
                Constraint::Percentage(4),
            ]
            .as_ref()
        )
        .direction(Direction::Horizontal)
        .split(chunks_keybord[2]);
    
        for _ in 0..(chunks_keybord_2.len() - 2) {
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .border_type(BorderType::Rounded)
                .style(Style::default());
            blocks.push(block);
        }
        text_index = 0usize;
        for i in 0..chunks_keybord_2.len() {
            if i == chunks_keybord_2.len() - 2 {
                let block = Block::default()
                    .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT)
                    .border_style(Style::default().fg(Color::White))
                    .border_type(BorderType::Rounded)
                    .style(Style::default());
                let text = texts[2][texts[2].len() - 1];
                let text = Paragraph::new(text).block(block).wrap(Wrap {trim: true});
                f.render_widget(text, chunks_keybord_2[i]);
            }
            else if i == chunks_keybord_2.len() - 1 {
                continue;
            }
            else {
                let block = blocks.pop().unwrap();
                let text = texts[2][text_index];
                let text = Paragraph::new(text).block(block).wrap(Wrap {trim: true});
                text_index += 1;
                f.render_widget(text, chunks_keybord_2[i]);
            }
        }

        let chunks_keybord_3 = Layout::default()
            .constraints(
                [
                    Constraint::Percentage(9),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(9),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                ]
                .as_ref()
            )
            .direction(Direction::Horizontal)
            .split(chunks_keybord[3]);
        
        for _ in 0..(chunks_keybord_3.len() - 5) {
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .border_type(BorderType::Rounded)
                .style(Style::default());
            blocks.push(block);
        }
        text_index = 0;
        for i in 0..chunks_keybord_3.len() {
            if i == 13 || i == 14 || i == 15 || i == chunks_keybord_3.len() - 1{
                continue;
            }
            else if i == chunks_keybord_3.len() - 2 {
                let block = Block::default()
                    .borders(Borders::LEFT | Borders::BOTTOM | Borders::RIGHT)
                    .border_style(Style::default().fg(Color::White))
                    .border_type(BorderType::Rounded)
                    .style(Style::default());
                f.render_widget(block, chunks_keybord_3[i]);
            }
            else {
                let block = blocks.pop().unwrap();
                let text = texts[3][text_index];
                let text = Paragraph::new(text).block(block).wrap(Wrap {trim: true});
                text_index += 1;
                f.render_widget(text, chunks_keybord_3[i]);
            }
        }

        let chunks_keybord_4 = Layout::default()
            .constraints(
                [
                    Constraint::Percentage(11),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(11),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                ]
                .as_ref()
            )
            .direction(Direction::Horizontal)
            .split(chunks_keybord[4]);

        for _ in 0..(chunks_keybord_3.len() - 4) {
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .border_type(BorderType::Rounded)
                .style(Style::default());
            blocks.push(block);
        }

        text_index = 0;
        for i in 0..chunks_keybord_4.len() {
            if i == 12 || i == 14 || i == chunks_keybord_4.len() - 1 {
                continue;
            }
            else if i == chunks_keybord_4.len() - 2 {
                let block = Block::default()
                    .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT)
                    .border_style(Style::default().fg(Color::White))
                    .border_type(BorderType::Rounded)
                    .style(Style::default());
                let text = texts[4][texts[4].len() - 1];
                let text = Paragraph::new(text).block(block).wrap(Wrap {trim: true});
                f.render_widget(text, chunks_keybord_4[i]);
            }
            else {
                let block = blocks.pop().unwrap();
                let text = texts[4][text_index];
                let text = Paragraph::new(text).block(block).wrap(Wrap {trim: true});
                text_index += 1;
                f.render_widget(text, chunks_keybord_4[i]);
            }
        }

        let chunks_keybord_5 = Layout::default()
            .constraints(
                [
                    Constraint::Percentage(6),
                    Constraint::Percentage(6),
                    Constraint::Percentage(4),
                    Constraint::Percentage(28),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(5),
                    Constraint::Percentage(6),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(7),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                    Constraint::Percentage(4),
                ]
                .as_ref()
            )
            .direction(Direction::Horizontal)
            .split(chunks_keybord[5]);
        
        for _ in 0..(chunks_keybord_5.len() - 1) {
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .border_type(BorderType::Rounded)
                .style(Style::default());
            blocks.push(block);
        }

        text_index = 0;
        for i in 0..chunks_keybord_5.len() {
            if i == chunks_keybord_5.len() - 2 {
                let block = Block::default()
                    .borders(Borders::LEFT | Borders::BOTTOM | Borders::RIGHT)
                    .border_style(Style::default().fg(Color::White))
                    .border_type(BorderType::Rounded)
                    .style(Style::default());
                f.render_widget(block, chunks_keybord_5[i]);
            }
            else if i == chunks_keybord_5.len() - 1 {
                continue;
            }
            else {
                let block = blocks.pop().unwrap();
                let text = texts[5][text_index];
                let text = Paragraph::new(text).block(block).wrap(Wrap {trim: true});
                text_index += 1;
                f.render_widget(text, chunks_keybord_5[i]);
            }
        }
}
