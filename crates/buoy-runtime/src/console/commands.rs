use bevy::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Command handler trait for hooks
pub trait CommandHandler: Send + Sync {
    fn handle(&self, args: &[&str], world: &mut World) -> Result<String, String>;
}

// Command registry to store handlers
#[derive(Resource)]
pub struct CommandRegistry {
    handlers: Arc<Mutex<HashMap<String, Box<dyn CommandHandler>>>>,
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl CommandRegistry {
    pub fn register<H: CommandHandler + 'static>(&self, name: &str, handler: H) {
        if let Ok(mut handlers) = self.handlers.lock() {
            handlers.insert(name.to_string(), Box::new(handler));
        }
    }

    pub fn execute(&self, command: &str, world: &mut World) -> Result<String, String> {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return Err("Empty command".to_string());
        }

        let command_name = parts[0];
        let args = &parts[1..];

        if let Ok(handlers) = self.handlers.lock() {
            if let Some(handler) = handlers.get(command_name) {
                handler.handle(args, world)
            } else {
                Err(format!("Unknown command: {}", command_name))
            }
        } else {
            Err("Failed to access command registry".to_string())
        }
    }

    pub fn list_commands(&self) -> Vec<String> {
        if let Ok(handlers) = self.handlers.lock() {
            handlers.keys().cloned().collect()
        } else {
            Vec::new()
        }
    }
}

// Built-in command handlers
pub struct ExitHandler;

impl CommandHandler for ExitHandler {
    fn handle(&self, _args: &[&str], world: &mut World) -> Result<String, String> {
        world.send_event(AppExit::Success);
        Ok("Exiting...".to_string())
    }
}

pub struct HelpHandler;

impl CommandHandler for HelpHandler {
    fn handle(&self, _args: &[&str], world: &mut World) -> Result<String, String> {
        let registry = world.resource::<CommandRegistry>();
        let commands = registry.list_commands();
        let help_text = format!("Available commands: {}", commands.join(", "));
        Ok(help_text)
    }
}

pub struct EchoHandler;

impl CommandHandler for EchoHandler {
    fn handle(&self, args: &[&str], _world: &mut World) -> Result<String, String> {
        if args.is_empty() {
            Ok("".to_string())
        } else {
            Ok(args.join(" "))
        }
    }
}

// System to register default commands
pub fn register_default_commands(registry: Res<CommandRegistry>) {
    registry.register("exit", ExitHandler);
    registry.register("help", HelpHandler);
    registry.register("echo", EchoHandler);
}
