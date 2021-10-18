use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
    fmt::write
};

use crate::console::*;

pub struct CounterFuture {
    shared_state: CounterSharedState
}

/// Shared state between the future and the waiting thread
struct CounterSharedState {
    counter: u32,
    final_value: u32,
    waker: Option<Waker>,
}

impl Future for CounterFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        //TODO: mutex get
        let _self = self.get_mut();
        if _self.shared_state.counter >= _self.shared_state.final_value {
            Poll::Ready(())
        } else {
            print!("Counting to {}, current {}", _self.shared_state.final_value, _self.shared_state.counter);
            _self.shared_state.counter += 1;
            _self.shared_state.waker = Some(cx.waker().clone());
            
            if let Some(waker) = _self.shared_state.waker.take() {
                waker.wake()
            }
            Poll::Pending
        }
    }
}

impl CounterFuture {
    pub fn new(final_value: u32) -> Self {
        let shared_state = CounterSharedState {
            counter: 0,
            final_value,
            waker: None,
        };

        CounterFuture { shared_state }
    }
}
