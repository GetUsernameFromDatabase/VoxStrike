use clap::Parser;

// TODO: make this parse and save settings for the user (when GUI is made)
//  - the profile that the user last used
//  - model that is used
//  - audio input device (microphone)

pub fn init_logger() -> anyhow::Result<()> {
    // TODO: create default logger file if does not exist
    return log4rs::init_file("log4rs.yml", Default::default());
}

// -----------------------------------------------------------------------------

#[derive(Parser, Debug)]
#[command(version, about = "VoxStrike command arguments", long_about = None)]
pub struct CommandArguments {
    // could be useful when writing linux support
    // https://github.com/RustAudio/cpal/blob/master/examples/record_wav.rs#L31
    //
    /// Name of the audio input device to use
    #[arg(short, long, default_value_t = String::from("default"))]
    pub audio_in: String,

    /// Path to whisper model
    /// see https://github.com/ggerganov/whisper.cpp/blob/master/models/README.md
    #[arg(short, long, default_value_t = String::from("ggml-base.en.bin"))]
    pub model_path: String,

    #[arg(short, long, default_value_t = String::from("profiles/helldivers2.toml"))]
    pub profile_path: String,

    /// The delay between each key press in `profiles.commands[i].action`\
    /// in milliseconds
    #[arg(short, long, default_value_t = 50)]
    pub key_delay: u64,
}

impl CommandArguments {
    pub fn new() -> Self {
        return Self::parse();
    }
}
