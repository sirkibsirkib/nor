use crate::var::Var;
use std::fmt;

#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct VarSet {
    bit_set: u64,
}
pub struct VarSetIter {
    remaining: VarSet,
}

impl fmt::Debug for VarSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.into_iter()).finish()
    }
}

impl Iterator for VarSetIter {
    type Item = Var;
    fn next(&mut self) -> Option<Var> {
        self.remaining.take()
    }
}
impl VarSet {
    pub fn singleton(var: Var) -> Self {
        Self { bit_set: 1 << var.index() }
    }
    pub fn into_iter(self) -> VarSetIter {
        VarSetIter { remaining: self }
    }
    pub fn take(&mut self) -> Option<Var> {
        let trailing = self.bit_set.trailing_zeros() as u8;
        if trailing < 64 {
            let var = Var::new(trailing)?;
            self.remove(var);
            Some(var)
        } else {
            None
        }
    }
    pub fn from_iter(vars: impl IntoIterator<Item = Var>) -> Self {
        let mut me = Self::default();
        for var in vars {
            me.add(var.into());
        }
        me
    }
    pub fn contains(self, var: Var) -> bool {
        self != self.removed(var)
    }
    pub fn is_subset(self, other: Self) -> bool {
        self.differed(other) == Self::default()
    }
    pub fn add(&mut self, var: Var) {
        *self = self.added(var)
    }
    pub fn remove(&mut self, var: Var) {
        *self = self.removed(var)
    }
    pub fn added(self, var: Var) -> Self {
        self.unified(Self::singleton(var))
    }
    pub fn removed(self, var: Var) -> Self {
        self.differed(Self::singleton(var))
    }
    pub fn differed(self, other: Self) -> Self {
        Self { bit_set: self.bit_set & !other.bit_set }
    }
    pub fn unified(self, other: Self) -> Self {
        Self { bit_set: self.bit_set | other.bit_set }
    }
    pub fn intersected(self, other: Self) -> Self {
        Self { bit_set: self.bit_set & other.bit_set }
    }
}
