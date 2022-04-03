use iced::{
    button, checkbox, executor,
    futures::executor::block_on,
    pick_list, scrollable,
    window::{self, Icon},
    Align, Application, Button, Checkbox, Clipboard, Column, Command, Container, Element,
    HorizontalAlignment, Length, PickList, Row, Scrollable, Settings, Subscription, Text,
    VerticalAlignment,
};

use crate::gui::{icon, style};

use crate::db::{DatabaseMeta, Direct};

use lazy_static::lazy_static;

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
            size: (800, 600),
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

#[derive(Default, Clone)]
pub struct MyUi {
    // db_meta:DatabaseMeta,
    db_name: String,
    db_name_to: String,
    db_meta: DatabaseMeta,
    direction: Direct,
    table_list: Vec<(String, usize, bool)>,
    check_button_list_left: Vec<button::State>,
    check_button_list_right: Vec<button::State>,
    theme: Option<style::Theme>,
    init_button: button::State,
    switch_button: button::State,
    ensure_button: button::State,
    font_dec_button: button::State,
    pick_list_theme: pick_list::State<style::Theme>,
    scroll_left: scrollable::State,
    scroll_right: scrollable::State,
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
    DirectChanged,
    LoadConf(Direct),

    SelectedTable((bool, usize)),
    Transfer,
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
        let init_button = Button::new(
            &mut self.init_button,
            Text::new("load conf").height(Length::Shrink),
        )
        .style(self.theme.unwrap())
        .on_press(Message::LoadConf(self.direction));

        let text = Text::new(&self.db_name).height(Length::Shrink);

        let table_list_left = self
            .table_list
            .iter()
            .filter(|x| x.2 == true)
            .zip(&mut self.check_button_list_left)
            .fold(
                Column::new().spacing(1),
                |col, ((name, idx, enabled), but)| {
                    col.push(
                        Button::new(but, Text::new(name).height(Length::Fill))
                            .style(self.theme.unwrap())
                            .on_press(Message::SelectedTable((enabled.to_owned(), idx.to_owned()))),
                    )
                },
            );

        let table_list_right = self
            .table_list
            .iter()
            .filter(|x| x.2 == false)
            .zip(&mut self.check_button_list_right)
            .fold(
                Column::new().spacing(1),
                |col, ((name, idx, enabled), but)| {
                    col.push(
                        Button::new(but, Text::new(name).height(Length::Fill))
                            .style(self.theme.unwrap())
                            .on_press(Message::SelectedTable((enabled.to_owned(), idx.to_owned()))),
                    )
                },
            );

        let db_table_scroll_left = Scrollable::new(&mut self.scroll_left)
            .push(table_list_left)
            .width(Length::Fill)
            .height(Length::Fill);

        let db_table_scroll_right = Scrollable::new(&mut self.scroll_right)
            .push(table_list_right)
            .width(Length::Fill)
            .height(Length::Fill);

        let middle_button_group = Column::new()
            .push(
                Button::new(
                    &mut self.switch_button,
                    Text::new("swicth").height(Length::Fill),
                )
                .style(self.theme.unwrap())
                .on_press(Message::LoadConf(Direct::FROM))
                .width(Length::Shrink)
                .height(Length::Shrink),
            )
            .push(
                Button::new(
                    &mut self.ensure_button,
                    Text::new("ok").height(Length::Fill),
                )
                .style(self.theme.unwrap())
                .on_press(Message::Transfer)
                .width(Length::Shrink)
                .height(Length::Shrink),
            )
            .align_items(Align::Center);

        let row = Row::new()
            .push(
                Container::new(db_table_scroll_left)
                    .style(self.theme.unwrap())
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .push(
                Container::new(middle_button_group)
                    .center_y()
                    .width(Length::Shrink)
                    .height(Length::Fill),
            )
            .push(
                Container::new(db_table_scroll_right)
                    .style(self.theme.unwrap())
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .padding(5);

        let content = Column::new()
            // .padding(5)
            // .align_items(Alignment::c)
            .push(init_button)
            .push(text)
            .push(row);

        // .push(button2);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn update(&mut self, message: Self::Message, _: &mut Clipboard) -> Command<Self::Message> {
        match message {
            Message::LoadConf(mut direct) => {
                let db_meta = DatabaseMeta::initial();
                self.db_meta = db_meta.clone();
                self.db_name = db_meta.clone().get_default_db(direct);
                self.db_name_to = db_meta.clone().get_default_db(direct.toggle());

                let table_list = block_on(async move {
                    db_meta
                        .clone()
                        .show_tables(direct, db_meta.clone().get_default_db(direct))
                        .await
                });

                match table_list {
                    Ok(val) => {
                        self.table_list = val
                            .iter()
                            .enumerate()
                            .map(|(idx, x)| (x.to_owned(), idx, true))
                            .collect::<Vec<(String, usize, bool)>>();
                        self.check_button_list_left = vec![button::State::new(); val.len()];
                        self.check_button_list_right = vec![button::State::new(); val.len()];
                    }
                    Err(_) => self.table_list = vec![("TimedOut".to_string(), 0, true)],
                }
            }
            Message::ThemeChanged(t) => self.theme = Some(t),
            Message::SelectedTable(b) => {
                println!("{:?}", b);

                self.table_list[b.1].2 = !b.0;

                // for each in &*checked{
                //     println!("{}",each);
                //     self.table_list[each.to_owned()].1 =b
                // }
            }
            Message::Transfer => {
                // self.db_meta.table_detail(direct, ddb, table)
                let res = self
                    .table_list
                    .iter()
                    .filter(|x| x.2 == false).map(|x| x.0.clone()).collect::<Vec<String>>();

                println!("{:?}",res);
                    

                let table_detail = block_on(async move {
                    self.db_meta
                        .clone()
                        .table_detail(self.direction, self.db_name.clone(), res[0].clone())
                        .await
                });

                println!("{:?}",table_detail);
            }
            Message::DirectChanged => match self.direction {
                Direct::FROM => self.direction = Direct::TO,
                Direct::TO => self.direction = Direct::FROM,
            },
        }

        Command::none()
    }
}
