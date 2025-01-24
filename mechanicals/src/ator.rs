use crate::{actions::Actions, experience::Experience, healt::Healt};

#[derive(Default)]
pub struct Ator {
    name: String,
    level: i32,
    xp_base: i32,
    incremento: i32,
    ataque: i32,
    hp: Healt,
    xp: Experience,
}

impl Ator {
    pub fn new(name: String, level: i32, xp_base: i32, incremento: i32) -> Self {
        let base_hp = xp_base + (incremento * (level - 1));
        let base_bat = base_hp * level;

        Ator {
            name,
            level,
            xp_base,
            incremento,
            ataque: base_bat,
            hp: Healt::new(String::from("HP"), base_hp * 3),
            xp: Experience::new(String::from("XP"), base_bat * 3),
        }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }

    // pub fn get_level(&self) -> i32 {
    //     self.level
    // }

    pub fn get_hp(&mut self) -> &mut Healt {
        &mut self.hp
    }

    // pub fn get_xp(&mut self) -> &mut experience {
    //     &mut self.xp
    // }

    pub fn show(&self) -> String {
        format!(
            "{} Level: {} {} {} Dano: {}",
            &self.name,
            self.level,
            self.xp.show(), // definido em actions
            self.hp.show(), // definido em actions
            self.ataque,
        )
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        let base_hp = self.xp_base * (self.incremento * (self.level - 1));
        let base_bat = base_hp * self.level;

        self.xp.level_up(base_hp); // definido em actions
        self.hp.level_up(base_bat * 3); // definido em actions
        self.ataque = base_bat;
    }

    pub fn atacar(&mut self, oponente: &mut Ator) -> bool {
        if self.hp.get_value() <= 0 {
            //println!("---- morto ----");
            return false;
        }

        if oponente.get_hp().defesa(self.ataque) {
            return true;
        }

        let oponente_hp: &mut Healt = oponente.get_hp();
        if oponente_hp.defesa(self.ataque) {
            return true;
        }

        println!(">>>>> {} was killed\n", oponente.get_name());

        self.hp.reset(); // definido em actions
        self.xp.kill_enemy(oponente.xp.get_value());

        if self.xp.finished() {
            println!(">>>>>>> {} -> LEVEL UP !!", self.get_name());
            self.level_up();
        }

        false
    }
}
