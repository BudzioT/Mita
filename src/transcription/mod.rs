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
    recognizer: Recognizer,
}

impl Transcription {
    pub fn new(model_path: String) -> Self {
        let host = cpal::default_host();
        let device = host.default_input_device()
            .expect("No default input device found!");
        let mut configs_range = device.supported_input_configs()
            .expect("Input device couldn't provide supported configs!");
        let config = configs_range.next()
            .expect("No supported config exists for the input!")
            .with_max_sample_rate();

        let speech_model = Model::new(model_path.as_str()).expect("Failed to get the model!");
        let mut recognizer: Recognizer = Recognizer::new(&speech_model, 16000.0).unwrap();

        Self {
            model_path,
            model: speech_model,
            host,
            device,
            config,
            recognizer,
        }
    }

    pub fn start_stream(self: Self) -> Stream {
        let stream = self.device.build_output_stream(
            &self.config.config(),
            move |data: &mut[f32], _: &cpal::OutputCallbackInfo| {

            },
            move |error| {

            },
            None,
        );

        stream.expect("Couldn't initialize stream")
    }
}