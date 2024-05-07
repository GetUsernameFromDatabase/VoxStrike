use crate::settings::CommandArguments;
use crate::speech_to_text::{StreamFinishProperties, SttStreamingState};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Host, Stream, SupportedStreamConfig};
use log::{debug, error};
use std::sync::Arc;

pub fn new_host() -> Host {
    // could be useful when writing linux support
    // https://github.com/RustAudio/cpal/blob/master/examples/record_wav.rs#L49
    let host = cpal::default_host();
    return host;
}

pub fn new_input_device(
    host: &Host,
    desired_input_device: &String,
) -> Result<Device, anyhow::Error> {
    let device = if desired_input_device == "default" {
        host.default_input_device()
    } else {
        host.input_devices()?.find(|x| {
            x.name()
                .map(|y| y == *desired_input_device)
                .unwrap_or(false)
        })
    }
    .expect("failed to find input device");
    return Ok(device);
}

// -----------------------------------------------------------------------------

pub struct VoxStream {
    pub stt: Arc<SttStreamingState>,
    pub audio_in: Stream,
}

impl VoxStream {
    /// this will drop audio_in
    pub fn finish_stream(self, initial_prompt: &str, stereo_to_string_conversions: u16) -> String {
        debug!("[VoxStream] finishing stream");
        drop(self.audio_in);
        let stream = Arc::into_inner(self.stt).expect("SttStreamingState required");

        return stream
            .finish_stream(StreamFinishProperties {
                verbose: false,
                initial_prompt,
                halver_count: stereo_to_string_conversions,
            })
            .unwrap_or("".to_string());
    }
}
/// my inexperience will most likely bite me for this
unsafe impl Send for VoxStream {}

// -----------------------------------------------------------------------------

pub struct VoxAudio {
    // host: Host,
    pub input_device: Device,
}

impl VoxAudio {
    pub fn new(args: &CommandArguments) -> Self {
        let host = new_host();
        let input_device =
            new_input_device(&host, &args.audio_in).expect("Audio input device needed");

        return Self { input_device };
    }

    pub fn input_stream_config(&self) -> SupportedStreamConfig {
        let config = self
            .input_device
            .default_input_config()
            .expect("failed to get default input supported stream config");
        return config;
    }

    pub fn new_stream(&self, start_play: bool) -> VoxStream {
        debug!("[VoxAudio] creating new stream");
        let stt_stream = Arc::new(SttStreamingState::new());

        let data_handler_stream = stt_stream.clone();
        let data_handler = move |data: &[f32], _: &_| data_handler_stream.feed_audio(data.to_vec());

        let input_stream: Stream = self
            .new_input_stream(data_handler)
            .expect("input stream required");

        if start_play {
            debug!("[VoxAudio] starting stream");
            input_stream.play().expect("play to work");
        }

        return VoxStream {
            stt: stt_stream,
            audio_in: input_stream,
        };
    }

    fn new_input_stream(
        &self,
        data_handler: impl FnMut(&[f32], &cpal::InputCallbackInfo) + std::marker::Send + 'static,
    ) -> Result<Stream, anyhow::Error> {
        let config: cpal::StreamConfig = self.input_stream_config().into();
        let input_device = &self.input_device;

        let err_fn = move |err| {
            error!("an error occurred on stream: {}", err);
        };

        let stream = input_device.build_input_stream(&config, data_handler, err_fn, None)?;
        return Ok(stream);
    }
}
