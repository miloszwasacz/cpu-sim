use super::{cond_branch, impl_branch_execute};

cond_branch!(Beq);

impl_branch_execute!(Beq, eq);
