use windows_reactor::*;

struct Repro;

impl Component for Repro {
    type Input = ();
    type Message = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("TitleBar padding repro");
        let marker = Border::new()
            .width(120.0)
            .height(32.0)
            .background(Brush::Solid(Color::rgb(220, 40, 40)))
            .content(TextBlock::new().text("right header"));
        StackPanel::new()
            .children((
                TitleBar::new().title("TitleBar padding repro").right_header(marker),
                Border::new().padding(Thickness::uniform(16.0)).content(TextBlock::new().text(
                    "At 100% scaling the red right header ends at the caption buttons. \
                     Above 100% there is a gap between them that grows with the scale.",
                )),
            ))
            .into()
    }
}

fn main() {
    App::run_component::<Repro>(()).unwrap();
}
