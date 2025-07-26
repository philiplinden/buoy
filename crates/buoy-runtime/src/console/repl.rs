use std::sync::{mpsc::{self, Receiver, Sender}, Mutex};
use std::thread;

use bevy::prelude::*;
use crate::console::commands::CommandRegistry;
use crate::sequencing::SimulationPipeline;

// Resource to hold the command queue safely
#[derive(Resource)]
pub struct CommandQueue(pub Mutex<Receiver<String>>);

// System to execute commands from the queue
pub fn command_executor(
    queue: Res<CommandQueue>,
    registry: Res<CommandRegistry>,
    world: &mut World,
) {
    let queue = queue.0.lock().unwrap();
    while let Ok(command) = queue.try_recv() {
        match registry.execute(&command, world) {
            Ok(result) => {
                if !result.is_empty() {
                    println!("{}", result);
                }
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }
}

// TUI plugin skeleton
pub struct ReplPlugin;

impl Plugin for ReplPlugin {
    fn build(&self, app: &mut App) {
        // Add the command executor system to the Console system set
        app.add_systems(Update, command_executor.in_set(SimulationPipeline::Console));
        // Add TUI systems here (drawing, stats, etc.)
    }
}

// Function to spawn the REPL input thread
pub fn start_repl_input(tx: Sender<String>) {
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
            if !input.is_empty() {
                let _ = tx.send(input.to_string());
            }
        }
    });
}

pub fn setup_repl(app: &mut App) {
    let (tx, rx) = mpsc::channel();
    app.insert_resource(CommandQueue(Mutex::new(rx)));
    app.insert_resource(CommandRegistry::default());
    app.add_systems(Startup, crate::console::commands::register_default_commands);
    start_repl_input(tx);
    app.add_plugins(ReplPlugin);
}
