use crate::kb::Kb;
use crate::var::Var;
use crate::vec_mut_q::VecMutQ;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Formula {
    Var { var: Var },
    Nor { formulae: Box<[Formula; 2]> },
}

impl Formula {}
