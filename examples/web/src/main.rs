use iced::{
    button, scrollable, Align, Button, Column, Container, Element, Length,
    Row, Sandbox, Scrollable, Settings, Text
};

use iced_aw::{floating_button::Anchor, Badge, FloatingButton};

const TITLE_SIZE: u16 = 42;

fn main() -> iced::Result {
    Web::run(Settings::default())
}

struct Web {
    scrollable_state: scrollable::State,
    floating_button_lines: Vec<String>,
    floating_button_scroll: scrollable::State,
    floating_button_state: button::State,
}

#[derive(Clone, Debug)]
enum Message {
    FloatingButtonPressed,
}

impl Sandbox for Web {

    type Message = Message;
    
    fn new() -> Self {
        Self {
            scrollable_state: scrollable::State::new(),
            //floating_button_lines: Vec::new(),
            floating_button_lines: vec!("Hello!".into(), "World".into()),
            floating_button_scroll: scrollable::State::new(),
            floating_button_state: button::State::new(),
        }
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::FloatingButtonPressed => {
                self.floating_button_lines.push("This is a newly added line.".into())
            },
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let content = Scrollable::new(&mut self.scrollable_state)
            .spacing(20)
            .max_width(600)
            .push(
                Text::new("Badge:")
                    .size(TITLE_SIZE)
            )
            .push(badge())
            .push(
                Text::new("Floating Button:")
                    .size(TITLE_SIZE)
            )
            .push(
                floating_button(
                    &self.floating_button_lines,
                    &mut self.floating_button_scroll,
                    &mut self.floating_button_state,
                )
            )
            ;

        Container::new(
            // Workaround: https://github.com/hecrj/iced/issues/643
            Column::new()
                .push(content)
                .max_width(600)
            
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}

fn badge<'a>() -> Element<'a, Message> {
    Column::new()
        .spacing(10)
        .push(
            Row::new()
                .align_items(Align::Center)
                .spacing(10)
                .push(
                    Badge::new(Text::new("Default"))
                        .style(iced_aw::style::badge::Default)
                )
                .push(
                    Badge::new(Text::new("Primary"))
                        .style(iced_aw::style::badge::Primary)
                )
                .push(
                    Badge::new(Text::new("Secondary"))
                        .style(iced_aw::style::badge::Secondary)
                )
                .push(
                    Badge::new(Text::new("Success"))
                        .style(iced_aw::style::badge::Success)
                )
                .push(
                    Badge::new(Text::new("Danger"))
                        .style(iced_aw::style::badge::Danger)
                )
        )
        .push(
            Row::new()
                .spacing(10)
                .align_items(Align::Center)
                .push(
                    Badge::new(Text::new("Warning"))
                        .style(iced_aw::style::badge::Warning)
                )
                .push(
                    Badge::new(Text::new("Info"))
                        .style(iced_aw::style::badge::Info)
                )
                .push(
                    Badge::new(Text::new("Dark"))
                        .style(iced_aw::style::badge::Dark)
                )
                .push(
                    Badge::new(Text::new("Light"))
                        .style(iced_aw::style::badge::Light)
                )
                .push(
                    Badge::new(Text::new("White"))
                        .style(iced_aw::style::badge::White)
                )
        )
        .into()
}

fn floating_button<'a>(
    lines: &[String],
    scrollable_state: &'a mut scrollable::State,
    button_state: &'a mut button::State,
) -> Element<'a, Message> {

    let column = lines.iter()
        .fold(
            Column::new(),
            |col, line| {
                col.push(Text::new(line.to_owned()))
            }
        )
        .width(Length::Fill);

    let scrollable = Scrollable::new(scrollable_state)
        .width(Length::Fill)
        .height(Length::Fill)
        .max_height(100)
        .push(column);

    Container::new(
        FloatingButton::new(
            scrollable,
            Button::new(button_state, Text::new("Press Me!")),
        )
        .on_press(Message::FloatingButtonPressed)
        .anchor(Anchor::SouthEast)
        .offset([20.0, 5.0])
    )
    .width(Length::Fill)
    .into()
}