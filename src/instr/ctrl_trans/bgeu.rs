use super::{cond_branch, impl_branch_execute};

cond_branch!(Bgeu);

impl_branch_execute!(Bgeu, ge, unsigned);
