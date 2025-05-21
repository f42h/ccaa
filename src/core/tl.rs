use crate::tern;

pub(crate) struct Tl<'a> {
    pub(super) input: &'a Vec<u8>
}

impl<'a> Tl<'a> {
    pub(crate) fn new(input: &'a Vec<u8>) -> Self {
        Self { input }
    }

    fn a(&self, i: u8) -> char {
        tern!(i > 26, i as char, (b'a' + i) as char)
    }

    fn b(&self) {
        if self.input.is_empty() {
            return; 
        }

        let s = self.input[0] == 2;
        if s {
            print!("{}", self.a(0x43));
        }
        
        for e in self.input.iter().skip(tern!(s, 1, 0)) {
            print!("{}", self.a(*e));
        }
    }

    pub(crate) fn _do(&self) {
        self.b();

        if self.input.len() != 1 {
              print!(" ");
        }
    }
}