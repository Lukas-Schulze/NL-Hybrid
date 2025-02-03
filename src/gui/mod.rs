use iced::{Application, Settings, Command};
use crate::fortnite_api::FortniteAPI;
use crate::config::UserConfig;

pub struct Interface {
    api_client: FortniteAPI,
    config: UserConfig,
}

#[derive(Debug, Clone)]
pub enum Message {
    RefreshItems,
    ToggleSkin(String),
    SaveConfig,
}

impl Application for Interface {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = (FortniteAPI, UserConfig);

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self {
            api_client: flags.0,
            config: flags.1,
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("NL Hybrid")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::RefreshItems => self.api_client.fetch_items(),
            Message::ToggleSkin(skin_id) => self.api_client.toggle_item(skin_id),
            Message::SaveConfig => self.config.save(),
        }
        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        iced::Column::new()
            .push(iced::Text::new("Cosmetics Manager").size(24))
            .into()
    }

    pub async fn run(api_client: FortniteAPI, config: UserConfig) -> Result<(), iced::Error> {
        Interface::run(Settings::with_flags((api_client, config)))
    }
}