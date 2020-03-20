// # 与第三个rotor相连接
pub struct PatchPanel {
    pairs: [usize; 26],
}
impl PatchPanel {
    pub fn new(pairs_string: &str) -> PatchPanel {
        let pairs_string = pairs_string.to_uppercase();
        let mut pairs = [0_usize; 26];
        for (i, letter) in pairs.iter_mut().enumerate() {
            *letter = i;
        }
        for pair in pairs_string.split_whitespace() {
            let mut chars = pair.chars();
            let c1 = chars.next().unwrap();
            let c2 = chars.next().unwrap();
            pairs[c1 as usize - 'A' as usize] = c2 as usize - 'A' as usize;
            pairs[c2 as usize - 'A' as usize] = c1 as usize - 'A' as usize;
        }
        PatchPanel { pairs }
    }

    pub fn panel_in(&self, letter: char) -> usize {
        self.pairs[letter as usize - 'A' as usize]
    }
    pub fn panel_out(&self, offset: usize) -> char {
        (self.pairs[offset] as u8 + 'A' as u8) as char
    }
}

#[cfg(test)]
mod tests {
    use super::super::get_offset;
    use super::*;
    #[test]
    fn test_panel() {
        let panel = PatchPanel::new("bq cr di ej kw mt os px uz gh");
        assert_eq!(panel.panel_in('C'), get_offset('R'));
        assert_eq!(panel.panel_out(get_offset('Z')), 'U');
    }
}
