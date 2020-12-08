//! Audio Player Implementation
//! 

use rodio::{
    source::Buffered,
    Decoder,
    Sink,
    OutputStream,
    OutputStreamHandle,
};
use std::io::BufReader;
use std::fs::File;
use std::time::Instant;

/// AudioPlayer Struct with sourcelist and sinklist
pub struct AudioPlayer {
    music_src_list: Vec<Buffered<Decoder<BufReader<File>>>>,
    _piano_src_list: Vec<Buffered<Decoder<BufReader<File>>>>,
    sink_list: Vec<Sink>,
    output_stream: (OutputStream, OutputStreamHandle),
    pre_time: Instant,
    progress: f64,
}

impl AudioPlayer {
    pub fn new(sink_num: usize) -> Result<Self, ()> {
        if sink_num <= 0 {
            return Err(());
        }
        let music_src_list = Vec::new();
        let _piano_src_list = Vec::new();
        let mut sink_list = Vec::new();
        let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        for _ in 0..sink_num {
            let sink = rodio::Sink::try_new(&stream_handle).unwrap();
            sink.set_volume(0.2);
            sink_list.push(sink);
        }
        Ok(
            Self {
                music_src_list,
                _piano_src_list,
                sink_list,
                output_stream: (stream, stream_handle),
                pre_time: Instant::now(),
                progress: 0.0,
            }
        )
    }

    /// add a source to source list
    pub fn append_music(&mut self, source: Buffered<Decoder<BufReader<File>>>) {
        self.music_src_list.push(source);
    }

    /// push a music source to sink_list[0]
    pub fn music2sink(&mut self) {
        for _ in 0..self.music_src_list.len() {
            let music_src = self.music_src_list.remove(0);
            self.sink_list[0].append(music_src);
        }
    }

    /// play music
    pub fn play_music(&mut self) {
        self.pre_time = Instant::now();
        self.sink_list[0].play();
        
    }
    
    /// pause music
    pub fn pause_music(&self) {
        self.sink_list[0].pause();
    }

    /// stop music
    pub fn stop_music(&mut self) {
        self.sink_list[0].stop();
        drop(&self.sink_list[0]);
        self.sink_list[0] = rodio::Sink::try_new(&self.output_stream.1).unwrap();
        self.sink_list[0].set_volume(0.2);
        self.pre_time = Instant::now();
    }

    /// sleep until end of play
    #[allow(dead_code)]
    pub fn sleep_until_end(&self, sink_index: usize) {
        if sink_index >= self.sink_list.len() {
            panic!("index out of bound!");
        }
        self.sink_list[sink_index].sleep_until_end();
    }

    pub fn clear_music_src_list(&mut self) {
        self.music_src_list.clear();
    }

    pub fn is_paly_music(&self) -> bool {
        !self.sink_list[0].is_paused()
    }

    pub fn set_music_volume(&mut self, value: f32) {
        self.sink_list[0].set_volume(value);
    }

    pub fn get_music_volume(&self) -> f32 {
        self.sink_list[0].volume()
    }

    pub fn progress(&mut self) -> f64 {
        if self.is_paly_music() {
            self.progress += self.pre_time.elapsed().as_micros() as f64;
        }
        self.pre_time = Instant::now();
        self.progress
    }

    pub fn refresh_progress(&mut self) {
        self.progress = 0.0;
    }
}