use windows_reactor::*;

struct TrayApp;

impl Application for TrayApp {
    type Input = ();
    type Message = ();

    fn create(_: &(), _: &ApplicationContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: (), context: &ApplicationContext<Self>) {
        context.exit();
    }

    fn view(&self, _: &(), context: &ApplicationViewContext<Self>) -> ApplicationView {
        // Same as crates/samples/reactor/notifyicon-basic. Right-click the tray icon a few times
        // and watch the pointer: it shows the busy cursor briefly before the menu responds.
        ApplicationView::new().notify_icon(
            "tray",
            NotifyIcon::new(concat!(env!("CARGO_MANIFEST_DIR"), "\\icon.ico"))
                .tooltip("Right-click for the menu")
                .menu(Menu::new(
                    [
                        MenuItem::item("one", "One"),
                        MenuItem::item("two", "Two"),
                        MenuItem::separator("separator"),
                        MenuItem::item("exit", "Exit"),
                    ],
                    context.callback(|_| ()),
                )),
        )
    }
}

fn main() {
    App::run_application::<TrayApp>(()).unwrap();
}
