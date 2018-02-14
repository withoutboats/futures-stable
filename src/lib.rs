#![feature(arbitrary_self_types)]

extern crate anchor_experiment;
extern crate futures_core;

mod unsafe_pin;

use anchor_experiment::{PinMut, AnchoredBox, Anchor};
use futures_core::{Future, Stream, Poll, task};

use unsafe_pin::UnsafePin;

pub trait PinnedFuture {
    type Item;
    type Error;

    fn poll(self: PinMut<Self>, ctx: &mut task::Context) -> Poll<Self::Item, Self::Error>;

    fn anchor<'a>(self) -> AnchoredBox<Future<Item = Self::Item, Error = Self::Error> + 'a>
        where Self: Sized + 'a
    {
        Anchor::new(Box::new(unsafe { UnsafePin::new(self) }))
    }
}

pub trait PinnedStream {
    type Item;
    type Error;

    fn poll(self: PinMut<Self>, ctx: &mut task::Context) -> Poll<Option<Self::Item>, Self::Error>;

    fn anchor<'a>(self) -> AnchoredBox<Stream<Item = Self::Item, Error = Self::Error> + 'a>
        where Self: Sized + 'a
    {
        Anchor::new(Box::new(unsafe { UnsafePin::new(self) }))
    }
}
