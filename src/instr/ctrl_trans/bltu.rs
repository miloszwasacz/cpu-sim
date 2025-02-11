use super::{cond_branch, impl_branch_execute};

cond_branch!(Bltu);

impl_branch_execute!(Bltu, lt, unsigned);
