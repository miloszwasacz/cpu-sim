use super::{cond_branch, impl_branch_execute};

cond_branch!(Bge);

impl_branch_execute!(Bge, ge);
