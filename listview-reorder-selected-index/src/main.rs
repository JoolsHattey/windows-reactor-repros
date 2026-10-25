use std::time::Duration;
use windows_reactor::*;

#[derive(Clone)]
enum Message {
    Play(Option<usize>),
    Loaded(&'static str),
}

/// A "recently played" list: the selection shows what's playing, and a song moves to the top
/// once it has loaded, the way a media player's history does.
struct Recent {
    items: Vec<&'static str>,
    playing: Option<&'static str>,
}

impl Component for Recent {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            items: vec!["One", "Two", "Three", "Four", "Five"],
            playing: None,
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::Play(Some(index)) => {
                let item = self.items[index];
                // Loading takes a moment; the selection keeps showing what's playing until then.
                context.spawn_background(move |_| {
                    std::thread::sleep(Duration::from_millis(300));
                    Message::Loaded(item)
                });
            }
            Message::Play(None) => {}
            Message::Loaded(item) => {
                self.playing = Some(item);
                self.items.retain(|i| *i != item);
                self.items.insert(0, item);
            }
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("ListView reorder repro");
        let selected = self.items.iter().position(|i| Some(*i) == self.playing);
        StackPanel::new()
            .spacing(12.0)
            .children((
                TextBlock::new().text(
                    "Click \"Three\" (not its button). It moves to the top once it has \"loaded\", \
                     and the process aborts.",
                ),
                ListView::new()
                    .selected_index(selected)
                    .on_selection_changed(context.callback(Message::Play))
                    .items(self.items.iter().map(|name| {
                        // Each row has a "..." button. With plain text rows it doesn't crash.
                        DataItem::new(*name, name).content(
                            Grid::new()
                                .columns([GridLength::Star(1.0), GridLength::Auto])
                                .height(40.0)
                                .children((
                                    TextBlock::new()
                                        .text(*name)
                                        .vertical_alignment(VerticalAlignment::Center),
                                    Button::new().grid_column(1).content("..."),
                                )),
                        )
                    })),
            ))
            .into()
    }
}

fn main() {
    App::run_component::<Recent>(()).unwrap();
}
