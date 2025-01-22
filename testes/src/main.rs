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
    dbg!(json);


    let mut heroi = Ator::new(String::from("Heroi"), 1, 5, 5);

    // Como array
    // let mut monstros = [
    //     Ator::new(String::from("Monstro01"), 1, 4, 4),
    //     Ator::new(String::from("Monstro02"), 1, 4, 4),
    //     Ator::new(String::from("Monstro03"), 1, 4, 4),
    //     Ator::new(String::from("Monstro04"), 3, 10, 10),
    // ];

    let mut monstros: Vec<Ator> = Vec::new();
    monstros.push(Ator::new(String::from("Monstro01"), 1, 4, 4));
    monstros.push(Ator::new(String::from("Monstro02"), 1, 4, 4));
    monstros.push(Ator::new(String::from("Monstro03"), 1, 4, 4));
    monstros.push(Ator::new(String::from("Monstro04"), 1, 4, 4));

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

    // Como vetor
    // for n in 0..monstros.len() {
    //     let mut turno = 1;
    //     while war(&mut heroi, &mut monstros[n], turno) {
    //         turno += 1;
    //     }

    //     if monstros[n].get_hp().get_value() > 0 {
    //         println!("Monstro venceu!!");
    //         return;
    //     }
    // }

    println!("Heroi venceu!!");


}

