#[allow(dead_code)]
pub struct Healt {
    pub name: String,
    pub value: i32,
    pub max: i32,
}

impl Healt {
    pub fn new(name: String, max: i32) -> Self {
        Healt {
            name,
            value: max,
            max,
        }
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }

    pub fn defesa(&mut self, value: i32) -> bool {
        self.value -= value;

        if self.value <= 0 {
            self.value = 0;
            false
        } else {
            true
        }
    }
}
