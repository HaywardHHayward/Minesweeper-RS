use std::num::{NonZeroU8, NonZeroU16};

use clap::{Args, Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, next_line_help = true)]
pub struct Cli {
    #[arg(
        value_enum,
        help = "Sets a preset difficulty level, or 'custom' to specify the settings for a custom game."
    )]
    difficulty: Option<PresetDifficulty>,
    #[arg(
        required_if_eq("difficulty", "custom"),
        help = "Width of the custom game board. Must be less than 256 and greater than 0. The 'custom' difficulty must be selected to use this option."
    )]
    width: Option<NonZeroU8>,
    #[arg(
        required_if_eq("difficulty", "custom"),
        help = "Height of the custom game board. Must be less than 256 and greater than 0. The 'custom' difficulty must be selected to use this option."
    )]
    height: Option<NonZeroU8>,
    #[arg(
        required_if_eq("difficulty", "custom"),
        help = "The number of mines in the custom game board. Must be less than 65536 and greater than 0. Additionally, must be less than WIDTH * HEIGHT. The 'custom' difficulty must be selected to use this option."
    )]
    mines: Option<NonZeroU16>,
}

#[derive(Args, Debug, Clone)]
struct CustomSettings {
    width: NonZeroU8,
    height: NonZeroU8,
    mines: NonZeroU16,
}

impl Cli {
    pub fn get_preset_difficulty(&self) -> Option<PresetDifficulty> {
        self.difficulty.clone()
    }
    pub fn get_custom_settings(&self) -> Option<(NonZeroU8, NonZeroU8, NonZeroU16)> {
        if !matches!(self.difficulty, Some(PresetDifficulty::Custom)) {
            return None;
        }
        let width = self.width?;
        let height = self.height?;
        let mines = self.mines?;
        Some((width, height, mines))
    }
}

#[derive(ValueEnum, Debug, Clone)]
pub enum PresetDifficulty {
    Beginner,
    Intermediate,
    Expert,
    Custom,
}
