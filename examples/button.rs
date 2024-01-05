use xilem::{
    view::{LinearLayout, View},
    App, AppLauncher,
};
use xilem_material::{Button, Text};

fn app(_: &mut ()) -> impl View<()> {
    LinearLayout::new(
        (
            Text::builder().font_size(100.).content("Large").build(),
            Button::new(Text::builder().font_size(24.).content("Filled").build()),
        ),
        xilem::Axis::Vertical,
    )
}

fn main() {
    let app = App::new((), app);
    AppLauncher::new(app).run()
}
