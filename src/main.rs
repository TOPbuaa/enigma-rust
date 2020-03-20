fn main() {
    let r = reflector::Reflector::new(reflector::UKW_B);
    assert_eq!(r.reflect(get_offset('F')), get_offset('S'));
    assert_eq!(r.reflect(get_offset('G')), get_offset('L'));
}

pub fn get_offset(letter: char) -> usize {
    letter as usize - 'A' as usize
}
mod patch_panel;
mod reflector;
mod rotor;
