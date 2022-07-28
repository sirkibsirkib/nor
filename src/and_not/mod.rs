mod formula;

use crate::kb::Kb;
use crate::var::VAR as V;
use crate::var_set::VarSet;
use formula::Formula;

pub fn main() {
    let kb = Kb {
        vars_true: VarSet::from_iter([V[0]]), // true
        vars_fals: VarSet::from_iter([V[0]]), // false
    };
    let formula = Formula::var(V[0]).and(Formula::var(V[1])).and(Formula::var(V[1]));
    println!("{:#?}\n ==>", formula);
    println!("{:#?}", formula.simplify_with_kb(&kb));
}
