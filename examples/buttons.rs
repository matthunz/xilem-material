use xilem::{
    view::{LinearLayout, View},
    App, AppLauncher, Axis,
};
use xilem_material::{Button, RadioButton, Text};

fn app(_: &mut ()) -> impl View<()> {
    LinearLayout::new(
        (
            Button::new(Text::builder().font_size(24.).content("Filled").build()),
            RadioButton::default(),
        ),
        Axis::Vertical,
    )
}

fn main() {
    let app = App::new((), app);
    AppLauncher::new(app).run()
}
