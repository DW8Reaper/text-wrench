use std::hash::DefaultHasher;
use std::time::Duration;

use crate::operations::text_operations::{TextOperation, TextOperations};
use crate::{fl, operations};
use cosmic::cosmic_theme::palette::convert::IntoColorUnclamped;
use cosmic::cosmic_theme::palette::num::Ln;
use cosmic::cosmic_theme::{Container, Theme};
use cosmic::iced::theme::palette::Pair;
use cosmic::iced::{Background, Color};
use cosmic::iced_core::widget::text;
use cosmic::widget::{container, style};
use cosmic::{
    app::{self, Core},
    iced::{
        alignment::{Horizontal, Vertical},
        clipboard, Command, Length,
    },
    iced_widget::text_editor,
    prelude::*,
    widget, Application, Element,
};

const DEFAULT_PADDING: f32 = 1.;

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
    selected_operations: Vec<usize>,
    operation_names: Vec<&'static str>,
    operation_ids: Vec<&'static str>,
    operation_none_index: usize,
}

pub struct TextContent {
    content: text_editor::Content,
}

impl TextContent {
    fn clear(&mut self) {
        self.content = text_editor::Content::with_text("");
    }
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
    InputContentEditorAction(text_editor::Action),
    SetInputContent(String),
    ClearInputContent,
    CopyInputContent,
    PasteInputContent,
    ConvertedContentEditorAction(text_editor::Action),
    SetConvertedContent(String),
    CopyConvertedContent,
    DeleteOperation(usize),
    SelectOperation(usize, usize),
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
                    let selected_ids = self
                        .selected_operations
                        .iter()
                        .map(|index| String::from(self.operation_ids[*index]))
                        .collect();
                    Command::perform(
                        perform_conversions(self.content_to_convert.content.text(), selected_ids),
                        |result| cosmic::app::Message::App(Message::SetConvertedContent(result)),
                    )
                } else {
                    Command::none()
                }
            }
            Message::SetInputContent(text) => {
                self.content_to_convert.content = text_editor::Content::with_text(text.as_str());
                self.requires_conversion = true;
                Command::none()
            }
            Message::InputContentEditorAction(action) => {
                self.content_to_convert.content.perform(action);
                self.requires_conversion = true;
                Command::none()
            }
            Message::ClearInputContent => {
                self.content_to_convert.clear();
                self.converted_content.clear();
                Command::none()
            }
            Message::CopyConvertedContent => {
                clipboard::write(self.converted_content.content.text())
            }
            Message::CopyInputContent => clipboard::write(self.content_to_convert.content.text()),
            Message::PasteInputContent => clipboard::read(|maybe_text| {
                let text = maybe_text.unwrap_or_else(|| String::from(""));
                cosmic::app::Message::App(Message::SetInputContent(text))
            }),
            Message::ConvertedContentEditorAction(action) => {
                match action {
                    text_editor::Action::Edit(_) => {}
                    _ => self.converted_content.content.perform(action),
                }
                Command::none()
            }
            Message::SetConvertedContent(text) => {
                self.converted_content.content = text_editor::Content::with_text(&text);
                Command::none()
            }
            Message::DeleteOperation(operation_index) => {
                if operation_index < self.selected_operations.len() {
                    if operation_index == self.selected_operations.len() - 1 {
                        self.selected_operations[operation_index] = self.operation_none_index;
                    } else {
                        self.selected_operations.remove(operation_index);
                    }
                    self.requires_conversion = true;
                }
                Command::none()
            }
            Message::SelectOperation(select_index, operation_index) => {
                self.selected_operations[select_index] = operation_index;
                self.requires_conversion = true;

                if (*self.selected_operations.last().unwrap() != self.operation_none_index) {
                    self.selected_operations.push(self.operation_none_index)
                } else {
                    while (self.selected_operations.len() > 2) {
                        let last = self.selected_operations.len() - 1;
                        let second_last = self.selected_operations.len() - 2;
                        if (self.selected_operations[last] == self.operation_none_index
                            && self.selected_operations[second_last] == self.operation_none_index)
                        {
                            self.selected_operations.pop();
                        } else {
                            break;
                        }
                    }
                }
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
        let text_operations = TextOperations::get_instance();
        let noop_id = text_operations.get_noop().get_id();
        let operation_ids = text_operations.get_operations();
        let operation_names = operation_ids
            .iter()
            .map(|&id| {
                let operation = text_operations.get_operation(id).unwrap();
                operation.get_name()
            })
            .collect();

        let mut operation_none_index: usize = 0;
        for (index, id) in operation_ids.iter().enumerate() {
            if (**id == *noop_id) {
                operation_none_index = index;
                break;
            }
        }

        let example = YourApp {
            core,
            content_to_convert: TextContent::default(),
            converted_content: TextContent::default(),
            requires_conversion: false,
            selected_operations: vec![operation_none_index],
            operation_ids,
            operation_names,
            operation_none_index,
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
        let operations_container = self.create_conversion_options();
        let text_input_view = self.create_text_input_view();
        let converted_view = self.create_converted_data_view();

        let text_io_container = widget::column()
            .padding([5., 0., 0., 0.])
            .push(text_input_view)
            .push(converted_view)
            .width(Length::FillPortion(4));

        let app_layout = widget::row()
            .spacing(5.)
            .push(operations_container)
            .push(text_io_container);

        widget::container(app_layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .into()
    }
}

impl YourApp {
    fn create_conversion_options(&self) -> Element<Message> {
        let mut operation_selection_list =
            widget::column().push(widget::text::heading("Conversion Operations"));

        for index in 0..self.selected_operations.len() {
            let operation_dropdown = widget::dropdown(
                &self.operation_names,
                Some(self.selected_operations[index]),
                move |operation_index| Message::SelectOperation(index, operation_index.clone()),
            );

            let mut operation_line = widget::row().spacing(2.).push(operation_dropdown);

            let add_delete = if (index == self.selected_operations.len() - 1
                || self.selected_operations.len() == 1)
            {
                // only add delete when there is a single or or when this is the last row and it is already none
                self.selected_operations[index] != self.operation_none_index
            } else {
                true
            };

            if (add_delete) {
                let operation_delete =
                    widget::button::icon(widget::icon::from_name("edit-delete-symbolic"))
                        .on_press(Message::DeleteOperation(index));

                operation_line = operation_line.push(operation_delete);
            }

            operation_selection_list = operation_selection_list.push(operation_line);
        }

        widget::container(operation_selection_list)
            .padding(5.)
            .height(Length::Fill)
            .width(Length::Fixed(250.))
            // .style(format)
            .style(cosmic::theme::Container::Secondary)
            .into()
    }

    fn create_text_input_view(&self) -> Element<Message> {
        let text_input_copy =
            widget::button::icon(cosmic::widget::icon::from_name("edit-copy-symbolic"))
                .tooltip("Copy all")
                .on_press(Message::CopyInputContent);
        let text_input_paste =
            widget::button::icon(cosmic::widget::icon::from_name("edit-paste-symbolic"))
                .tooltip("Paste overwrite")
                .on_press(Message::PasteInputContent);
        let text_input_clear =
            widget::button::icon(cosmic::widget::icon::from_name("edit-clear-symbolic"))
                .tooltip("Clear all")
                .on_press(Message::ClearInputContent);

        let text_input_toolbar = widget::row()
            .push(text_input_copy)
            .push(text_input_paste)
            .push(text_input_clear)
            .spacing(2);

        let text_input_heading = widget::text::heading("Input Text");
        let text_input_editor = text_editor(&self.content_to_convert.content)
            .font(cosmic::font::FONT_MONO_REGULAR)
            .on_action(Message::InputContentEditorAction);

        widget::column()
            .push(text_input_heading)
            .push(text_input_toolbar)
            .push(text_input_editor)
            .padding([
                DEFAULT_PADDING * 2.,
                DEFAULT_PADDING,
                DEFAULT_PADDING,
                DEFAULT_PADDING,
            ])
            .into()
    }

    fn create_converted_data_view(&self) -> Element<Message> {
        let converted_copy =
            widget::button::icon(cosmic::widget::icon::from_name("edit-copy-symbolic").size(16))
                .tooltip("Copy all")
                .on_press(Message::CopyConvertedContent);

        let converted_toolbar = widget::row().push(converted_copy).spacing(2);
        let converted_heading = cosmic::widget::text::heading("Conversion Result");
        let converted_text_viewer =
            cosmic::iced_widget::text_editor(&self.converted_content.content)
                .font(cosmic::font::FONT_MONO_REGULAR)
                .on_action(Message::ConvertedContentEditorAction);

        widget::column()
            .push(converted_heading)
            .push(converted_toolbar)
            .push(converted_text_viewer)
            .padding([
                DEFAULT_PADDING * 2.,
                DEFAULT_PADDING,
                DEFAULT_PADDING,
                DEFAULT_PADDING,
            ])
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
