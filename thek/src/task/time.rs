//! Time handling.

use core::sync::atomic::{
    AtomicU32, Ordering
};
use crate::cpu::arch::TIMER_FREQ_HZ;

/// Timer period in seconds.
pub const TIMER_PERIOD_SEC: f64 = 1.0 / TIMER_FREQ_HZ;

/// Sleeps for some time in milliseconds.
pub fn sleep(delay_ms: usize) {
    let delay_secs = delay_ms as f64 / 1000.0;
    let initial_ticks =  TICKS.load(Ordering::SeqCst);
    let initial_secs = initial_ticks as f64 * TIMER_PERIOD_SEC;
    loop {
        let current_ticks = TICKS.load(Ordering::SeqCst); 
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

/// Increment tick counter.
pub(super) fn tick() {
    TICKS.fetch_add(1, Ordering::SeqCst);
}

// We use u32 instead of usize because the conversion to float must fit in a f64.
static TICKS: AtomicU32 = AtomicU32::new(0);
