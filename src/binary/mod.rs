// use std::fmt;

mod formula;

/////////////////////////

// impl fmt::Debug for Formula {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::Var { var } => var.fmt(f),
//             Self::Nor { ref formulae } => {
//                 f.write_fmt(format_args!("[{:?}{:?}]", formulae[0], formulae[1]))
//             }
//         }
//     }
// }
// fn main() {}
