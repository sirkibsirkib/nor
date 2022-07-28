mod formula;

use crate::kb::Kb;
use crate::var::VAR as V;
use crate::var_set::VarSet;
use formula::Formula;

pub fn main() {
    let _kb = Kb {
        vars_true: VarSet::from_iter([V[0]]), // true
        vars_fals: VarSet::from_iter([V[0]]), // false
    };
    // let mut formula = Formula::var(V[0]).and(Formula::var(V[0]).yimpl(Formula::var(V[1])));

    // A & !(!(A & !B) & !B)

    let mut formula = Formula::var(V[0])
        .yimpl(Formula::var(V[1]))
        .and(Formula::var(V[0]))
        .and(Formula::var(V[1]).not());
    println!("{:#?}\n ==>", formula);
    // formula = formula.var_elim_with(&kb);
    // println!("{:#?}\n ==>", formula);
    formula = formula.simplify();
    println!("{:#?}\n", formula);
}
