use windows_reactor::*;

struct Repro;

impl Component for Repro {
    type Input = ();
    type Message = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("TitleBar height repro");
        // The goal: a 64 px title bar whose content (a search box) stretches between the title
        // and the caption buttons. `Tall` is the largest preferred height, and `height` makes the
        // element taller without changing WinUI's TitleBarExpandedHeight, so the caption buttons
        // stay 48 px and the content stays centred at its natural width. TitleBar has no
        // `resource_overrides` (Button and Expander do), so TitleBarExpandedHeight and
        // TitleBarContentHorizontalAlignment can't be set either.
        let search = TextBox::new().text("Search box: should fill the bar");
        StackPanel::new()
            .children((
                TitleBar::new()
                    .title("Height")
                    .preferred_height(WindowTitleBarHeight::Tall)
                    .height(64.0)
                    .content(search),
                Border::new().padding(Thickness::uniform(16.0)).content(TextBlock::new().text(
                    "Asked for a 64 px title bar with stretched content. Compare the bar's \
                     height with the caption buttons, and the search box's width.",
                )),
            ))
            .into()
    }
}

fn main() {
    App::run_component::<Repro>(()).unwrap();
}
