#[derive(Debug, Clone, Copy)]
struct Var {
    index: u8, // only using 6 bits
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
struct VarSet {
    bit_set: u64,
}

#[derive(Debug, Default)]
struct Kb {
    vars_true: VarSet,
    vars_fals: VarSet,
}
const VAR: [Var; 4] = [Var { index: 0 }, Var { index: 1 }, Var { index: 2 }, Var { index: 3 }];
impl Var {
    fn new(index: u8) -> Option<Self> {
        if index < 0b111111 {
            Some(Self { index })
        } else {
            None
        }
    }
}

impl VarSet {
    fn singleton(var: Var) -> Self {
        Self { bit_set: 1 << var.index }
    }
    fn from_iter(vars: impl IntoIterator<Item = Var>) -> Self {
        let mut me = Self::default();
        for var in vars {
            me.add(var.into());
        }
        me
    }
    fn add(&mut self, var: Var) {
        *self = self.added(var)
    }
    fn remove(&mut self, var: Var) {
        *self = self.removed(var)
    }
    fn added(self, var: Var) -> Self {
        self.unified(Self::singleton(var))
    }
    fn removed(self, var: Var) -> Self {
        self.differed(Self::singleton(var))
    }
    fn differed(self, other: Self) -> Self {
        Self { bit_set: self.bit_set & !other.bit_set }
    }
    fn unified(self, other: Self) -> Self {
        Self { bit_set: self.bit_set | other.bit_set }
    }
    fn intersected(self, other: Self) -> Self {
        Self { bit_set: self.bit_set & other.bit_set }
    }
}

fn main() {
    use VAR as V;
    let kb = Kb {
        vars_true: VarSet::from_iter([V[0], V[1]]), //true
        vars_fals: VarSet::from_iter([V[2]]),
    };
}
