// # 旋转
// # rotor_in:输入偏移量，输出偏移量
// # rotor_out:输入偏移量，输出偏移量

#[allow(dead_code)]
pub const WHEEL_I: (&str, char) = ("EKMFLGDQVZNTOWYHXUSPAIBRCJ", 'Q');
#[allow(dead_code)]
pub const WHEEL_II: (&str, char) = ("AJDKSIRUXBLHWTMCQGZNPYFVOE", 'E');
#[allow(dead_code)]
pub const WHEEL_III: (&str, char) = ("BDFHJLCPRTXVZNYEIWGAKMUSQO", 'V');
#[allow(dead_code)]
pub const WHEEL_IV: (&str, char) = ("ESOVPZJAYQUIRHXLNFTGKDCMWB", 'J');
#[allow(dead_code)]
pub const WHEEL_V: (&str, char) = ("VZBRGITYUPSDNHLXAWMJQOFECK", 'Z');

pub struct Rotor {
    turnover: usize,
    position: usize,
    forward: [usize; 26],
    reverse: [usize; 26],
}
impl Rotor {
    pub fn new(wheel: (&str, char), position: char) -> Rotor {
        let (table_string, turnover) = wheel;
        let turnover = turnover as usize - 'A' as usize;
        let position = position as usize - 'A' as usize;
        let mut forward = [0_usize; 26];
        let mut reverse = [0_usize; 26];
        for (i, letter) in table_string.chars().enumerate() {
            forward[i] = letter as usize - 'A' as usize;
            reverse[letter as usize - 'A' as usize] = i;
        }
        Rotor {
            turnover,
            position,
            forward,
            reverse,
        }
    }
    pub fn rotate(&mut self) -> bool {
        let mut parent_rot = false;
        if self.position == self.turnover {
            parent_rot = true;
        }
        self.position = (self.position + 1) % 26;
        parent_rot
    }
    pub fn middle_rotate(&mut self, fast_result: bool) -> bool {
        if fast_result || self.position == self.turnover {
            self.rotate()
        } else {
            false
        }
    }
    pub fn rotor_in(&self, offset_in: usize) -> usize {
        let offset_in = (offset_in + self.position) % 26;
        let offset_out = (self.forward[offset_in] + 26 - self.position) % 26;
        return offset_out;
    }
    pub fn rotor_out(&self, offset_in: usize) -> usize {
        let offset_in = (offset_in + self.position) % 26;
        let offset_out = (self.reverse[offset_in as usize] + 26 - self.position) % 26;
        return offset_out;
    }
}

#[cfg(test)]
mod tests {
    use super::super::get_offset;
    use super::*;
    #[test]
    fn test_rotor() {
        let rotor1 = Rotor::new(WHEEL_I, 'A');
        assert_eq!(rotor1.rotor_in(get_offset('D')), get_offset('F'));
        assert_eq!(rotor1.rotor_out(get_offset('S')), get_offset('S'));

        let rotor2 = Rotor::new(WHEEL_II, 'A');
        assert_eq!(rotor2.rotor_in(get_offset('C')), get_offset('D'));
        assert_eq!(rotor2.rotor_out(get_offset('S')), get_offset('E'));

        let rotor3 = Rotor::new(WHEEL_III, 'I');
        assert_eq!(rotor3.rotor_in(get_offset('C')), get_offset('P'));
        assert_eq!(rotor3.rotor_out(get_offset('O')), get_offset('J'));
    }
}
