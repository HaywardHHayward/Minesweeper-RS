use clap::{Args, Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[arg(value_enum)]
    difficulty: Option<PresetDifficulty>,
}

impl Cli {
    pub fn get_preset_difficulty(&self) -> Option<PresetDifficulty> {
        self.difficulty.clone()
    }
}

#[derive(ValueEnum, Debug, Clone)]
pub enum PresetDifficulty {
    Beginner,
    Intermediate,
    Expert,
}
