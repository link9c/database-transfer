use iced::{
    button, checkbox, container, pick_list, progress_bar, radio, rule, scrollable, slider,
    text_input,
};

// use super::{dark,light};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub const ALL: [Theme; 2] = [Theme::Light, Theme::Dark];
}

impl Default for Theme {
    fn default() -> Theme {
        Theme::Light
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Theme::Light => "Light",
                Theme::Dark => "Dark",
            }
        )
    }
}

impl<'a> From<Theme> for Box<dyn container::StyleSheet + 'a> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => light::TextContainerStyle.into(),
            Theme::Dark => dark::Container.into(),
        }
    }
}

impl<'a> From<Theme> for Box<dyn pick_list::StyleSheet + 'a> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => light::Picklist.into(),
            Theme::Dark => dark::Picklist.into(),
        }
    }
}

impl<'a> From<Theme> for Box<dyn radio::StyleSheet + 'a> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::Radio.into(),
        }
    }
}

impl<'a> From<Theme> for Box<dyn text_input::StyleSheet + 'a> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::TextInput.into(),
        }
    }
}

impl<'a> From<Theme> for Box<dyn button::StyleSheet + 'a> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => light::Button.into(),
            Theme::Dark => dark::Button.into(),
        }
    }
}

impl<'a> From<Theme> for Box<dyn scrollable::StyleSheet + 'a> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::Scrollable.into(),
        }
    }
}

impl<'a> From<Theme> for Box<dyn slider::StyleSheet + 'a> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::Slider.into(),
        }
    }
}

impl From<Theme> for Box<dyn progress_bar::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::ProgressBar.into(),
        }
    }
}

impl<'a> From<Theme> for Box<dyn checkbox::StyleSheet + 'a> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::Checkbox.into(),
        }
    }
}

// impl From<Theme> for Box<dyn toggler::StyleSheet> {
//     fn from(theme: Theme) -> Self {
//         match theme {
//             Theme::Light => Default::default(),
//             Theme::Dark => dark::Toggler.into(),
//         }
//     }
// }

impl From<Theme> for Box<dyn rule::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::Rule.into(),
        }
    }
}

pub mod light {

    use iced::{button, container, pick_list, Background, Color};

    pub struct TextContainerStyle;
    impl container::StyleSheet for TextContainerStyle {
        fn style(&self) -> container::Style {
            container::Style {
                text_color: None,
                background: None,
                border_radius: 0.0,
                border_width: 1.0,
                border_color: Color {
                    r: 0.94,
                    g: 0.55,
                    b: 0.55,
                    a: 1.0,
                },
            }
        }
    }

    pub struct Button;
    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            // 文本颜色随着主题而变化
            button::Style {
                background: Some(Background::Color(Color {
                    r: 0.79,
                    g: 0.90,
                    b: 1.0,
                    a: 1.0,
                })),
                border_radius: 9.0,
                border_width: 1.0,
                // 边框颜色置为透明
                border_color: Color {
                    r: 0.94,
                    g: 0.55,
                    b: 0.55,
                    a: 1.0,
                },
                text_color: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                },

                // 这个语法之前提到过了，Rust会自动将未指定的项设置的和..后的结构体的值一致
                ..Default::default()
            }
        }

        fn hovered(&self) -> button::Style {
            let active = self.active();

            button::Style {
                shadow_offset: active.shadow_offset + iced::Vector::new(0.0, 2.0),
                ..active
            }
        }

        fn pressed(&self) -> button::Style {
            button::Style {
                shadow_offset: iced::Vector::default(),
                ..self.active()
            }
        }
    }

    pub struct Picklist;

    impl pick_list::StyleSheet for Picklist {
        fn menu(&self) -> pick_list::Menu {
            pick_list::Menu {
                background: Background::Color(Color {
                    r: 0.79,
                    g: 0.90,
                    b: 1.0,
                    a: 1.0,
                }),
                ..Default::default()
            }
        }

        fn active(&self) -> pick_list::Style {
            pick_list::Style {
                background: Background::Color(Color {
                    r: 0.79,
                    g: 0.90,
                    b: 1.0,
                    a: 1.0,
                }),
                border_radius: 5.0,
                border_width: 1.0,
                // 边框颜色置为透明
                border_color: Color {
                    r: 0.94,
                    g: 0.55,
                    b: 0.55,
                    a: 1.0,
                },
                text_color: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                },

                // 这个语法之前提到过了，Rust会自动将未指定的项设置的和..后的结构体的值一致
                ..Default::default()
            }
        }

        fn hovered(&self) -> pick_list::Style {
            pick_list::Style {
                border_color: Color::BLACK,
                ..self.active()
            }
        }
    }
}
pub mod dark {
    use iced::{
        button, checkbox, container, pick_list, progress_bar, radio, rule, scrollable, slider,
        text_input, Color,
    };

    const SURFACE: Color = Color::from_rgb(
        0x40 as f32 / 255.0,
        0x44 as f32 / 255.0,
        0x4B as f32 / 255.0,
    );

    const ACCENT: Color = Color::from_rgb(
        0x6F as f32 / 255.0,
        0xFF as f32 / 255.0,
        0xE9 as f32 / 255.0,
    );

    const ACTIVE: Color = Color::from_rgb(
        0x72 as f32 / 255.0,
        0x89 as f32 / 255.0,
        0xDA as f32 / 255.0,
    );

    const HOVERED: Color = Color::from_rgb(
        0x67 as f32 / 255.0,
        0x7B as f32 / 255.0,
        0xC4 as f32 / 255.0,
    );

    pub struct Container;

    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                background: Color::from_rgb8(0x36, 0x39, 0x3F).into(),
                text_color: Color::WHITE.into(),
                border_width: 1.0,
                border_color: ACTIVE,
                ..container::Style::default()
            }
        }
    }

    pub struct Radio;

    impl radio::StyleSheet for Radio {
        fn active(&self) -> radio::Style {
            radio::Style {
                background: SURFACE.into(),
                dot_color: ACTIVE,
                border_width: 1.0,
                border_color: ACTIVE,
                // text_color: None,
            }
        }

        fn hovered(&self) -> radio::Style {
            radio::Style {
                background: Color { a: 0.5, ..SURFACE }.into(),
                ..self.active()
            }
        }
    }

    pub struct TextInput;

    impl text_input::StyleSheet for TextInput {
        fn active(&self) -> text_input::Style {
            text_input::Style {
                background: SURFACE.into(),
                border_radius: 2.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            }
        }

        fn focused(&self) -> text_input::Style {
            text_input::Style {
                border_width: 1.0,
                border_color: ACCENT,
                ..self.active()
            }
        }

        fn hovered(&self) -> text_input::Style {
            text_input::Style {
                border_width: 1.0,
                border_color: Color { a: 0.3, ..ACCENT },
                ..self.focused()
            }
        }

        fn placeholder_color(&self) -> Color {
            Color::from_rgb(0.4, 0.4, 0.4)
        }

        fn value_color(&self) -> Color {
            Color::WHITE
        }

        fn selection_color(&self) -> Color {
            ACTIVE
        }
    }

    pub struct Button;

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: ACTIVE.into(),
                border_radius: 3.0,
                text_color: Color::WHITE,
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            button::Style {
                background: HOVERED.into(),
                text_color: Color::WHITE,
                ..self.active()
            }
        }

        fn pressed(&self) -> button::Style {
            button::Style {
                border_width: 1.0,
                border_color: Color::WHITE,
                ..self.hovered()
            }
        }
    }

    pub struct Scrollable;

    impl scrollable::StyleSheet for Scrollable {
        fn active(&self) -> scrollable::Scrollbar {
            scrollable::Scrollbar {
                background: SURFACE.into(),
                border_radius: 2.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                scroller: scrollable::Scroller {
                    color: ACTIVE,
                    border_radius: 2.0,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
            }
        }

        fn hovered(&self) -> scrollable::Scrollbar {
            let active = self.active();

            scrollable::Scrollbar {
                background: Color { a: 0.5, ..SURFACE }.into(),
                scroller: scrollable::Scroller {
                    color: HOVERED,
                    ..active.scroller
                },
                ..active
            }
        }

        fn dragging(&self) -> scrollable::Scrollbar {
            let hovered = self.hovered();

            scrollable::Scrollbar {
                scroller: scrollable::Scroller {
                    color: Color::from_rgb(0.85, 0.85, 0.85),
                    ..hovered.scroller
                },
                ..hovered
            }
        }
    }

    pub struct Slider;

    impl slider::StyleSheet for Slider {
        fn active(&self) -> slider::Style {
            slider::Style {
                rail_colors: (ACTIVE, Color { a: 0.1, ..ACTIVE }),
                handle: slider::Handle {
                    shape: slider::HandleShape::Circle { radius: 9.0 },
                    color: ACTIVE,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
            }
        }

        fn hovered(&self) -> slider::Style {
            let active = self.active();

            slider::Style {
                handle: slider::Handle {
                    color: HOVERED,
                    ..active.handle
                },
                ..active
            }
        }

        fn dragging(&self) -> slider::Style {
            let active = self.active();

            slider::Style {
                handle: slider::Handle {
                    color: Color::from_rgb(0.85, 0.85, 0.85),
                    ..active.handle
                },
                ..active
            }
        }
    }

    pub struct ProgressBar;

    impl progress_bar::StyleSheet for ProgressBar {
        fn style(&self) -> progress_bar::Style {
            progress_bar::Style {
                background: SURFACE.into(),
                bar: ACTIVE.into(),
                border_radius: 10.0,
            }
        }
    }

    pub struct Checkbox;

    impl checkbox::StyleSheet for Checkbox {
        fn active(&self, is_checked: bool) -> checkbox::Style {
            checkbox::Style {
                background: if is_checked { ACTIVE } else { SURFACE }.into(),
                checkmark_color: Color::WHITE,
                border_radius: 2.0,
                border_width: 1.0,
                border_color: ACTIVE,
                // text_color: None,
            }
        }

        fn hovered(&self, is_checked: bool) -> checkbox::Style {
            checkbox::Style {
                background: Color {
                    a: 0.8,
                    ..if is_checked { ACTIVE } else { SURFACE }
                }
                .into(),
                ..self.active(is_checked)
            }
        }
    }

    // pub struct Toggler;

    // impl toggler::StyleSheet for Toggler {
    //     fn active(&self, is_active: bool) -> toggler::Style {
    //         toggler::Style {
    //             background: if is_active { ACTIVE } else { SURFACE },
    //             background_border: None,
    //             foreground: if is_active { Color::WHITE } else { ACTIVE },
    //             foreground_border: None,
    //         }
    //     }

    //     fn hovered(&self, is_active: bool) -> toggler::Style {
    //         toggler::Style {
    //             background: if is_active { ACTIVE } else { SURFACE },
    //             background_border: None,
    //             foreground: if is_active {
    //                 Color {
    //                     a: 0.5,
    //                     ..Color::WHITE
    //                 }
    //             } else {
    //                 Color { a: 0.5, ..ACTIVE }
    //             },
    //             foreground_border: None,
    //         }
    //     }
    // }

    pub struct Rule;

    impl rule::StyleSheet for Rule {
        fn style(&self) -> rule::Style {
            rule::Style {
                color: SURFACE,
                width: 2,
                radius: 1.0,
                fill_mode: rule::FillMode::Padded(15),
            }
        }
    }

    pub struct Picklist;

    impl pick_list::StyleSheet for Picklist {
        fn menu(&self) -> pick_list::Menu {
            pick_list::Menu {
                background: ACTIVE.into(),

                ..Default::default()
            }
        }

        fn active(&self) -> pick_list::Style {
            pick_list::Style {
                background: ACTIVE.into(),
                border_radius: 3.0,
                // border_width: 0.5,
                text_color: Color::WHITE,

                // 这个语法之前提到过了，Rust会自动将未指定的项设置的和..后的结构体的值一致
                ..Default::default()
            }
        }

        fn hovered(&self) -> pick_list::Style {
            pick_list::Style {
                background: HOVERED.into(),
                text_color: Color::WHITE,
                ..Default::default()
            }
        }
    }
}