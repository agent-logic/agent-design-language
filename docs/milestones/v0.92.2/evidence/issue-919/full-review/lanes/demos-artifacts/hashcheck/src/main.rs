fn main(){for p in std::env::args().skip(1){println!("{} {}",blake3::hash(&std::fs::read(&p).unwrap()),p);}}
