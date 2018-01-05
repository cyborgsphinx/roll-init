use std::slice::Iter;

use creature::Creature;

pub struct InitOrder {
    index: usize,
    init_list: Vec<Creature>,
    hold_list: Vec<Creature>,
}

impl<'a> InitOrder {
    pub fn new() -> Self {
        InitOrder { index: 0, init_list: Vec::new(), hold_list: Vec::new() }
    }

    // attempts to insert a creature into the initiative order
    // returns false if operation would provide ambiguous order (same initiative)
    pub fn insert(&mut self, thing: Creature) -> bool {
        let mut index = 0;
        while index < self.init_list.len() && thing < self.init_list[index] {
            index += 1;
        }
        if index == self.init_list.len() {
            self.init_list.push(thing);
            true
        } else if thing == self.init_list[index] {
            false
        } else {
            self.init_list.insert(index, thing);
            true
        }
    }

    pub fn iter(&self) -> Iter<Creature> {
        self.init_list.iter()
    }

    pub fn get_next(&'a mut self) -> Option<&'a Creature> {
        if self.init_list.len() == 0 {
            return None;
        }
        while self.index >= self.init_list.len() {
            self.index -= self.init_list.len();
        }
        self.index += 1;
        Some(&self.init_list[self.index-1])
    }
    
    pub fn delete_current(&mut self) {
        while self.index >= self.init_list.len() {
            self.index -= self.init_list.len();
        }
        self.init_list.remove(self.index);
    }

    pub fn clear(&mut self) {
        self.init_list.clear();
        self.hold_list.clear();
        self.index = 0;
    }

    // this is where things start to get weird
    // holding actions or turns messes with the whole initiative list
    pub fn hold(&'a mut self) -> &'a Creature {
        self.hold_list.push(self.init_list.remove(self.index));
        &self.init_list[self.index]
    }
    
    pub fn unhold(&mut self, _name: &str ) {
        ;
    }
}
