#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
struct Var {
    index: u8, // only using 6 bits
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Formula {
    Var(Var),
    Nor(Vec<Formula>),
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
        if index <= 0b111111 {
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
    fn contains(self, var: Var) -> bool {
        self != self.removed(var)
    }
    fn is_subset(self, other: Self) -> bool {
        self.differed(other) == Self::default()
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
impl Formula {
    fn normalize(self) -> Self {
        match self {
            x @ Self::Var(_) => x,
            Self::Nor(mut formulae) => {
                if let [only] = &mut formulae[..] {
                    // special case: drop double negation
                    let mut dummy = Self::Var(VAR[0]);
                    std::mem::swap(&mut dummy, only);
                    return dummy.normalize();
                } else {
                    let mut formulae: Vec<Self> =
                        formulae.into_iter().map(Self::normalize).collect();
                    formulae.sort();
                    formulae.dedup();
                    Self::Nor(formulae)
                }
            }
        }
    }
    fn top() -> Self {
        Self::Nor(vec![])
    }
    fn bot(self) -> Self {
        Self::top().not()
    }
    fn not(self) -> Self {
        Self::Nor(vec![self])
    }
    fn nor(formulae: Vec<Self>) -> Self {
        Self::Nor(formulae)
    }
    fn or(formulae: Vec<Self>) -> Self {
        Self::nor(formulae).not()
    }
    fn and(formulae: Vec<Self>) -> Self {
        Self::nor(formulae.into_iter().map(Self::not).collect())
    }
    fn nand(formulae: Vec<Self>) -> Self {
        Self::and(formulae).not()
    }
    fn var(var: Var) -> Self {
        Self::Var(var)
    }
}
impl Kb {
    fn test_var(&self, var: Var) -> Option<bool> {
        if self.vars_true.contains(var) {
            Some(true)
        } else if self.vars_fals.contains(var) {
            Some(false)
        } else {
            None
        }
    }
    fn test_formula(&self, formula: &Formula) -> Option<bool> {
        match formula {
            Formula::Var(var) => self.test_var(*var),
            Formula::Nor(formulae) => {
                let mut saw_u = false;
                for f in formulae {
                    match self.test_formula(f) {
                        Some(true) => return Some(false),
                        None => saw_u = true,
                        Some(false) => {}
                    }
                }
                match saw_u {
                    true => None,
                    false => Some(true),
                }
            }
        }
    }
}

fn main() {
    use VAR as V;
    let kb = Kb {
        vars_true: VarSet::from_iter([V[0], V[1]]), //true
        vars_fals: VarSet::from_iter([V[2]]),
    };
}
