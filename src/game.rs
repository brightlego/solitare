use std::collections::VecDeque;
use crate::deck::{Card, Deck, standard};
use std::sync::mpsc::Sender;
use arr_macro::arr;

#[derive(Debug)]
pub enum Command {
    Draw,
    Move {from: usize, to: usize, amount: usize},
    Place {stack: usize},
    GetState
}

#[derive(Clone, Debug)]
pub struct State {
    stacks: [VecDeque<Card>; 8],
    unplaced: VecDeque<Card>,
    deck: Deck<Card, 52>,
    out_top: [Option<Card>; 4],
}

impl State {
    fn new() -> Self {
        State {
            stacks: arr![VecDeque::new(); 8],
            unplaced: VecDeque::new(),
            deck: standard(),
            out_top: [None; 4]
        }
    }
}


pub struct Game {
    state: State,
    tx: Sender<State>
}

impl Game {

    pub fn new(tx: Sender<State>) -> Self {
        Game {
            state: State::new(),
            tx
        }
    }

    fn init(&mut self) {
        for i in 0..self.state.stacks.len() {
            let cards = self.state.deck.draw_many(i+1);
            for card in cards {
                self.state.stacks[i].push_back(card.clone());
            }
        }
    }

    fn draw(&mut self) {
        if self.state.deck.is_full() {
            self.init();
        }

        let cards = self.state.deck.draw_many(3);
        for card in cards {
            self.state.unplaced.push_back(card.clone())
        }
    }

    fn move_cards(&mut self, from: usize, to: usize, amount: usize) {
        let mut cards: VecDeque<Card> = VecDeque::with_capacity(amount);
        if let Some(stack_from) = self.state.stacks.get_mut(from) {
            for _ in 0..amount {
                if let Some(card) = stack_from.pop_back() {
                    cards.push_back(card);
                }
            }
        }

        if let Some(stack_to) = self.state.stacks.get_mut(to) {
            while !cards.is_empty() {
                if let Some(card) = cards.pop_front() {
                    stack_to.push_back(card);
                } else {
                    panic!("Unable to get an item from a non-empty deque")
                }
            }
        }
    }

    fn place(&mut self, stack: usize) {
        if let Some(stack) = self.state.stacks.get_mut(stack) {
            if let Some(card) = self.state.unplaced.pop_back() {
                stack.push_back(card);
            }
        }
    }

    pub fn run_command(&mut self, command: Command) {
        use Command::*;
        match command {
            Draw => self.draw(),
            Move {from, to, amount} => self.move_cards(from, to, amount),
            Place {stack} => self.place(stack),
            GetState => self.tx.send(self.state.clone()).unwrap()
        }
    }
}

