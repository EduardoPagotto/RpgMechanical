use std::fs;

use mechanicals::ator::Ator;

fn war(heroi: &mut Ator, oponente: &mut Ator, turno: i32) -> bool {
    println!(">>>>>[ {} ]>>>>>", turno);
    println!("{}", heroi.show());
    println!("{}", oponente.show());
    let r1 = heroi.atacar(oponente);
    let r2 = oponente.atacar(heroi);
    println!("{}", heroi.show());
    println!("{}", oponente.show());

    println!("<<<<<[ {} ]<<<<<<", turno);

    r1 & r2
}

fn main() {
    let data = fs::read_to_string("./assets/dados.json").expect("Impossivel ler arquivo");
    let json: serde_json::Value = serde_json::from_str(&data).expect("JSON masl formatado");
    //dbg!(json);

    let e_atores: &serde_json::Value = json.get("atores").unwrap();
    let atores = e_atores.as_array().unwrap();

    let mut heroi: Ator = Default::default();
    let mut monstros: Vec<Ator> = Vec::new();

    for v in atores {
        let name = v["name"].as_str().unwrap();
        let level = v["level"].as_i64().unwrap();
        let base_xp = v["base_xp"].as_i64().unwrap();
        let incremento = v["incremento"].as_i64().unwrap();
        //dbg!(name, level, base_xp, incremento);

        match name {
            "Heroi" => {
                heroi = Ator::new(
                    String::from(name),
                    level as i32,
                    base_xp as i32,
                    incremento as i32,
                );
            }
            _ => {
                monstros.push(Ator::new(
                    String::from(name),
                    level as i32,
                    base_xp as i32,
                    incremento as i32,
                ));
            }
        }
    }

    for monstro in &mut monstros {
        let mut turno = 1;
        while war(&mut heroi, monstro, turno) {
            turno += 1;
        }

        if monstro.get_hp().get_value() > 0 {
            println!("Monstro venceu!!");
            return;
        }
    }

    println!("Heroi venceu!!");
}
