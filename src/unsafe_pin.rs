use anchor_experiment::PinMut;
use futures_core::{Future, Stream, Poll, task};

use {PinnedFuture, PinnedStream};

pub(crate) struct UnsafePin<T> {
    inner: T,
}

impl<T> UnsafePin<T> {
    pub(crate) unsafe fn new(inner: T) -> UnsafePin<T> {
        UnsafePin { inner }
    }
}

impl<'a, T: PinnedFuture> Future for UnsafePin<T> {
    type Item = T::Item;
    type Error = T::Error;
    fn poll(&mut self, ctx: &mut task::Context) -> Poll<Self::Item, Self::Error> {
        T::poll(unsafe { PinMut::pinned_unchecked(&mut self.inner) }, ctx)
    }
}

impl<'a, T: PinnedStream> Stream for UnsafePin<T> {
    type Item = T::Item;
    type Error = T::Error;
    fn poll(&mut self, ctx: &mut task::Context) -> Poll<Option<Self::Item>, Self::Error> {
        T::poll(unsafe { PinMut::pinned_unchecked(&mut self.inner) }, ctx)
    }
}
