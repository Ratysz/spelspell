use specs::prelude::*;
use std::cmp::min;
use std::collections::VecDeque;
use std::sync::Arc;
pub use std::time::Duration;

const ZERO_DURATION: Duration = Duration::from_secs(0);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Instant(Duration);

impl Instant {
    pub fn compare_to(&self, other: Instant) -> DirectedTime {
        if self.0 < other.0 {
            DirectedTime::Future(other.0 - self.0)
        } else if self.0 == other.0 {
            DirectedTime::Still
        } else {
            DirectedTime::Past(self.0 - other.0)
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum DirectedTime {
    Future(Duration),
    Still,
    Past(Duration),
}

pub struct Timekeeper {
    real_time_delta: Duration,
    remaining_sim_time: Duration,
    sim_delta: DirectedTime,
    sim_time_factor: f32,
    sim_elapsed_time: Duration,
    sim_schedule: VecDeque<(Duration, VecDeque<Arc<Schedulable + Send + Sync>>)>,
}

trait Schedulable {}

impl Default for Timekeeper {
    fn default() -> Self {
        Self::new()
    }
}

impl Timekeeper {
    pub fn new() -> Timekeeper {
        Timekeeper {
            real_time_delta: ZERO_DURATION,
            remaining_sim_time: ZERO_DURATION,
            sim_delta: DirectedTime::Still,
            sim_time_factor: 1.0,
            sim_elapsed_time: ZERO_DURATION,
            sim_schedule: VecDeque::new(),
        }
    }

    pub fn update_real_time(&mut self, d_time: Duration) {
        self.real_time_delta = d_time;
        if self.remaining_sim_time > ZERO_DURATION {
            let adjusted = mul_dur_by_factor(self.real_time_delta, self.sim_time_factor.abs());
            let time_chunk = min(adjusted, self.remaining_sim_time);
            self.remaining_sim_time -= time_chunk;
            match self.sim_time_factor.signum() {
                1.0 => {
                    self.sim_elapsed_time += time_chunk;
                    self.sim_delta = DirectedTime::Future(time_chunk);
                }
                -1.0 => {
                    self.sim_elapsed_time -= time_chunk;
                    self.sim_delta = DirectedTime::Past(time_chunk);
                }
                _ => unreachable!(
                    "Time factor signum should never be anything other than `1.0` or `-1.0`"
                ),
            }
        } else {
            self.sim_delta = DirectedTime::Still;
        }
    }

    pub fn add_simulation_time(&mut self, d_time: Duration) {
        self.remaining_sim_time += d_time;
    }

    pub fn real_time_delta(&self) -> Duration {
        self.real_time_delta
    }

    pub fn delta(&self) -> DirectedTime {
        self.sim_delta
    }

    pub fn set_time_factor(&mut self, factor: f32) {
        self.sim_time_factor = factor;
    }

    pub fn time_factor(&self) -> f32 {
        self.sim_time_factor
    }

    pub fn now(&self) -> Instant {
        Instant(self.sim_elapsed_time)
    }

    pub fn schedule<T: Schedulable>(&mut self, time_from_now: Duration, event: T) -> Instant {
        Instant(self.sim_elapsed_time + time_from_now)
    }
}

fn mul_dur_by_factor<T: Copy + Into<f64>>(duration: Duration, factor: T) -> Duration {
    let adjusted_s: f64 = duration.as_secs() as f64 * factor.into();
    let mut adjusted_n: f64 = duration.subsec_nanos() as f64 * factor.into();
    let trunc_s = adjusted_s.trunc();
    let subsec = adjusted_s - trunc_s;
    let mut adjusted_s = trunc_s as u64;
    adjusted_n += subsec * 1_000_000_000.0;
    let rem_n = (adjusted_n % 1_000_000_000.0);
    adjusted_s += (adjusted_n / 1_000_000_000.0).trunc() as u64;
    let adjusted_n = rem_n as u32;
    Duration::new(adjusted_s, adjusted_n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sim_delta() {
        let mut timekeeper = Timekeeper::new();
        assert_eq!(timekeeper.delta(), DirectedTime::Still);
        timekeeper.update_real_time(Duration::from_secs(2));
        assert_eq!(timekeeper.delta(), DirectedTime::Still);
        timekeeper.add_simulation_time(Duration::from_secs(8));
        timekeeper.update_real_time(Duration::from_secs(2));
        assert_eq!(
            timekeeper.delta(),
            DirectedTime::Future(Duration::from_secs(2))
        );
        timekeeper.update_real_time(Duration::from_secs(2));
        assert_eq!(
            timekeeper.delta(),
            DirectedTime::Future(Duration::from_secs(2))
        );
        timekeeper.update_real_time(Duration::from_secs(3));
        assert_eq!(
            timekeeper.delta(),
            DirectedTime::Future(Duration::from_secs(3))
        );
        timekeeper.update_real_time(Duration::from_secs(2));
        assert_eq!(
            timekeeper.delta(),
            DirectedTime::Future(Duration::from_secs(1))
        );
        timekeeper.update_real_time(Duration::from_secs(1));
        assert_eq!(timekeeper.delta(), DirectedTime::Still);
    }

    #[test]
    fn sim_now() {
        let mut timekeeper = Timekeeper::new();
        let start = timekeeper.now();
        timekeeper.update_real_time(Duration::from_secs(1));
        assert_eq!(timekeeper.now(), start);
        timekeeper.add_simulation_time(Duration::from_secs(8));
        assert_eq!(timekeeper.now(), start);
        timekeeper.update_real_time(Duration::from_secs(1));
        assert_ne!(timekeeper.now(), start);
        assert_eq!(
            timekeeper.now().compare_to(start),
            DirectedTime::Past(Duration::from_secs(1))
        );
        timekeeper.update_real_time(Duration::from_secs(1));
        assert_eq!(
            timekeeper.now().compare_to(start),
            DirectedTime::Past(Duration::from_secs(2))
        );
    }

    #[test]
    fn sim_now_backwards() {
        let mut timekeeper = Timekeeper::new();
        timekeeper.add_simulation_time(Duration::from_secs(8));
        timekeeper.update_real_time(Duration::from_secs(8));
        let start = timekeeper.now();
        timekeeper.set_time_factor(-1.0);
        timekeeper.update_real_time(Duration::from_secs(1));
        assert_eq!(timekeeper.now(), start);
        timekeeper.add_simulation_time(Duration::from_secs(8));
        assert_eq!(timekeeper.now(), start);
        timekeeper.update_real_time(Duration::from_secs(1));
        assert_ne!(timekeeper.now(), start);
        assert_eq!(
            timekeeper.now().compare_to(start),
            DirectedTime::Future(Duration::from_secs(1))
        );
        timekeeper.update_real_time(Duration::from_secs(1));
        assert_eq!(
            timekeeper.now().compare_to(start),
            DirectedTime::Future(Duration::from_secs(2))
        );
    }

    #[test]
    fn time_factor() {
        let mut timekeeper = Timekeeper::new();
        timekeeper.add_simulation_time(Duration::from_secs(8));
        timekeeper.update_real_time(Duration::from_secs(2));
        assert_eq!(
            timekeeper.delta(),
            DirectedTime::Future(Duration::from_secs(2))
        );
        timekeeper.set_time_factor(2.0);
        assert_eq!(
            timekeeper.delta(),
            DirectedTime::Future(Duration::from_secs(2))
        );
        timekeeper.update_real_time(Duration::from_secs(2));
        assert_eq!(
            timekeeper.delta(),
            DirectedTime::Future(Duration::from_secs(4))
        );
        timekeeper.update_real_time(Duration::from_secs(2));
        assert_eq!(
            timekeeper.delta(),
            DirectedTime::Future(Duration::from_secs(2))
        );
        timekeeper.update_real_time(Duration::from_secs(2));
        assert_eq!(timekeeper.delta(), DirectedTime::Still);
        timekeeper.set_time_factor(0.5);
        timekeeper.add_simulation_time(Duration::from_secs(8));
        assert_eq!(timekeeper.delta(), DirectedTime::Still);
        timekeeper.update_real_time(Duration::from_secs(2));
        assert_eq!(
            timekeeper.delta(),
            DirectedTime::Future(Duration::from_secs(1))
        );
        timekeeper.update_real_time(Duration::from_secs(4));
        assert_eq!(
            timekeeper.delta(),
            DirectedTime::Future(Duration::from_secs(2))
        );
        timekeeper.set_time_factor(1.0);
        timekeeper.update_real_time(Duration::from_secs(2));
        assert_eq!(
            timekeeper.delta(),
            DirectedTime::Future(Duration::from_secs(2))
        );
        timekeeper.update_real_time(Duration::from_secs(8));
        assert_eq!(
            timekeeper.delta(),
            DirectedTime::Future(Duration::from_secs(3))
        );
        timekeeper.set_time_factor(-1.0);
        timekeeper.add_simulation_time(Duration::from_secs(8));
        timekeeper.update_real_time(Duration::from_secs(2));
        assert_eq!(
            timekeeper.delta(),
            DirectedTime::Past(Duration::from_secs(2))
        );
        timekeeper.set_time_factor(-0.5);
        timekeeper.update_real_time(Duration::from_secs(2));
        assert_eq!(
            timekeeper.delta(),
            DirectedTime::Past(Duration::from_secs(1))
        );
        timekeeper.set_time_factor(-5.0);
        timekeeper.update_real_time(Duration::from_secs(2));
        assert_eq!(
            timekeeper.delta(),
            DirectedTime::Past(Duration::from_secs(5))
        );
    }

    #[test]
    fn duration_multiplication() {
        assert_eq!(
            mul_dur_by_factor(Duration::new(4, 600_000_000), 0.5),
            Duration::new(2, 300_000_000)
        );
        assert_eq!(
            mul_dur_by_factor(Duration::new(4, 600_000_000), 2.0),
            Duration::new(9, 200_000_000)
        );
        assert_eq!(
            mul_dur_by_factor(Duration::new(5, 600_000_000), 0.5),
            Duration::new(2, 800_000_000)
        );
    }
}
