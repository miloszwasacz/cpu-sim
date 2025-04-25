pub use self::execution::ExecutionPage;

mod execution;

pub trait Page<R = ()> {
    //TODO Remove attribute when more pages are added
    #[allow(unused)]
    fn close(self) -> R;
}
