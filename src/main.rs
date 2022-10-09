#![windows_subsystem = "windows"]

use std::{path::PathBuf, sync::Arc, thread};

use clap::Parser;
use commands::commander;
use druid::{
    widget::{Flex, Padding, ZStack},
    AppLauncher, Data, Lens, UnitPoint, Widget, WidgetExt, WindowDesc,
};
use project::Project;
use rendering::RenderSettings;
use tracing::metadata::LevelFilter;
use tracing_subscriber::{prelude::*, EnvFilter};
use widgets::{Button, SpaceEditor};

use crate::error::Error;

#[macro_use]
mod style;
#[macro_use]
mod commands;

mod error;
mod math;
mod project;
mod rendering;
mod sparse_set;
mod widgets;

#[derive(Clone, Data, Lens)]
struct RootData {
    project: Project,
}

fn root() -> impl Widget<RootData> {
    let render_button = Button::new("Render").on_click(|_ctx, data: &mut RootData, _env| {
        let editable_space = Arc::clone(&data.project.space_editor.space);
        thread::spawn(move || rendering::render(editable_space, &RenderSettings::default()));
    });

    let space_editor = SpaceEditor::new()
        .lens(Project::space_editor)
        .lens(RootData::project);
    let bottom_right = Flex::row().with_child(render_button);

    let stack = ZStack::new(space_editor).with_aligned_child(
        Padding::new(style::WINDOW_PADDING, bottom_right),
        UnitPoint::BOTTOM_RIGHT,
    );
    commander(stack)
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
    let project = {
        if let Some(path) = &args.space_file {
            let json = std::fs::read_to_string(path)?;
            serde_json::from_str(&json)?
        } else {
            Project::new()
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
        .launch(RootData { project })?;

    Ok(())
}
