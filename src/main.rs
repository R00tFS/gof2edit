use crate::arguments::Arguments;
use crate::commands::Command;
use clap::Parser;
use std::io;

mod arguments;
mod bin_io;
mod commands;
mod data;
mod patchers;
mod repackers;
mod targets;
mod unpackers;
mod utilities;

fn main() {
    let arguments = Arguments::parse();

    match execute_command(&arguments.command) {
        Ok(_) => {}
        Err(error) => eprintln!("Error: {error}"),
    }
}

fn execute_command(command: &Command) -> io::Result<()> {
    match command {
        Command::Unpack(command) => command.execute(),
        Command::Repack(command) => command.execute(),
        Command::Patch(command) => command.execute(),
    }
}
