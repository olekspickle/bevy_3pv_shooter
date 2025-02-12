// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{
    asset::AssetMetaCheck, log, prelude::*, window::PrimaryWindow, winit::WinitWindows,
    DefaultPlugins,
};
use bevy_3rd_person_view::game;
use std::io::Cursor;
use winit::window::Icon;

fn main() {
    let window = WindowPlugin {
        primary_window: Some(Window {
            title: "Bevy 3rd person game".to_string(), // ToDo
            // Bind to canvas included in `index.html`
            canvas: Some("#bevy".to_owned()),
            fit_canvas_to_parent: true,
            // Tells wasm not to override default event handling, like F5 and Ctrl+R
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    };
    let assets = AssetPlugin {
        meta_check: AssetMetaCheck::Never,
        ..default()
    };
    let log_level = log::LogPlugin {
        level: log::Level::TRACE,
        filter: "info,wgpu=warn".to_string(),
        ..Default::default()
    };
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(window).set(assets).set(log_level))
        .add_plugins(game)
        .add_systems(Startup, set_window_icon);

    // set any additional plugins/systems

    app.run();
}

// Sets the icon on windows and X11
fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.single();
    let Some(primary) = windows.get_window(primary_entity) else {
        return;
    };
    let icon_buf = Cursor::new(include_bytes!("../assets/textures/icon.png"));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
