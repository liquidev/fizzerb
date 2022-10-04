use druid::{
    widget::{Container, Flex, Label, Padding, ZStack},
    AppLauncher, Data, Lens, UnitPoint, Vec2, Widget, WidgetExt, WindowDesc,
};
use widgets::{Button, SpaceEditor, SpaceEditorData};

use crate::error::Error;

mod error;
#[macro_use]
mod style;
mod math;
mod widgets;

#[derive(Clone, Data, Lens)]
pub struct RootData {
    space_editor: SpaceEditorData,
}

fn root() -> impl Widget<RootData> {
    ZStack::new(SpaceEditor::new().lens(RootData::space_editor)).with_aligned_child(
        Padding::new(
            style::WINDOW_PADDING,
            Flex::row()
                .with_child(Button::new("Render"))
                .with_spacer(8.0)
                .with_child(Button::new("Bye Egg")),
        ),
        UnitPoint::BOTTOM_RIGHT,
    )
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
        .launch(RootData {
            space_editor: SpaceEditorData::new(space),
        })?;

    Ok(())
}
