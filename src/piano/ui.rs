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
        Paragraph, Row, Sparkline, Table, Tabs, Wrap,
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
                .map(|i| ListItem::new(vec![Spans::from(Span::raw(i))]))
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
                    .bg(Color::Blue)
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
    let current_music = match app.cur_music {
        Some(music_index) => {app.music_list.items[music_index].as_str()},
        None => "No Music",
    };
    let line_gauge = LineGauge::default()
    .block(Block::default().title(current_music))
    .gauge_style(Style::default().fg(Color::LightBlue))
    .line_set(if app.enhanced_graphics {
        symbols::line::THICK
    } else {
        symbols::line::NORMAL
    })
    .ratio(app.progress);
    f.render_widget(line_gauge, area);
}

fn draw_second_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .direction(Direction::Horizontal)
        .split(area);
    let up_style = Style::default().fg(Color::Green);
    let failure_style = Style::default()
        .fg(Color::Red)
        .add_modifier(Modifier::RAPID_BLINK | Modifier::CROSSED_OUT);
    let header = ["Server", "Location", "Status"];
    let rows = app.servers.iter().map(|s| {
        let style = if s.status == "Up" {
            up_style
        } else {
            failure_style
        };
        Row::StyledData(vec![s.name, s.location, s.status].into_iter(), style)
    });
    let table = Table::new(header.iter(), rows)
        .block(Block::default().title("Servers").borders(Borders::ALL))
        .header_style(Style::default().fg(Color::Yellow))
        .widths(&[
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Length(10),
        ]);
    f.render_widget(table, chunks[0]);

    let map = Canvas::default()
        .block(Block::default().title("World").borders(Borders::ALL))
        .paint(|ctx| {
            ctx.draw(&Map {
                color: Color::White,
                resolution: MapResolution::High,
            });
            ctx.layer();
            ctx.draw(&Rectangle {
                x: 0.0,
                y: 30.0,
                width: 10.0,
                height: 10.0,
                color: Color::Yellow,
            });
            for (i, s1) in app.servers.iter().enumerate() {
                for s2 in &app.servers[i + 1..] {
                    ctx.draw(&Line {
                        x1: s1.coords.1,
                        y1: s1.coords.0,
                        y2: s2.coords.0,
                        x2: s2.coords.1,
                        color: Color::Yellow,
                    });
                }
            }
            for server in &app.servers {
                let color = if server.status == "Up" {
                    Color::Green
                } else {
                    Color::Red
                };
                ctx.print(server.coords.1, server.coords.0, "X", color);
            }
        })
        .marker(if app.enhanced_graphics {
            symbols::Marker::Braille
        } else {
            symbols::Marker::Dot
        })
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0]);
    f.render_widget(map, chunks[1]);
}
