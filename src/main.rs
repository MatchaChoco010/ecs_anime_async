use anyhow::Result;

mod app;
use app::*;

fn main() -> Result<()> {
    let mut app = App::new("app", "Orito Itsuki")?;
    app.load_animation("anim1", "./anim1.json")?;
    app.run()
}
