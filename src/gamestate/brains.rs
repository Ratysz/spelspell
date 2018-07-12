use specs::prelude::*;
use std::marker::PhantomData;

use super::command::*;
use super::physics::*;
use super::time::*;

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

pub trait Brain: Component {
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

    fn run(&mut self, (time, mut entity_s, mut brain_s, brain_timing): Self::SystemData) {
        let delta = time.delta();
        for (entity, brain, _) in (&*entity_s, &mut brain_s, brain_timing.scheduled()).join() {
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

#[derive(SystemData)]
struct PlayerCommandsData<'a> {
    time: Write<'a, Timekeeper>,
    commands: Write<'a, GameCommandQueue>,
    entity: Entities<'a>,
    brain: WriteStorage<'a, PlayerBrain>,
    brain_timing: Write<'a, TimingData<PlayerBrain>>,
    movable: WriteStorage<'a, Movable>,
    movable_timing: Write<'a, TimingData<Movable>>,
}

impl<'a> System<'a> for PlayerCommands {
    type SystemData = PlayerCommandsData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (entity, mut brain, mut movable) in
            (&*data.entity, &mut data.brain, &mut data.movable).join()
        {
            while let Some(command) = data.commands.pop() {
                match command {
                    GameCommand::Move(direction) => {
                        let duration = Duration::from_millis(250);
                        data.time.add_simulation_time(duration);
                        info!("Move {:?}", direction);
                        movable.start_moving(
                            &entity,
                            &data.time,
                            &mut data.movable_timing,
                            direction,
                            duration,
                        );
                    }
                }
            }
        }
    }
}
