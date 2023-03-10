// main.rs - sued, the text editor of all time, short for Shut Up Editor

use std::io;
use std::fs;
use std::process::Command;
use which::which;
use rand::Rng;

fn startup_message() {
    let messages: Vec<&str> = vec![
        "the editor of all time",
        "shut up and edit",
        "the nonstandard text editor",
        "sued as in editor, not as in law",
        "sued, man! ~run man sued",
        "there is no visual mode",
        "the itor fell off",
    ];
    let message: &str = messages[rand::thread_rng().gen_range(0..messages.len())];
    println!("sued - {message}\ntype ~ for commands, otherwise just start typing");
}

fn display_help() {
    println!("~save, ~open, ~show, ~run, ~exit, ~help");
}

fn extended_help() {
    println!("sued is a line editor, like ed but also not at all\n\
              to write stuff, just start typing after the welcome message\n\
              editor commands are prefixed with ~ (for example ~exit to quit the editor)\n\
              there's no regex stuff or syntax highlighting or anything like that. you just write\n\
              sued written by Arsalan Kazmi (That1M8Head)");
}

fn save() {
    println!("sorry, can't save yet"); // TODO write the file buffer first. then implement saving
}

fn show() {
    println!("sorry, no file buffer yet"); // TODO write the damn file buffer already
}

fn open(command_args: Vec<&str>) -> &str {
    if command_args.len() <= 1 {
        println!("open what?");
        return "";
    }
    else {
        return command_args[1];
    }
}

fn write(to_write: &str) {
    // TODO implement file buffer, then implement writing to buffer
    println!("{}", to_write);
}

fn shell_command(mut command_args: Vec<&str>) {
    if command_args.len() <= 1 {
        println!("run what?");
    }
    else {
        let cmdpath = which(command_args[1]);
        let command = command_args[1];
        match which(command) {
            Ok(_) => println!("running {}", command),
            Err(_) => {
                println!("no such command");
                return;
            }
        }
        command_args.drain(0..2);
        Command::new(cmdpath.clone().unwrap())
            .args(command_args.clone())
            .status()
            .expect("broken!");
        println!("finished running {}", command);
    }
}

fn main() {
    startup_message();
    let mut command: String = String::new();
    while !command.eq("~exit") {
        command.clear();
        io::stdin()
            .read_line(&mut command)
            .expect("can't read command");
        let len: usize = command.trim_end_matches(&['\r', '\n'][..]).len();
        command.truncate(len);
        let command_args = command.split(" ").collect::<Vec<&str>>();
        // TODO replace with editor functionality
        let _cmdproc: () = match command_args[0] {
            "~"     => { display_help(); },
            "~help" => { extended_help(); },
            "~save" => { save(); },
            "~show" => { show(); },
            "~open" => { println!("{}", open(command_args)); },
            "~run"  => { shell_command(command_args); },
            "~exit" => {},
            _       => { if command_args[0].starts_with("~") { println!("{} is an unknown command", command_args[0]); } else { write(command.as_str()); } }
        };
    }
}
