use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::time::{Duration, Instant};

pub struct AudioManager {
    pub _stream: OutputStream,
    pub music_sink: Sink,
    pub sfx_sink: Sink,
    pub running_sink: Sink,
    last_running_sound: Instant,
}

impl AudioManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let music_sink = Sink::try_new(&stream_handle)?;
        let sfx_sink = Sink::try_new(&stream_handle)?;
        let running_sink = Sink::try_new(&stream_handle)?;
        
        Ok(AudioManager {
            _stream,
            music_sink,
            sfx_sink,
            running_sink,
            last_running_sound: Instant::now(),
        })
    }
    
    pub fn play_background_music(&self) {
        if let Ok(file) = File::open("assets/sounds/music.mp3") {
            if let Ok(source) = Decoder::new(BufReader::new(file)) {
                self.music_sink.append(source);
                self.music_sink.set_volume(0.3);
                self.music_sink.play();
            }
        }
    }
    
    pub fn maintain_background_music(&self) {
        if self.music_sink.empty() {
            self.play_background_music();
        }
    }
    
    pub fn play_running_sound(&mut self) {
        if self.running_sink.empty() {
            match File::open("assets/sounds/running.mp3") {
                Ok(file) => {
                    match Decoder::new(BufReader::new(file)) {
                        Ok(source) => {
                            self.running_sink.append(source);
                            self.running_sink.set_volume(0.8);
                            self.running_sink.play();
                        },
                        Err(e) => println!("Error decodificando running.mp3: {}", e),
                    }
                },
                Err(e) => println!("Error abriendo running.mp3: {}", e),
            }
        }
    }
    
    pub fn stop_running_sound(&mut self) {
        //Detener el sonido de running cuando se deja de mover
        self.running_sink.stop();
    }
    
    pub fn play_start_sound(&self) {
        self.play_sound_effect("assets/sounds/start.mp3", 0.5);
    }
    
    pub fn play_win_sound(&self) {
        self.play_sound_effect("assets/sounds/win.mp3", 0.6);
    }
    
    pub fn play_up_down_sound(&self) {
        self.play_sound_effect("assets/sounds/up_down.mp3", 0.3);
    }
    
    pub fn play_menu_sound(&self) {
        self.play_sound_effect("assets/sounds/menu.mp3", 0.4);
    }
    
    fn play_sound_effect(&self, path: &str, volume: f32) {
        if let Ok(file) = File::open(path) {
            if let Ok(source) = Decoder::new(BufReader::new(file)) {
                //Limpiar efectos anteriores si están sonando
                self.sfx_sink.stop();
                self.sfx_sink.append(source);
                self.sfx_sink.set_volume(volume);
                self.sfx_sink.play();
            }
        }
    }
}
