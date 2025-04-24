pub(super) trait WritePolicy {}

pub struct WriteThrough;
impl WritePolicy for WriteThrough {}

pub struct WriteBack;
impl WritePolicy for WriteBack {}
