use super::{cond_branch, impl_branch_execute};

cond_branch!(Bne);

impl_branch_execute!(Bne, ne);
