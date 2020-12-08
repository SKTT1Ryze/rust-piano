use crate::util::{RandomSignal, SinSignal, StatefulList, TabsState};
use std::vec::Vec;
use walkdir::WalkDir;
use super::music::Music;
use super::player::AudioPlayer;

const EVENTS: [(&str, u64); 24] = [
    ("B1", 9),
    ("B2", 12),
    ("B3", 5),
    ("B4", 8),
    ("B5", 2),
    ("B6", 4),
    ("B7", 5),
    ("B8", 9),
    ("B9", 14),
    ("B10", 15),
    ("B11", 1),
    ("B12", 0),
    ("B13", 4),
    ("B14", 6),
    ("B15", 4),
    ("B16", 6),
    ("B17", 4),
    ("B18", 7),
    ("B19", 13),
    ("B20", 8),
    ("B21", 11),
    ("B22", 9),
    ("B23", 3),
    ("B24", 5),
];

pub struct Signal<S: Iterator> {
    source: S,
    pub points: Vec<S::Item>,
    tick_rate: usize,
}

impl<S> Signal<S>
where
    S: Iterator,
{
    fn on_tick(&mut self) {
        for _ in 0..self.tick_rate {
            self.points.remove(0);
        }
        self.points
            .extend(self.source.by_ref().take(self.tick_rate));
    }
}

pub struct Signals {
    pub sin1: Signal<SinSignal>,
    pub sin2: Signal<SinSignal>,
    pub window: [f64; 2],
}

impl Signals {
    fn on_tick(&mut self) {
        self.sin1.on_tick();
        self.sin2.on_tick();
        self.window[0] += 1.0;
        self.window[1] += 1.0;
    }
}
pub struct Server<'a> {
    pub name: &'a str,
    pub location: &'a str,
    pub coords: (f64, f64),
    pub status: &'a str,
}
pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub show_chart: bool,
    pub progress: f64,
    pub sparkline: Signal<RandomSignal>,
    pub signals: Signals,
    pub barchart: Vec<(&'a str, u64)>,
    pub servers: Vec<Server<'a>>,
    pub enhanced_graphics: bool,
    pub music_list: StatefulList<Music>,
    pub cur_music: Option<usize>,
    pub audio_player: AudioPlayer,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool, music_list_path: &'a str) -> App<'a> {
        let mut rand_signal = RandomSignal::new(0, 100);
        let sparkline_points = rand_signal.by_ref().take(300).collect();
        let mut sin_signal = SinSignal::new(0.2, 3.0, 18.0);
        let sin1_points = sin_signal.by_ref().take(100).collect();
        let mut sin_signal2 = SinSignal::new(0.1, 2.0, 10.0);
        let sin2_points = sin_signal2.by_ref().take(200).collect();
        let mut music_list = Vec::new();
        for entry in WalkDir::new(music_list_path) {
            let entry = entry.unwrap();
            let path = String::from(entry.path().to_str().unwrap());
            if path.len() > music_list_path.len() {
                let music = Music::new(path.as_str()).unwrap();
                music_list.push(music);
            }
        }
        let audio_player = AudioPlayer::new(3).unwrap();
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["Music", "Piano"]),
            show_chart: false,
            progress: 0.0,
            sparkline: Signal {
                source: rand_signal,
                points: sparkline_points,
                tick_rate: 1,
            },
            signals: Signals {
                sin1: Signal {
                    source: sin_signal,
                    points: sin1_points,
                    tick_rate: 5,
                },
                sin2: Signal {
                    source: sin_signal2,
                    points: sin2_points,
                    tick_rate: 10,
                },
                window: [0.0, 20.0],
            },
            barchart: EVENTS.to_vec(),
            servers: vec![
                Server {
                    name: "NorthAmerica-1",
                    location: "New York City",
                    coords: (40.71, -74.00),
                    status: "Up",
                },
                Server {
                    name: "Europe-1",
                    location: "Paris",
                    coords: (48.85, 2.35),
                    status: "Failure",
                },
                Server {
                    name: "SouthAmerica-1",
                    location: "São Paulo",
                    coords: (-23.54, -46.62),
                    status: "Up",
                },
                Server {
                    name: "Asia-1",
                    location: "Singapore",
                    coords: (1.35, 103.86),
                    status: "Up",
                },
            ],
            enhanced_graphics,
            music_list: StatefulList::with_items(music_list),
            cur_music: None,
            audio_player,
        }
    }

    pub fn on_up(&mut self) {
        self.music_list.previous();
    }

    pub fn on_down(&mut self) {
        self.music_list.next();
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            't' => {
                self.show_chart = !self.show_chart;
            }
            'p' => {
                if self.audio_player.is_paly_music() {
                    self.audio_player.pause_music();
                } else {
                    self.audio_player.play_music();
                }
            }
            '-' => {
                let mut volume = self.audio_player.get_music_volume();
                if volume > 0.02 {
                    if (volume - 0.02) < 0.01 {
                        volume = 0.0;
                    } else {
                        volume -= 0.02;
                    }
                } else {
                    if (0.02 - volume) < 0.01 {
                        volume = 0.0;
                    }
                }
                self.audio_player.set_music_volume(volume);
            }
            '+' => {
                let mut volume = self.audio_player.get_music_volume();
                if volume < 0.98 {
                    if (0.98 - volume) < 0.01 {
                        volume = 1.0;
                    } else {
                        volume += 0.02;
                    }
                } else {
                    if (volume - 0.98) < 0.01 {
                        volume = 1.0;
                    }
                }
                self.audio_player.set_music_volume(volume);
            }
            '\n' => {
                let update_playlist = match self.music_list.state.selected() {
                    Some(selected_index) => {
                        match self.cur_music {
                            Some(cur_index) => {
                                if selected_index != cur_index {
                                    self.audio_player.refresh_progress();
                                    true
                                } else {
                                    false
                                }
                            },
                            None => true,
                        }
                    },
                    None => false,
                };
                self.cur_music = self.music_list.state.selected();
                match self.cur_music {
                    Some(index) => {
                        if update_playlist {
                            self.audio_player.stop_music();
                            self.audio_player.clear_music_src_list();
                            for i in index..self.music_list.items.len() {
                                let music_src = self.music_list.items[i].get_source();
                                self.audio_player.append_music(music_src);
                            }
                            self.audio_player.music2sink();
                            self.audio_player.play_music();
                        }
                    },
                    None => {},
                }
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        // Update progress
        self.progress += 0.001;
        if self.progress > 1.0 {
            self.progress = 0.0;
        }

        self.sparkline.on_tick();
        self.signals.on_tick();

        let event = self.barchart.pop().unwrap();
        self.barchart.insert(0, event);
    }
}