use futures_core::{Future, Never};
use futures_core::executor::{Executor, SpawnError};
use futures_executor::{ThreadPool, LocalExecutor};
use anchor_experiment::PinBox;

pub trait StableExecutor: Executor {
    fn spawn_anchored(&mut self, f: PinBox<Future<Item = (), Error = Never> + Send>) -> Result<(), SpawnError>;
}

impl StableExecutor for ThreadPool {
    fn spawn_anchored(&mut self, f: PinBox<Future<Item = (), Error = Never> + Send>) -> Result<(), SpawnError> {
        unsafe { self.spawn(f.into_box_unchecked()) }
    }
}

impl StableExecutor for LocalExecutor {
    fn spawn_anchored(&mut self, f: PinBox<Future<Item = (), Error = Never> + Send>) -> Result<(), SpawnError> {
        unsafe { self.spawn(f.into_box_unchecked()) }
    }
}
