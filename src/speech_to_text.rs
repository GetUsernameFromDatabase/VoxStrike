/// copied and slightly modified from
/// https://github.com/scripty-bot/stt-service/blob/53b688bf58ea31b566e250a4a32110403c93a9bf/stts_speech_to_text/src/lib.rs
use log::{error, info};
use parking_lot::Mutex;
use regex::Regex;
use std::{fmt::Write, sync::OnceLock, time::Instant};
pub use whisper_rs::*;

pub static MODEL: OnceLock<WhisperContext> = OnceLock::new();
/// regex to unify prompt and command
pub static PROMPT_REGEX: OnceLock<Regex> = OnceLock::new();

pub fn load(model_path: &str) {
    info!("attempting to load model");
    let model = WhisperContext::new_with_params(model_path, WhisperContextParameters::new())
        .expect("failed to load model");
    MODEL.set(model).expect("failed to set models");
    info!("loaded model");

    PROMPT_REGEX
        .set(Regex::new(r"[\W]+").expect("regex required"))
        .expect("failed to set prompt regular expression");
}

fn get_new_model() -> Option<WhisperState<'static>> {
    // if we got a model, return it
    // on error, log it and return None
    match MODEL.get().map(|ctx| ctx.create_state()) {
        Some(Ok(state)) => Some(state),
        Some(Err(e)) => {
            error!("failed to create model: {:?}", e);
            None
        }
        None => {
            error!("models not set up yet: check that load_model was called");
            None
        }
    }
}

fn create_model_params(initial_prompt: &str) -> FullParams<'_, '_> {
    // whisper parameters
    let mut wp = FullParams::new(SamplingStrategy::BeamSearch {
        beam_size: 5,
        patience: -1.0,
    });
    wp.set_initial_prompt(initial_prompt);

    // params.set_n_threads(1);
    wp.set_language(Some("en"));
    wp.set_suppress_non_speech_tokens(true);
    // params.set_no_context(false);
    // since this is used for voice commands
    wp.set_single_segment(true);
    wp.set_no_context(true);

    wp.set_print_progress(false);
    wp.set_print_realtime(false);
    wp.set_print_timestamps(false);

    // wp.set_temperature(0.4);
    // wp.set_temperature_inc(1.0);

    return wp;
}

// -----------------------------------------------------------------------------

pub struct StreamFinishProperties<'a> {
    pub verbose: bool,
    pub initial_prompt: &'a str,
    /// the amount of times to call whisper_rs::convert_stereo_to_mono_audio
    pub halver_count: u16,
}

/// A wrapper around a Stream that holds the Stream on one thread constantly.
pub struct SttStreamingState {
    stream_data: Mutex<Vec<f32>>,
    last_access: Mutex<Instant>,
}

impl Default for SttStreamingState {
    fn default() -> Self {
        Self::new()
    }
}

impl SttStreamingState {
    pub fn new() -> Self {
        Self {
            stream_data: Mutex::new(Vec::new()),
            last_access: Mutex::new(Instant::now()),
        }
    }

    pub fn feed_audio(&self, mut audio: Vec<f32>) {
        self.stream_data.lock().append(&mut audio);
        self.update_last_access();
    }

    pub fn finish_stream(self, properties: StreamFinishProperties) -> Result<String, WhisperError> {
        let Self {
            stream_data,
            last_access: _,
        } = self;

        // we own the stream data now, so we can drop the lock
        let mut audio_data = stream_data.into_inner();
        if audio_data.is_empty() {
            return Ok(String::new());
        }

        // TODO: optimize this, maybe use the code inside
        for _i in 0..properties.halver_count {
            audio_data = whisper_rs::convert_stereo_to_mono_audio(&audio_data)
                .expect("failed to convert audio data");
        }

        let params = create_model_params(properties.initial_prompt);

        // get a model from the pool
        let mut state = get_new_model().expect("failed to get model from pool");

        // run the model
        let res = state.full(params, &audio_data);

        // check if the model failed
        if let Err(e) = res {
            error!("model failed: {:?}", e);
            return Err(e);
        }

        // get the result
        let num_segments: i32 = state.full_n_segments()?;
        // average english word length is 5.1 characters which we round up to 6
        let mut segments = String::with_capacity(6 * num_segments as usize);
        for i in 0..num_segments {
            match (state.full_get_segment_text(i), properties.verbose) {
                (Ok(s), false) => {
                    segments.push_str(&s);
                    if i < num_segments - 1 {
                        segments.push('\n');
                    }
                }
                (Ok(s), true) => {
                    // also add the start and end time
                    let start = state.full_get_segment_t0(i)?;
                    let end = state.full_get_segment_t1(i)?;
                    writeln!(segments, "[{} - {}]: {}", start, end, s)
                        .expect("failed to write segment");
                }
                (Err(e), _) => {
                    error!("failed to get segment text: {:?}", e);
                    return Err(e);
                }
            };
        }

        return Ok(segments);
    }

    fn update_last_access(&self) {
        let mut last_access = self.last_access.lock();
        *last_access = Instant::now();
    }

    // pub fn get_last_access(&self) -> Instant {
    //     *self.last_access.lock()
    // }
}
