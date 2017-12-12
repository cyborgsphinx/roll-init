use std::slice::Iter;

use creature::Creature;

pub struct InitOrder {
    index: usize,
    init_list: Vec<Creature>
}

impl<'a> InitOrder {
    pub fn new() -> Self {
        InitOrder { index: 0, init_list: Vec::new() }
    }

    // attempts to insert a creature into the initiative order
    // returns false if operation would provide ambiguous order (same initiative)
    pub fn insert(&mut self, thing: Creature) -> bool {
        self.init_list.push(thing);
        self.init_list.sort();
        self.init_list.reverse();
        true // assuming true for now
    }

    pub fn iter(&self) -> Iter<Creature> {
        self.init_list.iter()
    }

    pub fn get_next(&'a mut self) -> &'a Creature {
        while self.index >= self.init_list.len() {
            self.index -= self.init_list.len();
        }
        self.index += 1;
        &self.init_list[self.index-1]
    }
}
