use crate::kb::Kb;
use crate::nary::formula::Formula;
use crate::var::{Var, VAR as V};
use crate::var_set::VarSet;
use crate::vec_mut_q::VecMutQ;
use std::fmt;

mod formula;

//////////////////////////

pub fn main() {
    let kb = Kb {
        vars_true: VarSet::from_iter([V[0]]), // true
        vars_fals: VarSet::from_iter([V[1]]), // false
    };

    for form in [
        // wah
        Formula::Var { var: V[0] },
        Formula::top(),
        Formula::bottom(),
        Formula::bottom().not(),
        Formula::Var { var: V[0] }.not().not(),
        Formula::Var { var: V[1] }.not().not().nor(Formula::top()).not(),
        Formula::Var { var: V[2] }.nor(Formula::Var { var: V[2] }),
    ] {
        println!(
            "form: {:?} => {:?} ## {:?}",
            form,
            form.clone().normify(),
            kb.simplify_formula(form.clone())
        );
    }
}
