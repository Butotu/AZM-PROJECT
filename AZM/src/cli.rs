use std::io::stdin;

use crate::scam;

pub fn options_cli() {
    println!("1 ou 2?");
    let mut input1 = String::new();
    stdin().read_line(&mut input1).unwrap();

    match input1.trim() {
        "1" => scam::scam_rapido(),
        "2" => scam::scam_longo(),
        _ => println!("Opção inválida! Digite 1 ou 2."),
    }
}
