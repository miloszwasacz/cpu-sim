use super::{cond_branch, impl_branch_execute};

cond_branch!(Blt);

impl_branch_execute!(Blt, lt);
