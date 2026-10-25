use windows_reactor::*;

#[derive(Clone)]
enum Message {
    Pressed(PointerEventInfo),
}

struct Repro {
    presses: usize,
    last: String,
}

impl Component for Repro {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            presses: 0,
            last: "none yet".into(),
        }
    }

    fn update(&mut self, Message::Pressed(info): Message, _: &ComponentContext<Self>) {
        self.presses += 1;
        self.last = format!(
            "left={} right={} middle={} at ({:.0}, {:.0})",
            info.is_left_button_pressed,
            info.is_right_button_pressed,
            info.is_middle_button_pressed,
            info.window_x,
            info.window_y,
        );
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Pointer repro");
        // One Border around the whole page, the usual place for a Back/Forward handler.
        Border::new()
            // Transparent, but still hit-tests, so presses over empty space reach the Border.
            .background(Brush::Solid(Color::argb(0, 0, 0, 0)))
            .padding(Thickness::uniform(16.0))
            .on_pointer_pressed(context.callback(Message::Pressed))
            .content(
                StackPanel::new().spacing(12.0).children((
                    TextBlock::new().text(
                        "1. Press the mouse's Back or Forward button over empty space: the event \
                         fires, but nothing says which button it was (all three flags are false).\n\
                         2. Press any button over a list row or in the text box: the Border's \
                         handler doesn't fire at all, because the child handled the press.",
                    ),
                    TextBlock::new().text(format!("Presses seen: {}", self.presses)),
                    TextBlock::new().text(format!("Last press: {}", self.last)),
                    TextBox::new().text("A text box"),
                    ListView::new().items(
                        ["Row one", "Row two", "Row three"]
                            .into_iter()
                            .map(|name| DataItem::new(name, name)),
                    ),
                )),
            )
            .into()
    }
}

fn main() {
    App::run_component::<Repro>(()).unwrap();
}
