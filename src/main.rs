extern crate roll_init;
extern crate linefeed;

use roll_init::initorder::InitOrder;

use linefeed::{Reader, ReadResult};
use linefeed::terminal::Terminal;

fn main() {
    println!("Welcome to roll-init");
    let mut reader = Reader::new("roll-init").unwrap();
    let mut order = InitOrder::new();
    reader.set_prompt("Menu: [a]dd, [n]ext, [p]rint, [q]uit > ");
    while let Ok(res) = reader.read_line() {
        match res {
            ReadResult::Input(input) => { //{order.insert(inp.parse().unwrap());},
                match input.to_lowercase().as_ref() {
                    "a" | "add" => {
                        add_creature(&mut reader, &mut order);
                    },
                    "n" | "next" => {
                        println!("{}", order.get_next());
                    },
                    "p" | "print" => {
                        for creature in order.iter() {
                            println!("{}", creature);
                        }
                    },
                    "q" | "quit" => {
                        break;
                    },
                    _ => {}, // do nothing
                }
            },
            _ => break,
        }
    }
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
    reader.set_prompt("Menu: [a]dd, [n]ext, [p]rint, [q]uit > ");
}
