use std::time::{Duration, Instant};

use super::super::runtime;

pub async fn delay(duration: Duration) {
    let start = Instant::now();
    loop {
        let now = Instant::now();
        let duration_from_start = now.duration_since(start);
        if duration_from_start > duration {
            break;
        }
        runtime::next_frame().await;
    }
}
