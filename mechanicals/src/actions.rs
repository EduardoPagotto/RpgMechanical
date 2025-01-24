use crate::experience::Experience;
use crate::healt::Healt;

pub trait Actions {
    //fn new(name: String, max: i32) -> Self;
    fn level_up(&mut self, value: i32);
    fn reset(&mut self);
    fn show(&self) -> String;
}

impl Actions for Healt {
    // fn new(name: String, max: i32) -> Self {
    //     Healt::new(name, max)
    // }

    fn level_up(&mut self, value: i32) {
        self.value = value;
        self.max = value;
    }

    fn reset(&mut self) {
        self.value = self.max;
    }

    fn show(&self) -> String {
        format!("{}: {}/{}", self.name, self.value, self.max)
    }
}

impl Actions for Experience {
    // fn new(name: String, max: i32) -> Self {
    //     experience::new(name, max)
    // }

    fn level_up(&mut self, value: i32) {
        self.value = 1;
        self.max = value;
    }

    fn reset(&mut self) {
        self.value = 1;
    }

    fn show(&self) -> String {
        format!("{}: {}/{}", self.name, self.value, self.max)
    }
}
