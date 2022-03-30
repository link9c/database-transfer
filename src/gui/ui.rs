use iced::{
    button, checkbox, executor, pick_list, scrollable,
    window::{self, Icon},
    Application, Button, Checkbox, Column, Command, Container, Element, Length, PickList, Row,
    Scrollable, Settings, Subscription, Text,
};

use crate::gui::{icon, style};

// type Cardtype = Option<
//     Box<
//         dyn for<'r, 's> Fn(
//             &'r HashMap<String, Vec<String>>,
//             &'s Vec<Vec<String>>,
//             usize,
//         ) -> (usize, String),
//     >,
// >;

pub fn render_window() -> iced::Result {
    let dy_img = image::open("resource/1.ico");
    let icon = match dy_img {
        Ok(im) => {
            let h = im.height();
            let w = im.width();
            let rgb_img = im.as_rgba8().unwrap().as_raw();
            let icon = Icon::from_rgba(rgb_img.clone(), w, h).unwrap();
            Some(icon)
        }
        Err(_) => {
            let icon = Icon::from_rgba(icon::RAW_ICO.to_vec(), 32, 32).unwrap();
            Some(icon)
        }
    };

    MyUi::run(Settings {
        window: window::Settings {
            size: (440, 320),
            min_size: Some((200, 100)),
            max_size: None,
            resizable: true,
            decorations: true,
            transparent: false,
            always_on_top: true,
            icon,
        },
        // flags: c,
        ..Settings::default()
    })
}

#[derive(Default)]
pub struct MyUi {
    theme: Option<style::Theme>,
    init_button: button::State,
    left_button: button::State,
    right_button: button::State,
    font_dec_button: button::State,
    check_list: Vec<bool>,
    pick_list_left: pick_list::State<String>,
    pick_list_right: pick_list::State<String>,
    pick_list_theme: pick_list::State<style::Theme>,
}

// impl GetCard {
//     fn exec(&mut self) -> Option<String> {
//         let f = self.get.as_ref().unwrap();
//         let (cid, info) = f(&self.cards, &self.art_hash, self.cid);
//         if self.cid != cid && cid != 0 {
//             self.card = info.clone();
//             Some(info)
//         } else {
//             None
//         }
//     }
// }

#[derive(Debug, Clone)]
pub enum Message {
    DbSelected(String),
    ThemeChanged(style::Theme),
}

impl Application for MyUi {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    fn new(_: ()) -> (Self, Command<Self::Message>) {
        (
            MyUi {
                theme: Some(style::Theme::Light),
                ..Default::default() // card_pack: flags,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("MD卡片识图")
    }

    fn view(&mut self) -> Element<Message> {
        let choose_theme = PickList::new(
            &mut self.pick_list_theme,
            &style::Theme::ALL[..],
            self.theme,
            Message::ThemeChanged,
        )
        .style(self.theme.unwrap())
        .text_size(12)
        // .padding(0.1)
        .width(Length::Units(60));

        let left_pick_db = PickList::new(
            &mut self.pick_list_left,
            ["a".to_string(), "b".to_string(), "c".to_string()],
            Some("".to_string()),
            Message::DbSelected,
        )
        .style(self.theme.unwrap())
        .text_size(12)
        // .padding(0.1)
        .width(Length::Units(60));

        // let button2 = Button::new(
        //     &mut self.button2,
        //     Text::new("change time")
        //         .height(Length::Fill)
        //         .vertical_alignment(alignment::Vertical::Center),
        // )
        // .on_press(Message::Send(echo::Input::Change(2000)));

        let content = Column::new()
            // .padding(5)
            // .align_items(Alignment::c)
            .push(
                Container::new(cname_label)
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .padding(5)
                    .style(self.theme.unwrap()),
            )
            .push(
                Container::new(ename_label)
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .padding(5)
                    .style(self.theme.unwrap()),
            )
            .push(
                Container::new(jname_label)
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .padding(5)
                    .style(self.theme.unwrap()),
            )
            .push(
                Container::new(attr_label)
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .padding(5)
                    .style(self.theme.unwrap()),
            )
            .push(
                Container::new(text_scrollable)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(5)
                    .style(self.theme.unwrap()),
            )
            .push(
                Container::new(button_group)
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .align_x(alignment::Horizontal::Right)
                    .style(self.theme.unwrap()),
            );

        // .push(button2);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Echo((oid, event)) => match event {
                echo::Event::MessageToUI(echo::Output::Card(card)) => {
                    if self.card.cn_name != card.cn_name && !card.id.is_empty() && oid == 0 {
                        self.card = card;
                    }
                }
                echo::Event::Init => {
                    self.card.desc = "加载完成。请开启游戏。".to_string();
                }
                echo::Event::Looping => {}
            },

            // Message::Send => {},
            Message::Send => {}
            Message::FontSizeAdd => {
                if self.font_size < 30 {
                    self.font_size += 1;
                }
            }
            Message::FontSizeDec => {
                if self.font_size > 6 {
                    self.font_size -= 1;
                }
            }
            Message::ThemeChanged(theme) => {
                self.theme = Some(theme);
            }
        }

        Command::none()
    }
}

#[derive(Debug)]
struct EchoMap {
    id: usize,
}

impl EchoMap {
    pub fn new(id: usize) -> Self {
        EchoMap { id }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        echo::init(self.id).map(Message::Echo)
    }
}
