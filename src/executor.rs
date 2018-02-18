use futures_core::Future;
use futures_core::executor::{Executor, SpawnError};
use futures_executor::{ThreadPool, LocalExecutor};
use anchor_experiment::AnchoredBox;

pub trait StableExecutor: Executor {
    fn spawn_anchored(&mut self, f: AnchoredBox<Future<Item = (), Error = ()> + Send>) -> Result<(), SpawnError>;
}

impl StableExecutor for ThreadPool {
    fn spawn_anchored(&mut self, f: AnchoredBox<Future<Item = (), Error = ()> + Send>) -> Result<(), SpawnError> {
        unsafe { self.spawn(f.into_inner()) }
    }
}

impl StableExecutor for LocalExecutor {
    fn spawn_anchored(&mut self, f: AnchoredBox<Future<Item = (), Error = ()> + Send>) -> Result<(), SpawnError> {
        unsafe { self.spawn(f.into_inner()) }
    }
}
