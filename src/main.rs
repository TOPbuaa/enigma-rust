use std::time::Instant;
fn main() {
    let mut e = enigma::Enigma::new(
        [rotor::WHEEL_V, rotor::WHEEL_IV, rotor::WHEEL_III],
        ['A', 'Q', 'L'],
        reflector::UKW_B,
        "bq cr di ej kw mt os px uz gh",
    );
    let t1 = Instant::now();
    // 80Mbits
    for _ in 0..1000_000 {
        e.code("HELLOWORLD");
    }
    let t2 = Instant::now();
    println!(
        "time:{:?}   speed:{} Mbps",
        t2.duration_since(t1),
        10.0 * 8.0 / (t2.duration_since(t1).as_millis() as f64 / 1000.0)
    );
}

pub fn get_offset(letter: char) -> usize {
    letter as usize - 'A' as usize
}

mod enigma;
mod patch_panel;
mod reflector;
mod rotor;
