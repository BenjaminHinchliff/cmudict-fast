use cmudict_fast::Cmudict;

fn main() {
    let dict = Cmudict::new("./resources/cmudict.dict").expect("couldn't load dict");
    let word = dict.get("apple").unwrap().pronunciation();
    println!("{:?}", word); // &[Symbol::AE(Stress::Primary), Symbol::P, Symbol::AH(Stress::None), Symbol::L]
}
