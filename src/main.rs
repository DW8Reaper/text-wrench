use app::YourApp;
use cosmic::iced::{Limits, Size};
/// The `app` module is used by convention to indicate the main component of our application.
mod app;
mod core;
mod operations;

/// The `cosmic::app::run()` function is the starting point of your application.
/// It takes two arguments:
/// - `settings` is a structure that contains everything relevant with your app's configuration, such as antialiasing, themes, icons, etc...
/// - `()` is the flags that your app needs to use before it starts.
///  If your app does not need any flags, you can pass in `()`.
fn main() -> cosmic::iced::Result {
    let settings = cosmic::app::Settings::default()
        .size_limits(Limits::new(Size::new(600., 400.), Size::INFINITY));
    //.theme(cosmic::Theme::light())
    // .default_icon_theme("Pop");

    cosmic::app::run::<YourApp>(settings, ())
}
