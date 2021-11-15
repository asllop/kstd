//! Time handling.

use core::sync::atomic::{
    AtomicU32, Ordering
};
use crate::sys::KMutex;
use super::arch::{
    set_timer_handler, TIMER_FREQ_HZ
};

/// Init timer handlers.
pub fn init_time() {
    set_timer_handler(internal_timer_handler);
}

/// Register a function to be executed on each timer interrupt.
/// 
/// Handlers can only be registered but not unregistered and there is a maximum of 4, and they are executed sequentially starting from index 0.
/// Indexes 0 and 1 are reserved for the internal kernel behavior, so don't use them unless you know what you are doing.
/// 
/// *WARNINGS*:
/// - Never use mutex that can be used somewhere else within the timer handler, or you will likely cause a deadlock.
/// - The system will remain locked until all handler functions return.
pub fn register_handler(func: fn(), index: usize) -> bool {
    if index >= MAX_HANDLERS {
        return false;
    }
    let ints_where_enabled = super::check_ints();
    super::disable_ints();
    // The internal_timer_handler also locks this mutex, so we need ints disabled to avoid a deadlock.
    let mut time_handlers = TIMER_HANDLERS.acquire();
    if ints_where_enabled {
        super::enable_ints();
    }
    time_handlers[index] = func;
    true
}

const MAX_HANDLERS: usize = 4;
static TIMER_HANDLERS: KMutex<[fn(); MAX_HANDLERS]> = KMutex::new([|| {}; MAX_HANDLERS]);

/// Timer period in seconds.
pub const TIMER_PERIOD_SEC: f64 = 1.0 / TIMER_FREQ_HZ;

fn internal_timer_handler() {
    let time_handlers = TIMER_HANDLERS.acquire();
    for func in *time_handlers {
        func();
    }
    TICKS.fetch_add(1, Ordering::SeqCst);
}

/// Sleeps for some time in milliseconds.
pub fn sleep(delay_ms: usize) {
    let delay_secs = delay_ms as f64 / 1000.0;
    let initial_ticks =  TICKS.load(Ordering::Relaxed);
    let initial_secs = initial_ticks as f64 * TIMER_PERIOD_SEC;
    loop {
        let current_ticks = TICKS.load(Ordering::Relaxed); 
        if current_ticks >= initial_ticks {
            let current_secs = current_ticks as f64 * TIMER_PERIOD_SEC;
            if current_secs - initial_secs >= delay_secs {
                return;
            }
        }
        else {
            // Ticks counter overfloaded and restarted from 0
            let distance_to_overflow = u32::MAX - initial_ticks;
            let new_current_ticks = current_ticks + distance_to_overflow;
            let new_initial_secs = 0.0;
            let new_current_secs = new_current_ticks as f64 * TIMER_PERIOD_SEC;
            if new_current_secs - new_initial_secs >= delay_secs {
                return;
            }
        }
        //TODO: good place to yield the CPU
    }
}

// We use u32 instead of usize because the conversion to float must fit a f64.
static TICKS: AtomicU32 = AtomicU32::new(0);
