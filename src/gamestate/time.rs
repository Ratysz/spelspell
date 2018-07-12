use specs::join::JoinIter;
use specs::prelude::*;
use specs::world::Index;
use std::cmp::min;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::ops::{Add, Sub};
use std::sync::{Arc, Weak};
pub use std::time::Duration;

const ZERO_DURATION: Duration = Duration::from_secs(0);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
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

impl Add<Duration> for Instant {
    type Output = Instant;

    fn add(self, rhs: Duration) -> Instant {
        Instant(self.0 + rhs)
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;

    fn sub(self, rhs: Duration) -> Instant {
        Instant(self.0 - rhs)
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
    sim_closest_future: Duration,
    sim_closest_past: Duration,
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
            remaining_sim_time: ZERO_DURATION,
            sim_delta: DirectedTime::Still,
            sim_time_factor: 1.0,
            sim_elapsed_time: ZERO_DURATION,
            sim_closest_future: ZERO_DURATION,
            sim_closest_past: ZERO_DURATION,
        }
    }

    pub fn update_real_time(&mut self, d_time: Duration) {
        self.real_time_delta = d_time;
        if self.remaining_sim_time > ZERO_DURATION {
            let adjusted = mul_dur_by_factor(self.real_time_delta, self.sim_time_factor.abs());
            let signum = self.sim_time_factor.signum();
            if signum > 0.0 {
                let time_chunk = min(
                    adjusted,
                    if self.sim_closest_future != ZERO_DURATION {
                        min(
                            self.remaining_sim_time,
                            self.sim_closest_future - self.sim_elapsed_time,
                        )
                    } else {
                        self.remaining_sim_time
                    },
                );
                self.remaining_sim_time -= time_chunk;
                self.sim_elapsed_time += time_chunk;
                self.sim_delta = DirectedTime::Future(time_chunk);
            } else if signum < 0.0 {
                let time_chunk = min(
                    adjusted,
                    if self.sim_closest_past != ZERO_DURATION {
                        min(
                            self.remaining_sim_time,
                            self.sim_elapsed_time - self.sim_closest_past,
                        )
                    } else {
                        self.remaining_sim_time
                    },
                );
                self.remaining_sim_time -= time_chunk;
                self.sim_elapsed_time -= time_chunk;
                self.sim_delta = DirectedTime::Past(time_chunk);
            } else {
                self.sim_delta = DirectedTime::Still;
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
}

pub trait Timed: Component {
    fn schedule(
        &self,
        entity: &Entity,
        time: &Timekeeper,
        timing_data: &mut TimingData<Self>,
        duration: Duration,
    ) {
        timing_data.schedule(entity, time, duration);
    }
}

pub struct TimingData<T> {
    phantom_data: PhantomData<T>,
    should_update: BitSet,
    starts: BTreeMap<Instant, Vec<Index>>,
    ends: BTreeMap<Instant, Vec<Index>>,
}

impl<T> Default for TimingData<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> TimingData<T> {
    fn new() -> TimingData<T> {
        TimingData {
            phantom_data: PhantomData,
            should_update: BitSet::new(),
            ends: BTreeMap::new(),
            starts: BTreeMap::new(),
        }
    }

    fn clear_update_flags(&mut self) {
        self.should_update.clear();
    }

    fn set_update_flag(&mut self, entity: &Entity) {
        self.should_update.add(entity.id());
    }

    fn schedule(&mut self, entity: &Entity, time: &Timekeeper, duration: Duration) {
        let (start, end) = match time.delta() {
            DirectedTime::Past(_) => (time.now() - duration, time.now()),
            _ => (time.now(), time.now() + duration),
        };
        self.starts
            .entry(start)
            .or_insert_with(Vec::new)
            .push(entity.id());
        self.ends
            .entry(end)
            .or_insert_with(Vec::new)
            .push(entity.id());
        info!("scheduled {:?} for {:?}-{:?}", entity, start, end);
    }

    /*fn populate_schedule<C>(&mut self, join: JoinIter<(Entities, ReadStorage<C>)>, time: Timekeeper)
    where
        C: Component + Timed,
    {

    }*/

    pub fn scheduled(&self) -> &BitSet {
        &self.should_update
    }
}

pub struct TimingSystem<T> {
    phantom_data: PhantomData<T>,
}

impl<T> TimingSystem<T> {
    pub fn new() -> TimingSystem<T> {
        TimingSystem {
            phantom_data: PhantomData,
        }
    }
}

impl<'a, T> System<'a> for TimingSystem<T>
where
    T: Timed + Component + Send + Sync,
{
    type SystemData = (
        Write<'a, Timekeeper>,
        Entities<'a>,
        ReadStorage<'a, T>,
        Write<'a, TimingData<T>>,
    );

    fn run(&mut self, (time, entity_s, timed_s, mut timing_data): Self::SystemData) {
        timing_data.clear_update_flags();
        match time.delta() {
            DirectedTime::Still => (),
            _ => {
                for (entity, _) in (&*entity_s, &timed_s).join() {
                    timing_data.set_update_flag(&entity);
                }
            }
        }
        // TODO: Timekeeper::sim_closest_future, Timekeeper::sim_closest_past
        /*match time.delta() {
            DirectedTime::Still => (),
            other => {
                let (start, end) = match other {
                    DirectedTime::Future(delta) => (time.now() - delta, time.now()),
                    DirectedTime::Past(delta) => (time.now(), time.now() + delta),
                    _ => unreachable!("Literally impossible."),
                };
                for (entity, _) in (&*entity_s, &timed_s).join() {
                    timing_data.set_update_flag(&entity);
                }
            }
        };*/
    }

    fn setup(&mut self, resources: &mut Resources) {
        Self::SystemData::setup(resources);
        resources.insert(TimingData::<T>::new());
    }
}

fn mul_dur_by_factor<T: Copy + Into<f64>>(duration: Duration, factor: T) -> Duration {
    let adjusted_s: f64 = duration.as_secs() as f64 * factor.into();
    let mut adjusted_n: f64 = duration.subsec_nanos() as f64 * factor.into();
    let trunc_s = adjusted_s.trunc();
    let subsec = adjusted_s - trunc_s;
    let mut adjusted_s = trunc_s as u64;
    adjusted_n += subsec * 1_000_000_000.0;
    let rem_n = adjusted_n % 1_000_000_000.0;
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
