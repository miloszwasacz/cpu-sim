pub use self::execution::ExecutionPage;

mod execution;

pub trait Page<R = ()> {
    fn close(self) -> R;
}
