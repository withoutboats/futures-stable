use anchor_experiment::PinMut;
use futures_core::{Future, Poll, task};

use PinnedFuture;

// This wrapper implements Future for any PinnedFuture. You must guarantee a
// stable address when constructing it.
pub(crate) struct UnsafeFuture<F: PinnedFuture> {
    future: F
}

impl<F: PinnedFuture> UnsafeFuture<F> {
    pub(crate) unsafe fn new(future: F) -> UnsafeFuture<F> {
        UnsafeFuture { future }
    }
}

impl<'a, F: PinnedFuture> Future for UnsafeFuture<F> {
    type Item = F::Item;
    type Error = F::Error;
    fn poll(&mut self, ctx: &mut task::Context) -> Poll<Self::Item, Self::Error> {
        F::poll(unsafe { PinMut::pinned_unchecked(&mut self.future) }, ctx)
    }
}
