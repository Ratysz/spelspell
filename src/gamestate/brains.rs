use specs::prelude::*;
use std::marker::PhantomData;

use super::command::{GameCommand, GameCommandQueue};
use super::time::*;
use super::ComponentTracker;

pub fn module_systems<'a, 'b>(builder: DispatcherBuilder<'a, 'b>) -> DispatcherBuilder<'a, 'b> {
    builder
        .with(PlayerCommands, "player_commands", &[])
        .with(
            TimingSystem::<PlayerBrain>::new(),
            "player_brain_timing",
            &["player_commands"],
        )
        .with(
            BrainSystem::<PlayerBrain>::new(),
            "player_brain",
            &["player_brain_timing"],
        )
}

pub trait Brain {
    fn think(&mut self, delta: DirectedTime, entity: Entity);
}

struct BrainSystem<T> {
    phantom_data: PhantomData<T>,
}

impl<T> BrainSystem<T> {
    fn new() -> BrainSystem<T> {
        BrainSystem {
            phantom_data: PhantomData,
        }
    }
}

impl<'a, T> System<'a> for BrainSystem<T>
where
    T: Brain + Component + Send + Sync,
{
    type SystemData = (
        Read<'a, Timekeeper>,
        Entities<'a>,
        WriteStorage<'a, T>,
        Read<'a, TimingData<T>>,
    );

    fn run(&mut self, (time, mut entity_s, mut brain_s, timing_data): Self::SystemData) {
        let delta = time.delta();
        for (entity, brain, _) in (&*entity_s, &mut brain_s, timing_data.scheduled()).join() {
            brain.think(delta, entity);
        }
    }

    fn setup(&mut self, resources: &mut Resources) {
        Self::SystemData::setup(resources);
        let mut storage: WriteStorage<T> = SystemData::fetch(&resources);
    }
}

#[derive(Component, Debug)]
#[storage(HashMapStorage)]
pub struct PlayerBrain {}

impl Brain for PlayerBrain {
    fn think(&mut self, delta: DirectedTime, entity: Entity) {
        trace!("{:?} is thinking... {:?}", entity, delta);
    }
}

impl Timed for PlayerBrain {}

struct PlayerCommands;

impl<'a> System<'a> for PlayerCommands {
    type SystemData = (
        Write<'a, Timekeeper>,
        Write<'a, GameCommandQueue>,
        Entities<'a>,
        WriteStorage<'a, PlayerBrain>,
    );

    fn run(&mut self, (mut time, mut commands, entity_s, mut brain_s): Self::SystemData) {
        for (entity, mut brain) in (&*entity_s, &mut brain_s).join() {
            while let Some(command) = commands.pop() {
                match command {
                    GameCommand::Move(dir) => {
                        time.add_simulation_time(Duration::from_millis(250));
                        info!("Move {:?}", dir);
                    }
                }
            }
        }
    }
}
