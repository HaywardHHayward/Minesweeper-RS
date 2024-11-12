use std::collections::HashMap;

use iced::{widget::center, *};

mod game_screen;
mod start_screen;

trait Screen {
    type ScreenMessage;
    type ScreenAction;
    fn update(&mut self, message: Self::ScreenMessage) -> Self::ScreenAction;
    fn view(&self) -> Element<Self::ScreenMessage>;
}

enum ScreenData {
    Game(game_screen::GameScreen),
    Start(start_screen::StartScreen),
}

impl ScreenData {
    fn view(&self) -> Element<Message> {
        match self {
            Self::Game(game_screen) => game_screen.view().map(Message::Game),
            Self::Start(start_screen) => start_screen.view().map(Message::Start),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum ScreenChoice {
    Game,
    Start,
}

impl From<&ScreenData> for ScreenChoice {
    fn from(screen_data: &ScreenData) -> Self {
        match screen_data {
            ScreenData::Game(_) => Self::Game,
            ScreenData::Start(_) => Self::Start,
        }
    }
}

enum TimerStatus {
    Running,
    Stopped,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Game(game_screen::Message),
    Start(start_screen::Message),
    Exit,
}

pub struct MinesweeperApp {
    screens: HashMap<ScreenChoice, ScreenData>,
    current_screen: ScreenChoice,
    timer_status: TimerStatus,
}

impl Default for MinesweeperApp {
    fn default() -> Self {
        let mut screens = HashMap::with_capacity(2);
        screens.insert(
            ScreenChoice::Start,
            ScreenData::Start(start_screen::StartScreen::new()),
        );
        Self {
            screens,
            current_screen: ScreenChoice::Start,
            timer_status: TimerStatus::Stopped,
        }
    }
}

impl MinesweeperApp {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        assert!(
            self.screens.contains_key(&self.current_screen),
            "Current screen does not exist!"
        );
        match message {
            Message::Game(message) => {
                let ScreenData::Game(game_screen) =
                    self.screens.get_mut(&self.current_screen).unwrap()
                else {
                    return Task::none();
                };
                let action = game_screen.update(message);
                match action {
                    game_screen::Action::None => Task::none(),
                }
            }
            Message::Start(message) => {
                let ScreenData::Start(start_screen) =
                    self.screens.get_mut(&self.current_screen).unwrap()
                else {
                    return Task::none();
                };
                let action = start_screen.update(message);
                match action {
                    start_screen::Action::None => Task::none(),
                    start_screen::Action::MakeBoard {
                        width,
                        height,
                        mines,
                    } => {
                        self.screens.insert(
                            ScreenChoice::Game,
                            ScreenData::Game(game_screen::GameScreen::new(width, height, mines)),
                        );
                        self.current_screen = ScreenChoice::Game;
                        self.timer_status = TimerStatus::Running;
                        Task::done(Message::Game(game_screen::Message::CurrentTime(
                            time::Instant::now(),
                        )))
                    }
                }
            }
            Message::Exit => exit(),
        }
    }
    pub fn view(&self) -> Element<Message> {
        self.screens.get(&self.current_screen).map_or(
            center(widget::Text::new(
                "Error! Current screen does not exist. If you are seeing this, something has gone wrong!",
            )).into(),
            |screen| screen.view(),
        )
    }
    pub fn subscription(&self) -> Subscription<Message> {
        let subscription_batch = vec![
            // Active when timer is active
            match self.timer_status {
                TimerStatus::Running => Some(
                    time::every(time::Duration::from_secs(1))
                        .map(|s| Message::Game(game_screen::Message::CurrentTime(s))),
                ),
                TimerStatus::Stopped => None,
            },
            // Exits when ESC is pressed
            Option::from(keyboard::on_key_press(|key, _modifier| {
                if let keyboard::Key::Named(keyboard::key::Named::Escape) = key {
                    Some(Message::Exit)
                } else {
                    None
                }
            })),
        ];
        Subscription::batch(subscription_batch.into_iter().flatten().collect::<Vec<_>>())
    }
}
