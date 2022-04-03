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
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Status {
    LEFT,
    RIGHT,
    HIDE,
}

impl Default for Status {
    fn default() -> Self {
        Self::LEFT
    }
}

impl Status {
    pub fn toggle(self) -> Self {
        match self {
            Status::LEFT => Status::RIGHT,
            Status::RIGHT => Status::LEFT,
            Status::HIDE => Self::HIDE,
        }
    }
}

#[derive(Default, Clone)]
struct TableControl {
    name: String,
    index: usize,
    status: Status,
}
#[derive(Default, Clone)]
pub struct MyUi {
    // db_meta:DatabaseMeta,
    db_name: String,
    db_name_to: String,
    db_meta: DatabaseMeta,
    direction: Direct,
    table_list: Vec<TableControl>,
    table_status: Status,
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

#[derive(Debug, Clone)]
pub enum Message {
    DirectChanged,
    LoadConf(Direct),

    SelectedTable((Status, usize)),
    Transfer(Direct),
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
        String::from("SQL")
    }

    fn view(&mut self) -> Element<Message> {
        let init_button = Button::new(
            &mut self.init_button,
            Text::new("load conf").height(Length::Shrink),
        )
        .style(self.theme.unwrap())
        .on_press(Message::LoadConf(self.direction));

        let table_list_left = self
            .table_list
            .iter()
            .filter(|x| x.status == Status::LEFT)
            .zip(&mut self.check_button_list_left)
            .fold(Column::new().spacing(1), |col, (table, but)| {
                col.push(
                    Button::new(but, Text::new(table.name.to_owned()).height(Length::Fill))
                        .style(self.theme.unwrap())
                        .on_press(Message::SelectedTable((table.status, table.index))),
                )
            });

        let table_list_right = self
            .table_list
            .iter()
            .filter(|x| x.status == Status::RIGHT)
            .zip(&mut self.check_button_list_right)
            .fold(Column::new().spacing(1), |col, (table, but)| {
                col.push(
                    Button::new(but, Text::new(table.name.to_owned()).height(Length::Fill))
                        .style(self.theme.unwrap())
                        .on_press(Message::SelectedTable((table.status, table.index))),
                )
            });

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
                .on_press(Message::DirectChanged)
                .width(Length::Shrink)
                .height(Length::Shrink),
            )
            .push(
                Button::new(
                    &mut self.ensure_button,
                    Text::new("ok").height(Length::Fill),
                )
                .style(self.theme.unwrap())
                .on_press(Message::Transfer(self.direction))
                .width(Length::Shrink)
                .height(Length::Shrink),
            )
            .align_items(Align::Center);

        let row = Row::new()
            .push(
                Container::new(
                    Column::new()
                        .push(Text::new(&self.db_name).height(Length::Shrink))
                        .push(
                            Container::new(db_table_scroll_left)
                                .style(self.theme.unwrap())
                                .width(Length::Fill)
                                .height(Length::Fill),
                        ),
                )
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
                Container::new(
                    Column::new()
                        .push(Text::new(&self.db_name_to).height(Length::Shrink))
                        .push(
                            Container::new(db_table_scroll_right)
                                .style(self.theme.unwrap())
                                .width(Length::Fill)
                                .height(Length::Fill),
                        ),
                )
                .width(Length::Fill)
                .height(Length::Fill),
            )
            .padding(5);

        let content = Column::new()
            // .padding(5)
            // .align_items(Alignment::c)
            .push(init_button)
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
                println!("{:?}", direct);
                let db_meta = DatabaseMeta::initial();
                self.db_meta = db_meta.clone();
                match direct {
                    Direct::FROM => {
                        self.db_name = db_meta.clone().get_default_db(direct);
                        self.db_name_to = db_meta.clone().get_default_db(direct.toggle());
                    }
                    Direct::TO => {
                        self.db_name_to = db_meta.clone().get_default_db(direct);
                        self.db_name = db_meta.clone().get_default_db(direct.toggle());
                    }
                }

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
                            .map(|(idx, x)| TableControl {
                                name: x.to_owned(),
                                index: idx,
                                status: match direct {
                                    Direct::FROM => Status::LEFT,
                                    Direct::TO => Status::RIGHT,
                                },
                            })
                            .collect::<Vec<TableControl>>();
                        self.check_button_list_left = vec![button::State::new(); val.len()];
                        self.check_button_list_right = vec![button::State::new(); val.len()];
                    }
                    Err(_) => {
                        self.table_list = vec![TableControl {
                            name: "TIMEOUT".to_string(),
                            index: 0,
                            status: Status::LEFT,
                        }]
                    }
                }
            }
            Message::ThemeChanged(t) => self.theme = Some(t),
            Message::SelectedTable(table) => {
                println!("{:?}", table);

                self.table_list[table.1].status = table.0.toggle();

                // for each in &*checked{
                //     println!("{}",each);
                //     self.table_list[each.to_owned()].1 =b
                // }
            }
            Message::Transfer(direct) => {
                // self.db_meta.table_detail(direct, ddb, table)
                let res = self
                    .table_list
                    .iter()
                    .filter(|x| {
                        x.status
                            == match direct {
                                Direct::FROM => Status::RIGHT,
                                Direct::TO => Status::LEFT,
                            }
                    })
                    .map(|x| x.name.clone())
                    .collect::<Vec<String>>();

                println!("{:?}", res);

                let name = match direct {
                    Direct::FROM => self.db_name.clone(),
                    Direct::TO => self.db_name_to.clone(),
                };

                let table_detail = block_on(async move {
                    self.db_meta
                        .clone()
                        .table_detail(self.direction, name, res[0].clone())
                        .await
                });

                println!("{:?}", table_detail);
            }
            Message::DirectChanged => {
                println!("222{:?}", self.direction);
                self.direction = self.direction.toggle();
                self.table_status = self.table_status.toggle();
            }
        }

        Command::none()
    }
}
