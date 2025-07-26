use std::sync::{mpsc::{self, Receiver, Sender}, Mutex};
use std::thread;

use bevy::prelude::*;

// Minimal command enum for REPL
#[derive(Debug)]
pub enum Command {
    Exit,
}

// Resource to hold the command queue safely
#[derive(Resource)]
pub struct CommandQueue(pub Mutex<Receiver<Command>>);

// System to execute commands from the queue
pub fn command_executor(
    queue: Res<CommandQueue>,
    mut exit_events: EventWriter<AppExit>,
) {
    let queue = queue.0.lock().unwrap();
    while let Ok(cmd) = queue.try_recv() {
        match cmd {
            Command::Exit => {
                exit_events.write(AppExit::Success);
            }
            // Handle other commands
        }
    }
}

// TUI plugin skeleton
pub struct ReplPlugin;

impl Plugin for ReplPlugin {
    fn build(&self, app: &mut App) {
        // Add the command executor system
        app.add_systems(Update, command_executor);
        // Add TUI systems here (drawing, stats, etc.)
    }
}

// Function to spawn the REPL input thread
pub fn start_repl_input(tx: Sender<Command>) {
    thread::spawn(move || {
        use std::io::{self, Write};
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                break;
            }
            let input = input.trim();
            // Minimal parser: setfoo <val>
            if input == "exit" {
                let _ = tx.send(Command::Exit);
            }
            // Add more parsing as needed
        }
    });
}

pub fn setup_repl(app: &mut App) {
    let (tx, rx) = mpsc::channel();
    app.insert_resource(CommandQueue(Mutex::new(rx)));
    start_repl_input(tx);
    app.add_plugins(ReplPlugin);
}
