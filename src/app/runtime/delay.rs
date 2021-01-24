use std::future::Future;
use std::time::Duration;

use futures_timer::Delay;

pub fn delay(duration: Duration) -> impl Future {
    Delay::new(duration)
}
