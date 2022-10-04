use druid::{
    widget::{Flex, Label},
    AppLauncher, Widget, WindowDesc,
};
use widgets::{SpaceEditor, SpaceEditorData};

use crate::error::Error;

mod error;
#[macro_use]
mod style;
mod widgets;

fn root() -> impl Widget<SpaceEditorData> {
    Flex::column()
        .with_child(Label::new("Twój stary pijany mode: ON"))
        .with_flex_child(SpaceEditor::new(), 1.0)
}

fn main() -> Result<(), Error> {
    let space = std::fs::read_to_string("spaces/four_walls.json")?;
    let space = serde_json::from_str(&space)?;

    let window = WindowDesc::new(root())
        .window_size((600.0, 600.0))
        .resizable(true)
        .title("fizzerb");

    AppLauncher::with_window(window)
        .configure_env(|env, _| {
            style::configure_env(env).expect("cannot configure styles");
        })
        .launch(SpaceEditorData::new(space))?;

    Ok(())
}
