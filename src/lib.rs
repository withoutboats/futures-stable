#![feature(arbitrary_self_types)]

extern crate anchor_experiment;
extern crate futures_core;

mod anchored_newtypes;
mod unsafe_pin;

use anchor_experiment::{PinMut, Anchor};
use futures_core::{Poll, task};

use anchored_newtypes::*;
use unsafe_pin::UnsafePin;

pub trait PinnedFuture {
    type Item;
    type Error;

    fn poll(self: PinMut<Self>, ctx: &mut task::Context) -> Poll<Self::Item, Self::Error>;

    fn anchor<'a>(self) -> AnchoredFuture<'a, Self::Item, Self::Error>
        where Self: Sized + 'a
    {
        AnchoredFuture { inner: Anchor::new(Box::new(unsafe { UnsafePin::new(self) })) }
    }

    fn anchor_send<'a>(self) -> AnchoredFutureSend<'a, Self::Item, Self::Error>
        where Self: Send + Sized + 'a
    {
        AnchoredFutureSend { inner: Anchor::new(Box::new(unsafe { UnsafePin::new(self) })) }
    }
}

pub trait PinnedStream {
    type Item;
    type Error;

    fn poll(self: PinMut<Self>, ctx: &mut task::Context) -> Poll<Option<Self::Item>, Self::Error>;

    fn anchor<'a>(self) -> AnchoredStream<'a, Self::Item, Self::Error>
        where Self: Sized + 'a
    {
        AnchoredStream { inner: Anchor::new(Box::new(unsafe { UnsafePin::new(self) })) }
    }

    fn anchor_send<'a>(self) -> AnchoredStreamSend<'a, Self::Item, Self::Error>
        where Self: Send + Sized + 'a
    {
        AnchoredStreamSend { inner: Anchor::new(Box::new(unsafe { UnsafePin::new(self) })) }
    }
}
