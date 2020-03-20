// # 子模块：1.轮盘   2.反射器   3.插线板
use crate::patch_panel::PatchPanel;
use crate::reflector::Reflector;
use crate::rotor::Rotor;

pub struct Enigma {
    panel: PatchPanel,
    reflector: Reflector,
    rotor_slow: Rotor,
    rotor_middle: Rotor,
    rotor_fast: Rotor,
}
impl Enigma {
    pub fn new(wheel: [(&str, char); 3], position: [char; 3], refl: &str, pairs: &str) -> Enigma {
        let panel = PatchPanel::new(pairs);
        let reflector = Reflector::new(refl);
        let rotor_slow = Rotor::new(wheel[0], position[0]);
        let rotor_middle = Rotor::new(wheel[1], position[1]);
        let rotor_fast = Rotor::new(wheel[2], position[2]);
        Enigma {
            panel,
            reflector,
            rotor_slow,
            rotor_middle,
            rotor_fast,
        }
    }
    pub fn single_code(&mut self, input: char) -> char {
        // rotate
        let fast_result = self.rotor_fast.rotate();
        if self.rotor_middle.middle_rotate(fast_result) {
            self.rotor_slow.rotate();
        }
        // input
        let p = self.panel.panel_in(input);
        let b = self.rotor_fast.rotor_in(p);
        let c = self.rotor_middle.rotor_in(b);
        let d = self.rotor_slow.rotor_in(c);
        // reflect
        let reflect_result = self.reflector.reflect(d);
        // output
        let f = self.rotor_slow.rotor_out(reflect_result);
        let g = self.rotor_middle.rotor_out(f);
        let h = self.rotor_fast.rotor_out(g);
        return self.panel.panel_out(h);
    }

    pub fn code(&mut self, sin: &str) -> String {
        let sin = sin.to_uppercase();
        let mut sout = String::from("");
        for c in sin.chars() {
            sout.push(self.single_code(c));
        }
        return sout;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reflector;
    use crate::rotor;
    use std::fs;
    #[test]
    fn test_singlecode() {
        let mut e = Enigma::new(
            [rotor::WHEEL_I, rotor::WHEEL_II, rotor::WHEEL_III],
            ['A', 'A', 'A'],
            reflector::UKW_B,
            "",
        );
        assert_eq!(e.single_code('A'), 'B');
        let mut e = Enigma::new(
            [rotor::WHEEL_I, rotor::WHEEL_II, rotor::WHEEL_III],
            ['N', 'C', 'Q'],
            reflector::UKW_B,
            "",
        );
        assert_eq!(e.single_code('W'), 'G');
        // middle rotor double stepping
        let mut e = Enigma::new(
            [rotor::WHEEL_I, rotor::WHEEL_II, rotor::WHEEL_III],
            ['Q', 'E', 'R'],
            reflector::UKW_B,
            "",
        );
        assert_eq!(e.single_code('S'), 'W');
    }
    #[test]
    fn test_code() {
        let mut e = Enigma::new(
            [rotor::WHEEL_III, rotor::WHEEL_I, rotor::WHEEL_II],
            ['N', 'C', 'C'],
            reflector::UKW_B,
            "",
        );
        assert_eq!(e.code("HELLOWORLD"), "TGNGVBCDBC");
        let mut e = Enigma::new(
            [rotor::WHEEL_V, rotor::WHEEL_IV, rotor::WHEEL_III],
            ['A', 'Q', 'L'],
            reflector::UKW_B,
            "bq cr di ej kw mt os px uz gh",
        );
        assert_eq!(e.code("ENIGMA"), "TKHUER");
    }
    #[test]
    fn test_enigma() {
        let mut e = Enigma::new(
            [rotor::WHEEL_I, rotor::WHEEL_II, rotor::WHEEL_III],
            ['P', 'E', 'Z'],
            reflector::UKW_B,
            "",
        );
        let mut e1 = Enigma::new(
            [rotor::WHEEL_I, rotor::WHEEL_II, rotor::WHEEL_III],
            ['P', 'E', 'Z'],
            reflector::UKW_B,
            "",
        );
        let plaintext =
            fs::read_to_string("test/plaintext").expect("Something went wrong reading the file");
        let mut ciphertext =
            fs::read_to_string("test/ciphertext").expect("Something went wrong reading the file");
        ciphertext.retain(|c| c != ' ');
        assert_eq!(e.code(&plaintext), ciphertext.to_uppercase());
        assert_eq!(e1.code(&ciphertext), plaintext);
    }
}
