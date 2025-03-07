use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AudioType {
    Music,
    Sound,
    UI,
}

pub struct AudioQueueItem {
    name: String,
    audio_type: AudioType,
    volume: f32,
    looped: bool,
}

pub struct AudioManager {
    sounds: RwLock<HashMap<String, Vec<u8>>>,  // Store audio data in memory
    audio_queue: RwLock<VecDeque<AudioQueueItem>>,     // Queue for sounds to be played
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    // I really should set up a proper mixing solution but this is good enough for now.
    music_sinks: Vec<Arc<Sink>>, // 2 music sinks (or otherwise "Loop" sinks. You can loop any audio type for the sake of freedom, but I'd recommend doing it in these because stopping all music will be less abrupt than stopping all sounds, and music is probably what you are looping anyway)
    sound_sinks: Vec<Arc<Sink>>, // 16 sound sinks (for common sounds that can be dropped without much consequence if too many sounds are playing)
    ui_sinks: Vec<Arc<Sink>>, // 4 UI sinks (or otherwise "Priority" sinks, to be used sparingly for sounds that should never be dropped)
}

impl AudioManager {
    pub fn new() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().expect("Failed to create audio stream");
        
        let music_sinks = (0..2).map(|_| Arc::new(Sink::try_new(&stream_handle).unwrap())).collect();
        let sound_sinks = (0..16).map(|_| Arc::new(Sink::try_new(&stream_handle).unwrap())).collect();
        let ui_sinks = (0..4).map(|_| Arc::new(Sink::try_new(&stream_handle).unwrap())).collect();
        
        AudioManager {
            sounds: RwLock::new(HashMap::new()),
            audio_queue: RwLock::new(VecDeque::new()),
            _stream: stream,
            stream_handle,
            music_sinks,
            sound_sinks,
            ui_sinks,
        }
    }

    // Enqueue a audio for playback
    pub fn enqueue_audio(&self, name: &str, audio_type: AudioType, volume: f32, looped: bool) {
        let mut queue = self.audio_queue.write().unwrap();
        queue.push_back(AudioQueueItem {
            name: name.to_string(),
            audio_type,
            volume,
            looped,
        });
    }

    // Process and play all audio in the queue
    pub fn process_audio_queue(&self) -> Result<(), String> {
        let mut queue = self.audio_queue.write().unwrap();

        while let Some(item) = queue.pop_front() {
            self.play_sound(&item)?;
            println!("Audio: {}", item.name);
        }

        Ok(())
    }

    // Play the sound
    pub fn play_sound(&self, item: &AudioQueueItem) -> Result<(), String> {
        let sounds = self.sounds.read().unwrap();
        let sound_data = sounds.get(&item.name).ok_or("Sound not found".to_string())?;
        let cursor = std::io::Cursor::new(sound_data.clone());
        let source = Decoder::new(BufReader::new(cursor)).map_err(|_| "Failed to decode audio".to_string())?;

        let sink = match item.audio_type {
            AudioType::Music => self.music_sinks.iter().find(|s| s.empty()).cloned(),
            AudioType::Sound => self.sound_sinks.iter().find(|s| s.empty()).cloned(),
            AudioType::UI => self.ui_sinks.iter().find(|s| s.empty()).cloned(),
        };

        // If no sinks are available for that sound type, we will just not play the audio.
        let Some(sink) = sink else { return Ok(()); };
        
        sink.set_volume(item.volume);
        if item.looped {
            sink.append(source.repeat_infinite());
        } 
        else {
            sink.append(source);
        }
        
        Ok(())
    }

    pub fn stop_audio(&self) {
        self.stop_music_sinks();
        self.stop_sound_sinks();
        self.stop_ui_sinks();
    }

    pub fn stop_music_sinks(&self) {
        for sink in &self.music_sinks {
            sink.stop();
        }
    }

    pub fn stop_sound_sinks(&self) {
        for sink in &self.sound_sinks {
            sink.stop();
        }
    }

    pub fn stop_ui_sinks(&self) {
        for sink in &self.ui_sinks {
            sink.stop();
        }
    }

    // Load a sound
    pub fn load_sound(&self, name: &str, path: &str) -> Result<(), String> {
        let mut sounds = self.sounds.write().unwrap();
        
        if sounds.contains_key(name) {
            return Ok(()); // Sound is already loaded
        }

        let file = File::open(path).map_err(|_| "Failed to open audio file".to_string())?;
        let mut buffer = Vec::new();
        BufReader::new(file).read_to_end(&mut buffer).map_err(|_| "Failed to read audio file".to_string())?;
        
        sounds.insert(name.to_string(), buffer);
        Ok(())
    }

    // Load sounds from directory
    pub fn load_sounds_from_directory(&self, dir_path: &str) -> Result<(), String> {
        let paths = std::fs::read_dir(dir_path).map_err(|_| "Failed to read directory".to_string())?;

        for path in paths {
            let entry = path.map_err(|_| "Failed to read directory entry".to_string())?;
            let file_name = entry.file_name().into_string().map_err(|_| "Invalid file name".to_string())?;
            let full_path = entry.path();

            if full_path.is_file() {
                if let Some(extension) = full_path.extension() {
                    if extension == "mp3" || extension == "wav" || extension == "flac" {
                        let name = file_name.trim_end_matches(".mp3").trim_end_matches(".wav").trim_end_matches(".flac");
                        self.load_sound(name, full_path.to_str().unwrap()).map_err(|e| format!("Error loading sound '{}': {}", name, e))?;
                    }
                }
            }
        }

        Ok(())
    }
}
