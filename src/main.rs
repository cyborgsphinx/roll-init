extern crate roll_init;
extern crate linefeed;

use roll_init::initorder::InitOrder;

use linefeed::{Reader, ReadResult};
use linefeed::terminal::Terminal;

const HELP_MSG: &'static str = "[a]dd - add a creature
[c]lear - clear the initiative list
[d]elete - delete the current creature
[n]ext - get the next creature in initiative
[p]rint - print out the initiative list in its current state
[q]uit - quit roll-init
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

fn dispatch<T: Terminal>(input: &str, mut reader: &mut Reader<T>, mut order: &mut InitOrder) -> Action {
    match input {
        "a" | "add" => {
            add_creature(&mut reader, &mut order);
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
        "?" => {
            println!("{}", HELP_MSG);
        }
        _ => {}, // do nothing
    }
    Action::Continue
}

fn add_creature<T: Terminal>(reader: &mut Reader<T>, order: &mut InitOrder) {
    reader.set_prompt("Name and Initiative > ");
    match reader.read_line() {
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
    reader.set_prompt(MAIN_PROMPT);
}
