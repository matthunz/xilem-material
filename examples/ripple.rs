use xilem::{view::View, App, AppLauncher};
use xilem_material::Ripple;

fn app(_: &mut ()) -> impl View<()> {
    Ripple::new()
}

fn main() {
    let app = App::new((), app);
    AppLauncher::new(app).run()
}
