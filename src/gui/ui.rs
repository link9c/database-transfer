use iced::{Align, Application, Button, Checkbox, Clipboard, Column, Command, Container, Element, HorizontalAlignment, Length, PickList, Row, Scrollable, Settings, Subscription, Text, VerticalAlignment, button, checkbox, executor, futures::executor::block_on, pick_list, scrollable, window::{self, Icon}};

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

#[derive(Default, Clone)]
pub struct MyUi {
    // db_meta:DatabaseMeta,
    db_name: String,
    table_list: Vec<(String, usize,bool)>,
    check_button_list: Vec<button::State>,
    theme: Option<style::Theme>,
    init_button: button::State,
    left_button: button::State,
    right_button: button::State,
    font_dec_button: button::State,
    check_list: Vec<String>,
    pick_list_theme: pick_list::State<style::Theme>,
    scroll_left: scrollable::State,
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
    LoadConf,
    SelectedTable((bool, usize)),
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
        let init_button = Button::new(
            &mut self.init_button,
            Text::new("load conf").height(Length::Shrink),
        )
        .style(self.theme.unwrap())
        .on_press(Message::LoadConf);

        let text = Text::new(&self.db_name).height(Length::Shrink);

        let table_list = self
            .table_list
            .iter()
            .filter(|x| x.2==true)
            .zip(&mut self.check_button_list)
            .fold(
                Column::new().spacing(1),
                |col, ((name, idx,enabled), but)| {
                    col.push(
                        Row::new()
                            .push(
                                Button::new(but, Text::new(name).height(Length::Fill))
                                    .style(self.theme.unwrap())
                                    .on_press(Message::SelectedTable((enabled.to_owned(),idx.to_owned()))),
                            )
                            .push(Text::new("X").vertical_alignment(VerticalAlignment::Center)),
                    )
                },
            );

        let db_table_scroll_left = Scrollable::new(&mut self.scroll_left)
            .push(table_list)
            .width(Length::Fill)
            .height(Length::Fill);

        let content = Column::new()
            // .padding(5)
            // .align_items(Alignment::c)
            .push(
                Container::new(init_button)
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .padding(5)
                    .style(self.theme.unwrap()),
            )
            .push(text)
            .push(db_table_scroll_left);

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
            Message::LoadConf => {
                let db_meta = DatabaseMeta::initial();
                self.db_name = db_meta.clone().get_default_db(Direct::TO);

                let table_list =
                    block_on(async move { db_meta.clone().show_dbs(Direct::TO).await });

                match table_list {
                    Ok(val) => {
                        self.table_list = val
                            .iter()
                            .enumerate()
                            .map(|(idx, x)| (x.to_owned(), idx,true))
                            .collect::<Vec<(String, usize,bool)>>();
                        self.check_button_list = vec![button::State::new(); val.len()];
                    }
                    Err(_) => self.table_list = vec![("TimedOut".to_string(), 0,true)],
                }
            }
            Message::DbSelected(_) => todo!(),
            Message::ThemeChanged(t) => self.theme = Some(t),
            Message::SelectedTable(b) => {
                println!("{:?}", b);
           
                self.table_list[b.1].2=!b.0;
             
                
                // for each in &*checked{
                //     println!("{}",each);
                //     self.table_list[each.to_owned()].1 =b
                // }
            }
        }

        Command::none()
    }
}
