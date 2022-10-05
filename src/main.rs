#![windows_subsystem = "windows"]

use std::{path::PathBuf, sync::Arc, thread};

use clap::Parser;
use druid::{
    widget::{Flex, Padding, ZStack},
    AppLauncher, Data, Lens, UnitPoint, Widget, WidgetExt, WindowDesc,
};
use rendering::RenderSettings;
use tracing::metadata::LevelFilter;
use tracing_subscriber::{prelude::*, EnvFilter};
use widgets::{data::EditableSpace, Button, SpaceEditor, SpaceEditorData};

use crate::error::Error;

mod error;
#[macro_use]
mod style;
mod math;
mod rendering;
mod widgets;

#[derive(Clone, Data, Lens)]
struct RootData {
    space_editor: SpaceEditorData,
}

fn root() -> impl Widget<RootData> {
    let render_button = Button::new("Render").on_click(|_ctx, data: &mut RootData, _env| {
        let editable_space = Arc::clone(&data.space_editor.space);
        thread::spawn(move || rendering::render(editable_space, &RenderSettings::default()));
    });

    let space_editor = SpaceEditor::new().lens(RootData::space_editor);
    let bottom_right = Flex::row().with_child(render_button);

    ZStack::new(space_editor).with_aligned_child(
        Padding::new(style::WINDOW_PADDING, bottom_right),
        UnitPoint::BOTTOM_RIGHT,
    )
}

#[derive(Parser)]
struct Args {
    space_file: Option<PathBuf>,
}

fn main() -> Result<(), Error> {
    let subscriber = tracing_subscriber::registry()
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::DEBUG.into())
                .from_env_lossy(),
        )
        .with(tracing_subscriber::fmt::layer().without_time());
    tracing::subscriber::set_global_default(subscriber)
        .expect("cannot set default tracing subscriber");

    let args = Args::parse();
    let space = {
        if let Some(path) = &args.space_file {
            let json = std::fs::read_to_string(path)?;
            serde_json::from_str(&json)?
        } else {
            EditableSpace::new()
        }
    };

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
