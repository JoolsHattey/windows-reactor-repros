use windows_reactor::*;

struct Repro {
    slider: bool,
    value: f64,
}

enum Message {
    AddSlider,
    Value(f64),
}

impl Component for Repro {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        // `--slider-first` puts the slider in the title bar from the first layout, for comparison.
        Self {
            slider: std::env::args().any(|arg| arg == "--slider-first"),
            value: 50.0,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::AddSlider => self.slider = true,
            Message::Value(value) => self.value = value,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("TitleBar drag regions repro");
        // WinUI's TitleBar computes the passthrough regions for controls in its content on the
        // content's first layout (`AutoRefreshDragRegions` defaults to false), and Reactor never
        // asks it to recompute. A slider added after that gets no passthrough region, so dragging
        // its thumb drags the window.
        let mut content: Vec<View> = vec![TextBlock::new().text("Title bar content:").into()];
        if self.slider {
            content.push(
                Slider::new()
                    .minimum(0.0)
                    .maximum(100.0)
                    .value(self.value)
                    .width(200.0)
                    .on_value_changed(context.callback(Message::Value))
                    .into(),
            );
        }
        StackPanel::new()
            .children((
                TitleBar::new().title("Drag regions").content(
                    StackPanel::new()
                        .orientation(Orientation::Horizontal)
                        .spacing(8.0)
                        .children(content),
                ),
                StackPanel::new()
                    .spacing(8.0)
                    .margin(Thickness::uniform(16.0))
                    .children((
                        Button::new()
                            .content("Add slider")
                            .is_enabled(!self.slider)
                            .on_click(context.callback(|_| Message::AddSlider)),
                        TextBlock::new().text(format!("Slider value: {:.0}", self.value)),
                    )),
            ))
            .into()
    }
}

fn main() {
    App::run_component::<Repro>(()).unwrap();
}
