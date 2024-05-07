use inputbot::KeybdKey;
use regex::Regex;
use serde::Deserialize;
use std::{collections::HashMap, fs, time::Duration};

use crate::{
    inputbot_patch::KeySequence, settings::CommandArguments, speech_to_text::PROMPT_REGEX,
};

// -----------------------------------------------------------------------------
// --- PROFILE TOML ---
// -----------------------------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct Command {
    pub name: String,
    pub action: String,
    pub modifiers: Option<Vec<KeybdKey>>,
}

impl Command {
    pub fn execute(&self) {
        match &self.modifiers {
            None => None,
            Some(m) => Some({
                m.iter().for_each(|x| x.press());
            }),
        };

        let key_sequence = KeySequence(&self.action);
        key_sequence.send(Duration::from_millis(50));

        match &self.modifiers {
            None => None,
            Some(m) => Some({
                m.iter().for_each(|x| x.release());
            }),
        };
    }
}

#[derive(Deserialize, Debug)]
pub struct Whisper {
    pub initial_prompt: String,
}

#[derive(Deserialize, Debug)]
/// Profile for the commands
pub struct Profile {
    pub record_keybind: KeybdKey,
    pub commands: Vec<Command>,
    pub whisper: Whisper,
}

impl Profile {
    pub fn new(args: &CommandArguments) -> Self {
        let file_contents = fs::read_to_string(&args.profile_path).expect("Unable to read file");
        let mut parsed: Self = toml::from_str(&file_contents).expect("Unable to parse TOML");
        // TODO: enforce that parsed.commands[i].modifiers are unique
        //  goal is to remove duplicate "leftcommand" and the likes
        //  not really a priority however as I expect the user not to do that

        // not actually sure if the following is needed
        let nrgx = Regex::new(r"[\n]+").expect("regex required");
        parsed.whisper.initial_prompt = nrgx.replace_all(&parsed.whisper.initial_prompt, " ").to_string();
        
        return parsed;
    }
}

// -----------------------------------------------------------------------------
// --- WRAPPERS ---
// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Config {
    pub profile: Profile,
    /// Initial prompt for whisper
    // pub initial_prompt: String, // replaced by Profile.whisper.initial_prompt
    /// command map that contains indexes for profile.commands
    command_map: HashMap<String, usize>,
}

impl Config {
    pub fn new(args: &CommandArguments) -> Self {
        let profile = Profile::new(args);
        let mut command_map: HashMap<String, usize> = HashMap::new();

        // initialized with arbritary capacity
        // let mut initial_prompt = String::with_capacity(64);
        // initial_prompt.push_str("Glossary: ");

        let prompt_rgx = PROMPT_REGEX.get().expect("regex required");

        let commands_length = profile.commands.len();
        for command_index in 0..commands_length {
            let command = &profile.commands[command_index];

            // // TODO: add only unique words
            // initial_prompt.push_str(&command.name);
            // if command_index != commands_length - 1 {
            //     initial_prompt.push_str(", ")
            // }

            let processed_name = prompt_rgx.replace_all(&command.name, "").to_lowercase();
            command_map.insert(processed_name, command_index);
        }

        return Self {
            profile,
            // initial_prompt,
            command_map,
        };
    }

    pub fn get_command(&self, command_name: &str) -> Option<&Command> {
        // command index
        let index = self.command_map.get(command_name);
        match index {
            None => return None,
            Some(x) => return Some(&self.profile.commands[*x]),
        }
    }
}
