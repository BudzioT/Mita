use std::sync::{Arc, Mutex};
use cpal;
use cpal::Stream;
use cpal::traits::{DeviceTrait, HostTrait};
use vosk;
use vosk::{Model, Recognizer};

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

        let speech_model = Model::new(model_path).expect("Failed to get the model!");
        let recognizer: Arc<Mutex<Recognizer>> = Arc::new(Mutex::new(Recognizer::new(&speech_model, 16000.0).expect("Couldn't create recognizer")));

        Self {
            model_path: model_path.to_string(),
            model: speech_model,
            host,
            device,
            config,
            recognizer,
        }
    }

    pub fn start_stream(self: Self) -> Stream {
        let recognizer = self.recognizer.clone();

        let stream = self.device.build_input_stream(
            &self.config.config(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let samples: Vec<i16> = data.iter()
                    .map(|&x| (x * 32768.0) as i16)
                    .collect();

                if let Ok(mut rec) = recognizer.lock() {
                    rec.accept_waveform(&samples).expect("Couldn't accept waveform");

                    let partial = rec.partial_result();
                    if !partial.partial.is_empty() {
                        println!("Partial {}", partial.partial);
                    }
                    let result = rec.final_result();

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