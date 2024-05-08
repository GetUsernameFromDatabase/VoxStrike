use crate::{audio::VoxStream, speech_to_text::PROMPT_REGEX};
use cpal::traits::DeviceTrait;
use log::{info, warn};
use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

mod audio;
mod inputbot_patch;
mod profiles;
mod settings;
mod speech_to_text;

pub fn main() {
    let args = settings::CommandArguments::new();
    settings::init_logger().expect("logger failed to initialize");
    speech_to_text::load(&args.model_path);

    let vox_audio = Arc::new(audio::VoxAudio::new(&args));
    let input_config = vox_audio.input_stream_config();
    let key_delay = Duration::from_millis(args.key_delay);
    // toml profile config
    let config = Arc::new(Mutex::new(profiles::Config::new(&args)));
    let record_keybind = config.lock().unwrap().profile.record_keybind;

    info!(
        "using input device: {:?}",
        vox_audio
            .input_device
            .name()
            .unwrap_or("NO_NAME_FOUND".to_string())
    );
    info!("using input config: {:?}", input_config);
    // this just here to seperate better in console
    println!();

    // -------------------------------------------------------------------------

    let stream: Arc<Mutex<Option<VoxStream>>> = Arc::new(Mutex::new(None));
    let vox1 = vox_audio.clone();
    record_keybind.bind(move || {
        let this = record_keybind;
        let stream_binding = stream.clone();
        let mut local_stream = stream_binding.lock().expect("could not lock mutex");

        // due to inputbot weirdness -- after `while this.is_pressed()` is falsey and lock is released
        // then loads of callbacks that were blocked orsm come rushing here
        if !this.is_pressed() {
            return;
        }

        if local_stream.is_none() {
            info!("[RECORDING] starting new audio input stream");
            local_stream.replace(vox1.new_stream(true));
        }
        while this.is_pressed() {
            sleep(Duration::from_millis(50));
        }

        // this could be changed to bind_release which is only on windows
        let local_stream = local_stream.take();
        if local_stream.is_none() {
            warn!("[RECORDING] could not get local stream");
            return;
        }

        let local_config = config.lock().unwrap();
        let stream_result = local_stream.unwrap().finish_stream(
            &local_config.profile.whisper.initial_prompt,
            input_config.channels(),
        );
        info!("[RECORDING] stream result: {}", stream_result);

        let rgx = PROMPT_REGEX.get().expect("regex required");
        let processed_result = rgx.replace_all(&stream_result, "").to_lowercase();
        let command = local_config.get_command(&processed_result);
        match command {
            None => info!("[ACTION] no command found with {}", processed_result),
            Some(c) => {
                info!("[ACTION] executing command '{}'", c.name);
                c.execute(key_delay);
                info!("[ACTION] command finished")
            }
        }
    });

    inputbot::handle_input_events(false);
}
