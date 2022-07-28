use crate::var::Var;
use crate::var_set::VarSet;

#[derive(Debug, Default)]
pub struct Kb {
    pub vars_true: VarSet,
    pub vars_fals: VarSet,
}

impl Kb {
    pub fn test_var(&self, var: Var) -> Option<bool> {
        if self.vars_true.contains(var) {
            Some(true)
        } else if self.vars_fals.contains(var) {
            Some(false)
        } else {
            None
        }
    }
}
