#[allow(dead_code)]
#[derive(Default, Debug, PartialEq)]
pub struct Experience {
    pub name: String,
    pub value: i32,
    pub max: i32,
}

impl Experience {
    pub fn new(name: String, max: i32) -> Self {
        Experience {
            name,
            value: 1,
            max,
        }
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }

    pub fn finished(&self) -> bool {
        self.value >= self.max
    }

    pub fn kill_enemy(&mut self, value: i32) {
        self.value += value
    }
}
