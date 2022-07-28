use crate::kb::Kb;
use crate::var::Var;
use crate::vec_mut_q::VecMutQ;
use core::fmt::Display;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Formula {
    Var { var: Var },
    And { formulae: Vec<Formula> },
    Not { formula: Box<Formula> },
}

impl fmt::Debug for Formula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Var { var } => var.fmt(f),
            Self::And { formulae } => {
                // f.write_fmt(format_args!("("))?;
                // for formula in formulae {
                //     formula.fmt(f)?;
                // }
                // f.write_fmt(format_args!(")"))
                f.debug_list().entries(formulae).finish()
            }
            // Self::Not { formula } => f.debug_tuple("¬").field(formula).finish(),
            Self::Not { formula } => {
                Display::fmt(&'¬', f)?;
                formula.fmt(f)
            }
        }
    }
}

impl Formula {
    pub fn top() -> Self {
        Self::And { formulae: vec![] }
    }
    pub fn var(var: Var) -> Self {
        Self::Var { var }
    }
    pub fn constant(true_false: bool) -> Self {
        match true_false {
            true => Self::top(),
            false => Self::bottom(),
        }
    }
    pub fn bottom() -> Self {
        Self::top().not()
    }
    pub fn not(self) -> Self {
        Self::Not { formula: Box::new(self) }
    }
    pub fn and(self, other: Self) -> Self {
        Self::And { formulae: vec![self, other] }
    }
    pub fn nand(self, other: Self) -> Self {
        self.and(other).not()
    }
    pub fn nor(self, other: Self) -> Self {
        self.not().and(other.not())
    }
    pub fn or(self, other: Self) -> Self {
        self.nor(other).not()
    }
    pub fn yimpl(self, other: Self) -> Self {
        self.nimpl(other).not()
    }
    pub fn nimpl(self, other: Self) -> Self {
        self.and(other.not())
    }
    pub fn and_flatten(self) -> Self {
        match self {
            Self::And { formulae: mut outer } => {
                let mut vmq = VecMutQ::new(&mut outer);
                while let Some(formula) = vmq.take_unprocessed() {
                    match formula {
                        Self::And { formulae: inner } => vmq.extend_processed(inner),
                        x => vmq.add_processed(x),
                    }
                }
                Self::And { formulae: outer }
            }
            x => x,
        }
    }
    pub fn top_elim(mut self) -> Self {
        if let Self::And { formulae } = &mut self {
            formulae.retain(|x| x != &Formula::top())
        }
        self
    }
    pub fn and_elim(mut self) -> Self {
        if let Self::And { formulae } = &mut self {
            if formulae.contains(&Formula::bottom()) {
                return Self::bottom();
            }
            formulae.sort();
            formulae.dedup();
            if formulae.len() == 1 {
                return formulae.pop().unwrap();
            }
        }
        self
    }
    pub fn not_not_elim(self) -> Self {
        match self {
            Self::Not { formula } => match *formula {
                Self::Not { formula } => *formula,
                formula => Self::Not { formula: Box::new(formula) },
            },
            x => x,
        }
    }
    pub fn var_elim(self, kb: &Kb) -> Self {
        match self {
            Self::Var { var } => match kb.test_var(var) {
                Some(true_false) => Self::constant(true_false),
                None => Self::Var { var },
            },
            x => x,
        }
    }
    pub fn replace(&mut self, func: &mut impl FnMut(Self) -> Self) {
        *self = func(std::mem::replace(self, Self::And { formulae: vec![] }))
    }
    pub fn depth_first_visit_mut(&mut self, func: &mut impl FnMut(&mut Formula)) {
        match self {
            Self::Var { .. } => {}
            Self::Not { formula } => formula.depth_first_visit_mut(func),
            Self::And { formulae } => {
                for formula in formulae {
                    formula.depth_first_visit_mut(func)
                }
            }
        }
        // print!("{:?} --> ", self);
        func(self);
        // println!(" {:?}", self);
    }
    pub fn simplify(mut self) -> Self {
        self.depth_first_visit_mut(&mut move |formula| {
            formula.replace(&mut move |f| f.not_not_elim().and_flatten().top_elim().and_elim())
        });
        self
    }
    pub fn simplify_with_kb(mut self, kb: &Kb) -> Self {
        self.depth_first_visit_mut(&mut move |formula| {
            formula.replace(&mut move |f| {
                f.var_elim(kb).not_not_elim().and_flatten().top_elim().and_elim()
            })
        });
        self
    }
}
