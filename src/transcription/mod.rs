use std::sync::{Arc, Mutex};
use cpal;
use cpal::Stream;
use cpal::traits::{DeviceTrait, HostTrait};
use vosk;
use vosk::{Model, Recognizer};

// Struct to magically turn voice into text
pub struct Transcription {
    model_path: String,
    model: Model,
    host: cpal::Host,
    device: cpal::Device,
    config: cpal::SupportedStreamConfig,
    recognizer: Arc<Mutex<Recognizer>>,
}

impl Transcription {
    pub fn new(model_path: &str) -> Self {
        let host = cpal::default_host();
        let device = host.default_input_device()
            .expect("No default input device found!");

        /*
        let mut configs_range = device.supported_input_configs()
            .expect("Input device couldn't provide supported configs!");
        let config = configs_range.next()
            .expect("No supported config exists for the input!")
            .with_max_sample_rate();
         */

        /*
        // Get config of your device, try to get mono channel one if possible
        // Grab the one with max sample rate, 16k seems to fail
        let config = match device.supported_input_configs()
            .expect("Input device with correct config not found")
            .find(|config| config.channels() == 1)
        {
            Some(mono_config) => mono_config.with_sample_rate(cpal::SampleRate(16000)),
            None => {
                let default_config = device.supported_input_configs()
                    .expect("Range of supported configs not found")
                    .next()
                    .expect("No supported config found")
                    .with_max_sample_rate();

                default_config
            },
        };
        */

        // Print all available configurations for debugging
        println!("Available input configurations:");
        for config in device.supported_input_configs()
            .expect("Error getting supported configs")
        {
            println!("  Channels: {}", config.channels());
            println!("  Sample rate range: {} - {} Hz",
                     config.min_sample_rate().0,
                     config.max_sample_rate().0);
            println!("  Sample format: {:?}", config.sample_format());
            println!("---");
        }

        // Try to find a working configuration, being more flexible
        let config = device.supported_input_configs()
            .expect("Error getting supported configs")
            .find(|config| {
                // Accept any number of channels and any sample format
                config.min_sample_rate().0 <= 16000 &&
                    config.max_sample_rate().0 >= 16000
            })
            .map(|config| config.with_sample_rate(cpal::SampleRate(16000)))
            .unwrap_or_else(|| {
                // Fallback: just get the default config
                println!("Couldn't find ideal 16kHz config, falling back to default");
                device.supported_input_configs()
                    .expect("Error getting supported configs")
                    .next()
                    .expect("No input configs available")
                    .with_sample_rate(cpal::SampleRate(44100))
            });

        println!("Selected configuration:");
        println!("  Channels: {}", config.channels());
        println!("  Sample rate: {} Hz", config.sample_rate().0);
        println!("  Sample format: {:?}", config.sample_format());

        let speech_model = Model::new(model_path).expect("Failed to get the model!");
        let recognizer: Arc<Mutex<Recognizer>> = Arc::new(Mutex::new(Recognizer::new(&speech_model, 16000.0).expect("Couldn't create recognizer")));

        // Set words configuration to get more partial results
        recognizer.lock().unwrap().set_words(true);
        recognizer.lock().unwrap().set_max_alternatives(3);

        Self {
            model_path: model_path.to_string(),
            model: speech_model,
            host,
            device,
            config,
            recognizer,
        }
    }

    // Start invading privacy and listening to your microphone, convert speech to text
    pub fn start_stream(self: Self) -> Stream {
        let recognizer = self.recognizer.clone();
        let mut buffer = Vec::new();
        let sample_rate = self.config.sample_rate().0;

        let channels = self.config.channels() as usize;

        // Add volume monitoring
        let mut max_volume = 0.0f32;
        let mut samples_since_last_print = 0;

        let stream = self.device.build_input_stream(
            &self.config.config(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let current_max = data.iter().map(|&x| x.abs()).fold(0.0f32, f32::max);
                max_volume = max_volume.max(current_max);
                samples_since_last_print += data.len();

                // Print volume levels every second
                if samples_since_last_print >= sample_rate as usize {
                    println!("Current max volume: {:.2}", max_volume);
                    max_volume = 0.0;
                    samples_since_last_print = 0;
                }

                // Just convert samples data f32 to i16
                let samples: Vec<i16> = if channels > 1 {
                    data.chunks(channels)
                        .map(|chunk| {
                            let avg = chunk.iter().sum::<f32>() / channels as f32;
                            // Amplify the signal slightly
                            ((avg * 1.5).max(-1.0).min(1.0) * 32767.0) as i16
                        })
                        .collect()
                } else {
                    data.iter()
                        .map(|&x| ((x * 1.5).max(-1.0).min(1.0) * 32767.0) as i16)
                        .collect()
                };

                buffer.extend(samples);

                if buffer.len() >= sample_rate as usize / 4 {
                    // Get the recognizer and grab results for some time
                    if let Ok(mut rec) = recognizer.lock() {
                        if let Err(e) = rec.accept_waveform(&buffer) {
                            eprintln!("Error processing waveform: {}", e);
                        }

                        let partial = rec.partial_result();
                        if !partial.partial.is_empty() {
                            println!("Partial {}", partial.partial);
                        }
                        let result = rec.final_result();
                        //println!("{:?}", result);

                        // Not sure what's the difference between single and multiple results of text
                        // But just use them
                        match result {
                            vosk::CompleteResult::Single(single) => {
                                if !single.text.is_empty() {
                                    println!("Final (single): {}", single.text);
                                }
                            }

                            vosk::CompleteResult::Multiple(multiple) => {
                                for result in multiple.alternatives.iter() {
                                    println!("Final (multiple): {} (conf: {})", result.text, result.confidence);
                                }
                            }
                        }

                        buffer.clear();
                    }
                }
            },
            move |error| {
                eprintln!("Error on audio stream: {}", error);
            },
            None,
        ).expect("Failed to build input stream");

        stream
    }

    pub fn toggle_mute() {

    }
}