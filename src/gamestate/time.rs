use specs::prelude::*;
use std::cmp::min;
use std::time::Duration;

const ZERO_DURATION: Duration = Duration::from_secs(0);

pub struct Timekeeper {
    real_time_delta: Duration,
    sim_timer: Duration,
    sim_delta: Option<Duration>,
    sim_time_factor: f32,
}

impl Default for Timekeeper {
    fn default() -> Self {
        Self::new()
    }
}

impl Timekeeper {
    pub fn new() -> Timekeeper {
        Timekeeper {
            real_time_delta: ZERO_DURATION,
            sim_timer: ZERO_DURATION,
            sim_delta: None,
            sim_time_factor: 1.0,
        }
    }

    pub fn update_real_time(&mut self, d_time: Duration) {
        self.real_time_delta = d_time;
        if self.sim_timer > ZERO_DURATION {
            let adjusted = mul_dur_by_f32(self.real_time_delta, self.sim_time_factor);
            let time_chunk = min(adjusted, self.sim_timer);
            self.sim_timer -= time_chunk;
            self.sim_delta = Some(time_chunk);
        } else {
            self.sim_delta = None;
        }
    }

    pub fn get_real_delta(&self) -> Duration {
        self.real_time_delta
    }

    pub fn set_sim_time_factor(&mut self, factor: f32) {
        self.sim_time_factor = factor;
    }

    pub fn add_sim_time(&mut self, d_time: Duration) {
        self.sim_timer += d_time;
    }

    pub fn get_sim_delta(&self) -> Option<Duration> {
        self.sim_delta
    }

    pub fn get_sim_time_factor(&self) -> f32 {
        self.sim_time_factor
    }
}

fn mul_dur_by_f32(duration: Duration, factor: f32) -> Duration {
    let adjusted_s: f64 = duration.as_secs() as f64 * factor as f64;
    let mut adjusted_n: f64 = duration.subsec_nanos() as f64 * factor as f64;
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
    fn basic_timekeeping() {
        let mut timekeeper = Timekeeper::new();
        assert_eq!(timekeeper.get_sim_delta(), None);
        timekeeper.update_real_time(Duration::from_secs(2));
        assert_eq!(timekeeper.get_sim_delta(), None);
        timekeeper.add_sim_time(Duration::from_secs(8));
        timekeeper.update_real_time(Duration::from_secs(2));
        assert_eq!(timekeeper.get_sim_delta(), Some(Duration::from_secs(2)));
        timekeeper.update_real_time(Duration::from_secs(2));
        assert_eq!(timekeeper.get_sim_delta(), Some(Duration::from_secs(2)));
        timekeeper.update_real_time(Duration::from_secs(3));
        assert_eq!(timekeeper.get_sim_delta(), Some(Duration::from_secs(3)));
        timekeeper.update_real_time(Duration::from_secs(2));
        assert_eq!(timekeeper.get_sim_delta(), Some(Duration::from_secs(1)));
        timekeeper.update_real_time(Duration::from_secs(1));
        assert_eq!(timekeeper.get_sim_delta(), None);
    }

    #[test]
    fn time_factor() {
        let mut timekeeper = Timekeeper::new();
        timekeeper.add_sim_time(Duration::from_secs(8));
        timekeeper.update_real_time(Duration::from_secs(2));
        assert_eq!(timekeeper.get_sim_delta(), Some(Duration::from_secs(2)));
        timekeeper.set_sim_time_factor(2.0);
        assert_eq!(timekeeper.get_sim_delta(), Some(Duration::from_secs(2)));
        timekeeper.update_real_time(Duration::from_secs(2));
        assert_eq!(timekeeper.get_sim_delta(), Some(Duration::from_secs(4)));
        timekeeper.update_real_time(Duration::from_secs(2));
        assert_eq!(timekeeper.get_sim_delta(), Some(Duration::from_secs(2)));
        timekeeper.update_real_time(Duration::from_secs(2));
        assert_eq!(timekeeper.get_sim_delta(), None);
        timekeeper.set_sim_time_factor(0.5);
        timekeeper.add_sim_time(Duration::from_secs(8));
        assert_eq!(timekeeper.get_sim_delta(), None);
        timekeeper.update_real_time(Duration::from_secs(2));
        assert_eq!(timekeeper.get_sim_delta(), Some(Duration::from_secs(1)));
        timekeeper.update_real_time(Duration::from_secs(4));
        assert_eq!(timekeeper.get_sim_delta(), Some(Duration::from_secs(2)));
        timekeeper.set_sim_time_factor(1.0);
        timekeeper.update_real_time(Duration::from_secs(2));
        assert_eq!(timekeeper.get_sim_delta(), Some(Duration::from_secs(2)));
        timekeeper.update_real_time(Duration::from_secs(8));
        assert_eq!(timekeeper.get_sim_delta(), Some(Duration::from_secs(3)));
    }

    #[test]
    fn duration_multiplication() {
        assert_eq!(
            mul_dur_by_f32(Duration::new(4, 600_000_000), 0.5),
            Duration::new(2, 300_000_000)
        );
        assert_eq!(
            mul_dur_by_f32(Duration::new(4, 600_000_000), 2.0),
            Duration::new(9, 200_000_000)
        );
        assert_eq!(
            mul_dur_by_f32(Duration::new(5, 600_000_000), 0.5),
            Duration::new(2, 800_000_000)
        );
    }
}
