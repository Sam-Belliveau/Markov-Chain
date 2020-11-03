extern crate rand;

use crate::char_set;
use rand::Rng;

type Count = i32;

#[derive(Clone)]
pub struct CharProb {
    total: Count,
    probs: [Count; char_set::LENGTH]
}

impl CharProb {

    pub fn new() -> Self {
        Self {
            total: 0,
            probs: [0 as Count; char_set::LENGTH]
        }
    }

    pub fn add(&mut self, l: char) {
        self.total += 1;
        self.probs[char_set::get_id(l)] += 1;
    }

    pub fn get_char(&self) -> char {
        let mut rng = rand::thread_rng();
        
        if 0 == self.total {
            // THIS IS EXTREMELY UNLIKELY
            // UNLESS YOU ARE TRYING TO FORCE THIS
            return '?';
        }

        let mut rand_id = rng.gen_range(0, self.total);

        for (id, i) in self.probs.iter().enumerate() {
            rand_id -= i;
            if rand_id < 0  {
                return char_set::get_char(id as char_set::IDType);
            }
        }

        return char_set::get_char(0);
    }


    pub fn print(&self) {
        for i in self.probs.iter() {
            println!("{}", i);
        }
    }
}