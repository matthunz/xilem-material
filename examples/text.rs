use xilem::{
    view::{LinearLayout, View},
    App, AppLauncher, Axis,
};
use xilem_material::Text;

fn app(_: &mut ()) -> impl View<()> {
    LinearLayout::new(
        (
            Text::builder().font_size(100.).content("Large").build(),
            Text::builder().font_size(60.).content("Medium").build(),
            Text::builder().font_size(20.).content("Small").build(),
        ),
        Axis::Vertical,
    )
}

fn main() {
    let app = App::new((), app);
    AppLauncher::new(app).run()
}
