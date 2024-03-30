mod deck;
mod game;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use crate::game::{Command, Game};

fn decode(string: String) -> Option<Command> {
    let string = string.to_lowercase();
    let string: Vec<&str> = string.split(" ").collect();
    if let Some(&c) = string.first() {
        match c {
            "d" => Some(Command::Draw),
            "g" => Some(Command::GetState),
            "p" => {
                if let Some(&n) = string.get(1) {
                    if let Ok(n) = n.parse::<usize>() {
                        Some(Command::Place {stack: n})
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            "m" => {
                /*
                If had proper monads:
                
                let n1 = string.get(1)?
                let n2 = string.get(2)?
                let n3 = string.get(3)?
                
                let n = n1.parse()?
                let m = n2.parse()?
                let k = n3.parse()?
                
                pure(Command::Move{ from: n, to: m, amount: k })
                 */
                string.get(1)
                    .and_then(|&n1| string.get(2)
                    .and_then(|&n2| string.get(3)
                    .and_then(|&n3| n1.parse::<usize>().ok()
                    .and_then(|n| n2.parse::<usize>().ok()
                    .and_then(|m| n3.parse::<usize>().ok()
                    .and_then(|k| Some(Command::Move{from: n, to: m, amount: k})))))))
            }
            _ => None
        }
    } else {
        None
    }
}

fn main() {
    let (tx_command, rx_command) = mpsc::channel();
    let (tx_state, rx_state) = mpsc::channel();

    thread::spawn(move || {
        let mut game = Game::new(tx_state);
        loop {
            let command = rx_command.recv().unwrap();
            game.run_command(command);
        }
    });

    thread::spawn(move || {
        loop {
            let state = rx_state.recv().unwrap();
            println!("{:?}", state);
        }
    });

    let mut input = String::new();

    loop {
        input.clear();
        print!("> ");
        std::io::stdin().read_line(&mut input).unwrap();
        if let Some(command) = decode(input.clone().strip_suffix("\n").unwrap().to_string()) {
            tx_command.send(command).unwrap();
        }
        thread::sleep(Duration::from_millis(200));
    }
}
