use std::time::Duration;

use crate::fl;
use crate::operations::text_operations::TextOperations;
use cosmic::{
    app::{self, Core},
    iced::{
        alignment::{Horizontal, Vertical},
        Command, Length, Padding,
    },
    iced_core::renderer::Style,
    iced_style::text_editor::StyleSheet,
    iced_widget::{text_editor, TextEditor},
    widget, Application, Element,
};

/// This is the struct that represents your application.
/// It is used to define the data that will be used by your application.
#[derive(Clone, Default)]
pub struct YourApp {
    /// This is the core of your application, it is used to communicate with the Cosmic runtime.
    /// It is used to send messages to your application, and to access the resources of the Cosmic runtime.
    core: Core,
    content_to_convert: TextContent,
    converted_content: TextContent,
    requires_conversion: bool,
    selected_operations: Vec<String>,
}

pub struct TextContent {
    content: text_editor::Content,
}

impl Default for TextContent {
    fn default() -> Self {
        return TextContent {
            content: text_editor::Content::with_text(""),
        };
    }
}

impl Clone for TextContent {
    fn clone(&self) -> Self {
        let text = self.content.text();

        return TextContent {
            content: text_editor::Content::with_text(&text),
        };
    }
}

/// This is the enum that contains all the possible variants that your application will need to transmit messages.
/// This is used to communicate between the different parts of your application.
/// If your application does not need to send messages, you can use an empty enum or `()`.
#[derive(Debug, Clone)]
pub enum Message {
    TickSlow,
    UpdateContentLeft(text_editor::Action),
    UpdateContentRight(text_editor::Action),
    SetTextRight(String),
}

/// Implement the `Application` trait for your application.
/// This is where you define the behavior of your application.
///
/// The `Application` trait requires you to define the following types and constants:
/// - `Executor` is the executor that will be used to run your application.
/// - `Flags` is the data that your application needs to use before it starts.
/// - `Message` is the enum that contains all the possible variants that your application will need to transmit messages.
/// - `APP_ID` is the unique identifier of your application.
impl Application for YourApp {
    type Executor = cosmic::executor::Default;

    type Flags = ();

    type Message = Message;

    const APP_ID: &'static str = "com.broken-d.TextWrench";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    /// This is the header of your application, it can be used to display the title of your application.
    fn header_center(&self) -> Vec<Element<Self::Message>> {
        vec![widget::text::text(fl!("app-title")).into()]
    }

    fn subscription(&self) -> cosmic::iced::Subscription<Self::Message> {
        cosmic::iced::time::every(Duration::from_millis(200)).map(|_| Message::TickSlow)
    }

    fn update(
        &mut self,
        message: Self::Message,
    ) -> cosmic::iced::Command<app::Message<Self::Message>> {
        match message {
            Message::TickSlow => {
                //Command::none()
                if self.requires_conversion == true {
                    self.requires_conversion = false;
                    Command::perform(
                        perform_conversions(
                            self.content_to_convert.content.text(),
                            self.selected_operations.clone(),
                        ),
                        |result| cosmic::app::Message::App(Message::SetTextRight(result)),
                    )
                } else {
                    Command::none()
                }
            }
            Message::UpdateContentLeft(action) => {
                self.content_to_convert.content.perform(action);
                self.requires_conversion = true;
                Command::none()
            }
            Message::UpdateContentRight(action) => {
                match action {
                    text_editor::Action::Edit(_) => {}
                    _ => self.converted_content.content.perform(action),
                }
                Command::none()
            }
            Message::SetTextRight(text) => {
                self.converted_content.content = text_editor::Content::with_text(&text);
                Command::none()
            }
        }
    }

    /// This is the entry point of your application, it is where you initialize your application.
    ///
    /// Any work that needs to be done before the application starts should be done here.
    ///
    /// - `core` is used to passed on for you by libcosmic to use in the core of your own application.
    /// - `flags` is used to pass in any data that your application needs to use before it starts.
    /// - `Command` type is used to send messages to your application. `Command::none()` can be used to send no messages to your application.
    fn init(core: Core, _flags: Self::Flags) -> (Self, Command<app::Message<Self::Message>>) {
        let example = YourApp {
            core,
            content_to_convert: TextContent::default(),
            converted_content: TextContent::default(),
            requires_conversion: false,
            selected_operations: vec![String::from("UPPER_CASE")],
        };

        (example, Command::none())
    }

    /// This is the main view of your application, it is the root of your widget tree.
    ///
    /// The `Element` type is used to represent the visual elements of your application,
    /// it has a `Message` associated with it, which dictates what type of message it can send.
    ///
    /// To get a better sense of which widgets are available, check out the `widget` module.
    fn view(&self) -> Element<Self::Message> {
        let text_input_heading = widget::text::heading("Input Text");
        let text_input_editor = text_editor(&self.content_to_convert.content)
            .font(cosmic::font::FONT_MONO_REGULAR)
            .on_action(Message::UpdateContentLeft);
        let text_input_view = widget::column()
            .push(text_input_heading)
            .push(text_input_editor)
            .padding(2_f32);

        let converted_heading = cosmic::widget::text::heading("Conversion Result");
        let converted_text_viewer =
            cosmic::iced_widget::text_editor(&self.converted_content.content)
                .font(cosmic::font::FONT_MONO_REGULAR)
                .on_action(Message::UpdateContentRight);
        let converted_view = widget::column()
            .push(converted_heading)
            .push(converted_text_viewer)
            .padding(2_f32);

        let text_io_container = widget::column().push(text_input_view).push(converted_view);

        widget::container(text_io_container)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .into()
    }
}

async fn perform_conversions(source_text: String, conversions: Vec<String>) -> String {
    let mut source_text = String::from(source_text.trim_end());

    if source_text.len() == 0 {
        String::from("")
    } else {
        let text_operations = TextOperations::get_instance();

        for operation_name in conversions {
            let operation = text_operations.get_operation_or_noop(operation_name.as_str());
            let conversion = operation.convert(source_text.as_str());

            match conversion {
                Ok(result) => source_text = result.into(),
                Err(e) => {
                    source_text = e.to_string();
                    break;
                }
            };
        }

        source_text
    }
}
