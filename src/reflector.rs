// 反射是固定的，输入偏移量，输出偏移量
pub const UKW_B: &str = "YRUHQSLDPXNGOKMIEBFZCWVJAT";
pub struct Reflector {
    table: [u8; 26],
}
impl Reflector {
    pub fn new(table_string: &str) -> Reflector {
        let mut table: [u8; 26] = [0; 26];
        for (i, c) in table_string.chars().enumerate() {
            table[i] = c as u8 - 'A' as u8;
        }
        Reflector { table }
    }
    pub fn reflect(&self, offset_in: usize) -> u8 {
        self.table[offset_in]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_offset(letter: char) -> usize {
        letter as usize - 'A' as usize
    }
    #[test]
    fn test_reflect() {
        let r = Reflector::new(UKW_B);
        assert_eq!(r.reflect(get_offset('F')), get_offset('S') as u8);
        assert_eq!(r.reflect(get_offset('G')), get_offset('L') as u8);
    }
}
