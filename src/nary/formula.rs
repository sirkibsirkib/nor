use std::fmt;

use crate::nary::Kb;
use crate::var::Var;
use crate::vec_mut_q::VecMutQ;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Formula {
    Var { var: Var },
    Nor { formulae: Vec<Formula> },
}

impl fmt::Debug for Formula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Var { var } => var.fmt(f),
            Self::Nor { formulae } => {
                f.debug_list().entries(formulae).finish()
                // f.write_fmt(format_args!("["))?;
                // for formula in formulae {
                //     formula.fmt(f)?;
                // }
                // f.write_fmt(format_args!("]"))
            }
        }
    }
}

impl Formula {
    pub fn top() -> Self {
        Self::Nor { formulae: vec![] }
    }
    pub fn bottom() -> Self {
        Self::top().not()
    }
    pub fn not(self) -> Self {
        Self::Nor { formulae: vec![self] }
    }
    pub fn var(var: Var) -> Self {
        Self::Var { var }
    }
    pub fn n_nor(formulae: Vec<Self>) -> Self {
        Self::Nor { formulae }
    }
    pub fn n_or(formulae: Vec<Self>) -> Self {
        Self::n_nor(formulae).not()
    }
    pub fn n_and(formulae: Vec<Self>) -> Self {
        Self::n_nor(formulae.into_iter().map(Self::not).collect())
    }
    pub fn n_nand(formulae: Vec<Self>) -> Self {
        Self::n_and(formulae).not()
    }
    pub fn and(self, other: Self) -> Self {
        Self::n_and(vec![self, other])
    }
    pub fn nor(self, other: Self) -> Self {
        Self::n_nor(vec![self, other])
    }
    pub fn or(self, other: Self) -> Self {
        Self::n_or(vec![self, other])
    }
    pub fn nimpl(self, other: Self) -> Self {
        self.not().nor(other)
    }
    pub fn yimpl(self, other: Self) -> Self {
        self.nimpl(other).not()
    }
    pub fn normify(self) -> Self {
        match self {
            x @ Self::Var { .. } => x,
            Self::Nor { mut formulae } => {
                let mut miv = VecMutQ::new(&mut formulae);
                while let Some(f) = miv.take_unprocessed() {
                    match f.normify().nor_not_to_or() {
                        Ok(f2) => {
                            for f3 in f2 {
                                miv.add_unprocessed(f3)
                            }
                        }
                        Err(f2) => miv.add_processed(f2),
                    }
                }
                formulae.sort();
                formulae.dedup();
                if formulae.contains(&Formula::top()) {
                    return Formula::bottom();
                }
                formulae.retain(|x| x != &Formula::bottom());
                Self::Nor { formulae }
            }
        }
    }
    pub fn nor_not_to_or(self) -> Result<Vec<Self>, Self> {
        match self {
            x @ Self::Var { .. } => Err(x),
            Self::Nor { formulae: mut f1 } => {
                // [f1]
                if let [Self::Nor { formulae: f2 }] = &mut f1[..] {
                    // [[f2]]
                    Ok(std::mem::replace(f2, vec![]))
                } else {
                    Err(Self::Nor { formulae: f1 })
                }
            }
        }
    }
}

impl Kb {
    pub fn test_formula(&self, formula: Formula) -> Option<bool> {
        let x = self.simplify_formula(formula);
        if x == Formula::top() {
            Some(true)
        } else if x == Formula::bottom() {
            Some(false)
        } else {
            None
        }
    }
    pub fn simplify_formula(&self, formula: Formula) -> Formula {
        match formula {
            Formula::Var { var } => match self.test_var(var) {
                Some(true) => Formula::top(),
                Some(false) => Formula::bottom(),
                None => Formula::Var { var },
            },
            Formula::Nor { mut formulae } => {
                VecMutQ::in_place_endo_map(&mut formulae, |f| self.simplify_formula(f));
                Formula::Nor { formulae }
            }
        }
        .normify()
    }
}
