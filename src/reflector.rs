// 反射是固定的，输入偏移量，输出偏移量
pub const UKW_B: &str = "YRUHQSLDPXNGOKMIEBFZCWVJAT";
pub struct Reflector {
    table: [usize; 26],
}
impl Reflector {
    pub fn new(table_string: &str) -> Reflector {
        let mut table = [0_usize; 26];
        for (i, c) in table_string.chars().enumerate() {
            table[i] = c as usize - 'A' as usize;
        }
        Reflector { table }
    }
    pub fn reflect(&self, offset_in: usize) -> usize {
        self.table[offset_in]
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    #[test]
    fn test_reflect() {
        let r = Reflector::new(UKW_B);
        assert_eq!(r.reflect(get_offset('F')), get_offset('S'));
        assert_eq!(r.reflect(get_offset('G')), get_offset('L'));
    }
}
