use anchor_experiment::{PinMut, AnchoredBox};

use futures_core::{Future, Stream, Poll, task};

pub struct AnchoredFuture<'a, T, E> {
    pub(crate) inner: AnchoredBox<Future<Item = T, Error = E> + 'a>,
}

impl<'a, T, E> Future for AnchoredFuture<'a, T, E> {
    type Item = T;
    type Error = E;
    fn poll(&mut self, ctx: &mut task::Context) -> Poll<Self::Item, Self::Error> {
        let mut pin = self.inner.as_pin_mut();
        let this = unsafe { PinMut::get_mut(&mut pin) };
        this.poll(ctx)
    }
}

pub struct AnchoredFutureSend<'a, T, E> {
    pub(crate) inner: AnchoredBox<Future<Item = T, Error = E> + Send + 'a>,
}

impl<'a, T, E> Future for AnchoredFutureSend<'a, T, E> {
    type Item = T;
    type Error = E;
    fn poll(&mut self, ctx: &mut task::Context) -> Poll<Self::Item, Self::Error> {
        let mut pin = self.inner.as_pin_mut();
        let this = unsafe { PinMut::get_mut(&mut pin) };
        this.poll(ctx)
    }
}

pub struct AnchoredStream<'a, T, E> {
    pub(crate) inner: AnchoredBox<Stream<Item = T, Error = E> + 'a>,
}

impl<'a, T, E> Stream for AnchoredStream<'a, T, E> {
    type Item = T;
    type Error = E;
    fn poll(&mut self, ctx: &mut task::Context) -> Poll<Option<Self::Item>, Self::Error> {
        let mut pin = self.inner.as_pin_mut();
        let this = unsafe { PinMut::get_mut(&mut pin) };
        this.poll(ctx)
    }
}

pub struct AnchoredStreamSend<'a, T, E> {
    pub(crate) inner: AnchoredBox<Stream<Item = T, Error = E> + Send + 'a>,
}

impl<'a, T, E> Stream for AnchoredStreamSend<'a, T, E> {
    type Item = T;
    type Error = E;
    fn poll(&mut self, ctx: &mut task::Context) -> Poll<Option<Self::Item>, Self::Error> {
        let mut pin = self.inner.as_pin_mut();
        let this = unsafe { PinMut::get_mut(&mut pin) };
        this.poll(ctx)
    }
}
