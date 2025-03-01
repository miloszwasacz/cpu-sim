pub trait Writeback {
    fn reg_write(&self) -> bool;
}
