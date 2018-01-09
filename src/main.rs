extern crate roll_init;
extern crate linefeed;

use std::fs::File;
use std::io::{self, Write};

use roll_init::initorder::InitOrder;

use linefeed::{Reader, ReadResult};
use linefeed::terminal::Terminal;

const HELP_MSG: &'static str = "[a]dd - add a creature
[c]lear - clear the initiative list
[d]elete - delete the current creature
[n]ext - get the next creature in initiative
[p]rint - print out the initiative list in its current state
[q]uit - quit roll-init
[s]ave - save initiative order to a file
? - print this message";

const MAIN_PROMPT: &'static str = "> ";

enum Action {
    Break,
    Continue,
}

fn main() {
    println!("Welcome to roll-init");
    println!("use \"?\" to display possible actions");
    let mut reader = Reader::new("roll-init").unwrap();
    let mut order = InitOrder::new();
    reader.set_prompt(MAIN_PROMPT);
    while let Ok(res) = reader.read_line() {
        match res {
            ReadResult::Input(input) => {
                match dispatch(input.to_lowercase().as_ref(), &mut reader, &mut order) {
                    Action::Break => break,
                    _ => {},
                }
            },
            _ => break,
        }
    }
}

fn dispatch<T: Terminal>(input: &str, reader: &mut Reader<T>, mut order: &mut InitOrder) -> Action {
    match input {
        "a" | "add" => {
            reader.set_prompt("Name and Initiative > ");
            add_creature(reader.read_line(), &mut order);
            reader.set_prompt(MAIN_PROMPT);
        },
        "c" | "clear" => {
            order.clear();
        },
        "d" | "delete" => {
            order.delete_current();
        },
        "n" | "next" => {
            if let Some(creature) = order.get_next() {
                println!("{}", creature);
            } else {
                println!("Unable to get next creature");
            }
        },
        "p" | "print" => {
            for creature in order.iter() {
                println!("{}", creature);
            }
        },
        "q" | "quit" => {
            return Action::Break;
        },
        "s" | "save" => {
            reader.set_prompt("Save to [init.csv] > ");
            if save(&order, reader.read_line()).is_err() {
                println!("Could not save to file");
            }
            reader.set_prompt(MAIN_PROMPT);
        },
        "?" => {
            println!("{}", HELP_MSG);
        }
        _ => {}, // do nothing
    }
    Action::Continue
}

fn add_creature(input: Result<ReadResult, io::Error>, order: &mut InitOrder) {
    match input {
        Ok(res) => {
            match res {
                ReadResult::Input(inp) => {
                    order.insert(inp.parse().unwrap());
                },
                _ => {},
            }
        },
        _ => {
            println!("Could not understand that");
        },
    }
}

fn save(order: &InitOrder, input: Result<ReadResult, io::Error>) -> Result<(), io::Error> {
    let def = String::from("init.csv");
    let filename = match input {
        Ok(res) => {
            match res {
                ReadResult::Input(inp) => {
                    if inp.is_empty() { def } else { inp }
                },
                _ => def,
            }
        },
        _ => def,
    };
    let mut file = File::create(filename)?;
    for creature in order.iter() {
        let _ = writeln!(file, "{}", creature)?;
    }
    Ok(())
}
