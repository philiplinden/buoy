use bevy::{
    app::{AppExit, ScheduleRunnerPlugin},
    prelude::*,
};
use bevy_ratatui::{
    error::exit_on_error, event::KeyEvent, terminal::RatatuiContext, RatatuiPlugins,
};
fn main() {
    let wait_duration = std::time::Duration::from_secs_f64(1. / 60.); // 60 FPS
    App::new()
        .add_plugins(RatatuiPlugins::default())
        .add_plugins(ScheduleRunnerPlugin::run_loop(wait_duration))
        .add_systems(PreUpdate, keyboard_input_system)
        .add_systems(Update, hello_world.pipe(exit_on_error))
        .run();
}

fn hello_world(mut context: ResMut<RatatuiContext>) -> color_eyre::Result<()> {
    context.draw(|frame| {
        let text = ratatui::text::Text::raw("hello world\nPress 'q' to Quit");
        frame.render_widget(text, frame.area())
    })?;
    Ok(())
}

fn keyboard_input_system(mut events: EventReader<KeyEvent>, mut exit: EventWriter<AppExit>) {
    use crossterm::event::KeyCode;
    for event in events.read() {
        match event.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                exit.send_default();
            }
            _ => {}
        }
    }
}
