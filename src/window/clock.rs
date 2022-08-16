use std::time::{Instant, Duration};


const TICK_MICROSECONDS: u64 = 16667;  // 60 FPS

pub(crate) struct Clock {
    tick: u64,
    last_received_tick: u64,
    last_tick_at: Option<Instant>
}

impl Clock {
    pub fn new() -> Self {
        Clock { 
            tick: 0,
            last_received_tick: 0,
            last_tick_at: None,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let (is_new_tick, new_tick_time) = if let Some(lfa) = self.last_tick_at {
            let ntt_1 = lfa + Duration::from_micros(TICK_MICROSECONDS);
            let ntt_2 = lfa + Duration::from_micros(TICK_MICROSECONDS * 2);
            if ntt_2 < now {
                (true, now)  // don't use the ordinary tick timing, we've dropped a frame
            }  else if ntt_1 < now{
                (true, ntt_1)  // pretend the tick happened at its scheduled time
            } else {
                (false, now)
            }
        } else {
            (true, now)
        };

        if is_new_tick {
            // never generate more than one tick without giving other code an opportunity
            self.tick = self.last_received_tick + 1;
            self.last_tick_at = Some(new_tick_time);
        }
    }

    pub(crate) fn pop_event(&mut self) -> Option<u64> {
        if self.last_received_tick == self.tick {
            return None
        } else {
            self.last_received_tick += 1;
            return Some(self.last_received_tick)
        }
    }

    pub(crate) fn peek_event(&self) -> Option<u64> {
        if self.last_received_tick == self.tick {
            return None
        } else {
            return Some(self.last_received_tick + 1)
        }
    }
}