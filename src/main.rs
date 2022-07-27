use std::fmt;

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
struct Var {
    index: u8, // only using 6 bits
}

struct MutIterVec<'a, T> {
    vec: &'a mut Vec<T>,
    processed_before: usize,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Formula {
    Var { var: Var },
    Nor { formulae: Vec<Formula> },
}

impl fmt::Debug for Formula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Var { var } => var.fmt(f),
            Self::Nor { formulae } => f.debug_list().entries(formulae).finish(),
        }
    }
}

#[derive(Default, Copy, Clone, Eq, PartialEq)]
struct VarSet {
    bit_set: u64,
}
struct VarSetIter {
    remaining: VarSet,
}

#[derive(Debug, Default)]
struct Kb {
    vars_true: VarSet,
    vars_fals: VarSet,
}

//////////////////////////

impl<'a, T> MutIterVec<'a, T> {
    fn in_place_endo_map(vec: &'a mut Vec<T>, mut func: impl FnMut(T) -> T) {
        let mut me = Self::new(vec);
        while let Some(x) = me.take_unprocessed() {
            me.add_processed(func(x));
        }
    }
    fn new(vec: &'a mut Vec<T>) -> Self {
        Self { vec, processed_before: 0 }
    }
    fn take_unprocessed(&mut self) -> Option<T> {
        if self.processed_before < self.vec.len() {
            Some(self.vec.swap_remove(self.processed_before))
        } else {
            None
        }
    }
    fn add_unprocessed(&mut self, t: T) {
        self.vec.push(t)
    }
    fn add_processed(&mut self, t: T) {
        self.vec.push(t);
        self.processed_before += 1;
        let len = self.vec.len();
        self.vec.swap(len - 1, self.processed_before - 1);
    }
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
impl fmt::Debug for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("v{}", self.index))
    }
}
impl Iterator for VarSetIter {
    type Item = Var;
    fn next(&mut self) -> Option<Var> {
        self.remaining.take()
    }
}
impl fmt::Debug for VarSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.into_iter()).finish()
    }
}

impl VarSet {
    fn singleton(var: Var) -> Self {
        Self { bit_set: 1 << var.index }
    }
    fn into_iter(self) -> VarSetIter {
        VarSetIter { remaining: self }
    }
    fn take(&mut self) -> Option<Var> {
        let trailing = self.bit_set.trailing_zeros() as u8;
        if trailing < 64 {
            let var = Var::new(trailing)?;
            self.remove(var);
            Some(var)
        } else {
            None
        }
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
    fn top() -> Self {
        Self::Nor { formulae: vec![] }
    }
    fn bottom() -> Self {
        Self::top().not()
    }
    fn not(self) -> Self {
        Self::Nor { formulae: vec![self] }
    }
    fn var(var: Var) -> Self {
        Self::Var { var }
    }
    fn n_nor(formulae: Vec<Self>) -> Self {
        Self::Nor { formulae }
    }
    fn n_or(formulae: Vec<Self>) -> Self {
        Self::n_nor(formulae).not()
    }
    fn n_and(formulae: Vec<Self>) -> Self {
        Self::n_nor(formulae.into_iter().map(Self::not).collect())
    }
    fn n_nand(formulae: Vec<Self>) -> Self {
        Self::n_and(formulae).not()
    }
    fn and(self, other: Self) -> Self {
        Self::n_and(vec![self, other])
    }
    fn nor(self, other: Self) -> Self {
        Self::n_nor(vec![self, other])
    }
    fn or(self, other: Self) -> Self {
        Self::n_or(vec![self, other])
    }
    fn nimpl(self, other: Self) -> Self {
        self.not().nor(other)
    }
    fn yimpl(self, other: Self) -> Self {
        self.nimpl(other).not()
    }
    fn normify(self) -> Self {
        // match
        match self {
            x @ Self::Var { .. } => x,
            Self::Nor { mut formulae } => {
                let mut miv = MutIterVec::new(&mut formulae);
                while let Some(f) = miv.take_unprocessed() {
                    match f.normify().not_nor_to_or() {
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
    fn not_not_elim(mut self) -> Self {
        if let Some(only) = self.match_not().and_then(Self::match_not) {
            let mut dummy = Self::Var { var: VAR[0] };
            std::mem::swap(&mut dummy, only);
            dummy
        } else {
            self
        }
    }
    // fn normal(mut self) -> Self {
    //     // TODO SOMETHING IS WRONG
    //     self = self.not_not_elim();
    //     match self {
    //         x @ Self::Var { .. } => x,
    //         Self::Nor { formulae } => {
    //             let mut formulae: Vec<Self> = formulae.into_iter().map(Self::normal).collect();
    //             formulae.sort();
    //             formulae.dedup();
    //             Self::Nor { formulae }
    //         }
    //     }
    // }
    fn match_not(&mut self) -> Option<&mut Self> {
        if let Self::Nor { formulae } = self {
            if let [only] = &mut formulae[..] {
                return Some(only);
            }
        }
        None
    }
    fn not_nor_to_or(self) -> Result<Vec<Self>, Self> {
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
    fn test_var(&self, var: Var) -> Option<bool> {
        if self.vars_true.contains(var) {
            Some(true)
        } else if self.vars_fals.contains(var) {
            Some(false)
        } else {
            None
        }
    }
    fn test_formula_inner(&self, formula: &Formula) -> Option<bool> {
        match formula {
            Formula::Var { var } => self.test_var(*var),
            Formula::Nor { formulae } => {
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
    fn test_formula(&self, formula: &Formula) -> Option<bool> {
        let ret = self.test_formula_inner(formula);
        println!("T({:?}) = {:?}", formula, ret);
        ret
    }
    fn simplify_formula(&self, formula: Formula) -> Formula {
        match formula {
            Formula::Var { var } => match self.test_var(var) {
                Some(true) => Formula::top(),
                Some(false) => Formula::bottom(),
                None => Formula::Var { var },
            },
            Formula::Nor { mut formulae } => {
                MutIterVec::in_place_endo_map(&mut formulae, |f| self.simplify_formula(f));
                Formula::Nor { formulae }
            }
        }
        .normify()
    }
    // fn make_true(&self, formula: &Formula) -> Formula {
    //     match formula {
    //         Formula::Var { var } => match self.test_var(*var) {
    //             Some(true) => Formula::top(),
    //             Some(false) => Formula::bottom(),
    //             None => formula.clone(),
    //         },
    //         Formula::Nor { formulae } => {
    //             let mut new = Vec::with_capacity(formulae.len());
    //             for f in formulae {
    //                 match self.test_formula(f) {
    //                     Some(false) => {}
    //                     Some(true) => return Formula::bottom(),
    //                     None => new.push(f.clone().not()),
    //                 }
    //             }
    //             return Formula::Nor { formulae: new }.not().normal();
    //         }
    //     }
    // }
}

fn main() {
    use VAR as V;
    let kb = Kb {
        vars_true: VarSet::from_iter([V[0]]), // true
        vars_fals: VarSet::from_iter([V[1]]), // false
    };
    // println!("{:?}", kb);
    // let query =
    //     Formula::var(V[0]).and(Formula::var(V[3])).not().not().yimpl(Formula::top()).normal();
    // println!("{:?}", kb.test_formula(&query));

    for form in [
        // wah
        Formula::Var { var: V[0] },
        Formula::top(),
        Formula::bottom(),
        Formula::bottom().not(),
        Formula::Var { var: V[0] }.not().not(),
        Formula::Var { var: V[1] }.not().not().nor(Formula::top()).not(),
        Formula::Var { var: V[2] }.nor(Formula::top()),
    ] {
        println!(
            "form: {:?} => {:?} ## {:?}",
            form,
            form.clone().normify(),
            kb.simplify_formula(form.clone())
        );
    }
}
