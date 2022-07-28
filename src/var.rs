use std::fmt;

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Var {
    index: u8, // only using 6 bits
}

pub const VAR: [Var; 4] = [Var { index: 0 }, Var { index: 1 }, Var { index: 2 }, Var { index: 3 }];

impl Var {
    pub const TOP: Self = Self { index: 0 };
    pub fn new(index: u8) -> Option<Self> {
        if index <= 0b111111 {
            Some(Self { index })
        } else {
            None
        }
    }
    pub fn index(self) -> u8 {
        self.index
    }
}
impl fmt::Debug for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("v{}", self.index))
    }
}
