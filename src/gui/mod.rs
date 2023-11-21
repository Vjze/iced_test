use self::{
    bar::{Bar, Message as BarMessage},
    left_view::{Leftview, Message as LeftMessage},
    styles::theme::TroxideTheme,
};

use crate::gui::styles::container_styles::release_time_container_theme;
use crate::gui::views::other::{login::login, tip::tip};
use crate::util::sql::login_user;
use assets::fonts::FONT;
use iced::widget::row;
use iced::{executor, widget::container, Application, Command, Length, Renderer, Subscription, window};

pub mod assets;
pub mod bar;
// pub mod foot;
pub mod left_view;
pub mod styles;
pub mod views;
#[derive(Debug, Clone, Copy, Default)]
pub enum TipType {
    #[default]
    EmptyTip,
    LoginTip,
}
pub struct MyTools {
    bar: Bar,
    leftwidget: Leftview,
    pub now: time::OffsetDateTime,
    pub runtime: f32,
    logined: bool,
    name: String,
    password: String,
    pub tip_type: TipType,
    show_modal:bool,
}

#[derive(Clone, Debug)]
pub enum Message {
    BarMessage(BarMessage),
    Datetime(time::OffsetDateTime),
    FontLoaded(Result<(), iced::font::Error>),
    LeftMessage(LeftMessage),
    Namechange(String),
    Passchange(String),
    OpenModal,
    CloseModal,
    LoginCence,
    Logingpre,
    Loginbool(bool),
}

impl Application for MyTools {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let font_command = iced::font::load(FONT);
        (
            Self {
                leftwidget: Leftview::new(),
                bar: Bar::new(),
                now: time::OffsetDateTime::now_local()
                    .unwrap_or_else(|_| time::OffsetDateTime::now_utc()),
                runtime: 0.0,
                logined: false,
                name: "".to_string(),
                password: "".to_string(),
                tip_type: Default::default(),
                show_modal: false,
            },
            font_command.map(Message::FontLoaded),
        )
    }

    fn title(&self) -> String {
        format!(
            "{} - V {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        )
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::BarMessage(msg) => self.bar.update(msg).map(Message::BarMessage),

            Message::Datetime(local_time) => {
                let now = local_time;
                if now != self.now {
                    self.now = now;
                }
                Command::none()
            }
            Message::FontLoaded(_res) => Command::none(),
            Message::LeftMessage(msg) => self.leftwidget.update(msg).map(Message::LeftMessage),
            Message::Namechange(name) => {
                self.name = name;
                Command::none()
            }
            Message::Passchange(password) => {
                self.password = password;
                Command::none()
            }
            Message::LoginCence => {
                self.logined = false;
                window::close()
            }
            Message::Logingpre => {
                let name = self.name.clone();
                let password = self.password.clone();
                Command::perform(login_user(name, password), Message::Loginbool)
            }
            Message::Loginbool(bool) => {
                self.logined = bool;
                if !self.logined {
                    self.show_modal = true;
                    self.tip_type = TipType::LoginTip;
                }
                Command::none()
            }
            Message::OpenModal => {
                self.show_modal = true;
                Command::none()
            }
            Message::CloseModal => {
                self.show_modal = false;
                Command::none()
            }
        }
    }
    fn view(&self) -> iced::Element<'_, Message, Renderer> {
        let loginview = login(&self);
        let leftwidgets = self.leftwidget.view(self).map(Message::LeftMessage);
        let body = match self.tip_type {
            TipType::EmptyTip => "输入框不能为空，请输入要查询的内容再进行查询！",
            TipType::LoginTip => "登录异常，工号不存在或者密码错误，请确认后再进行登录！",
        };
        let all_tip = tip(self, loginview, body);

        container(row!(leftwidgets, all_tip))
            .padding(5)
            .style(release_time_container_theme())
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        let custom_theme = Box::new(
            match self.leftwidget.theme {
                true => styles::theme::TroxideTheme::Dark,
                false => styles::theme::TroxideTheme::Light,
            }
            .get_custom_theme(),
        );
        iced::Theme::Custom(custom_theme)
    }
    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(1000)).map(|_| {
            Message::Datetime(
                time::OffsetDateTime::now_local()
                    .unwrap_or_else(|_| time::OffsetDateTime::now_utc()),
            )
        })
    }
}
