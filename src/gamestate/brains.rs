use specs::prelude::*;
use std::marker::PhantomData;

use super::command::{GameCommand, GameCommandQueue};
use super::time::{DirectedTime, Duration, Timekeeper};

pub fn module_systems<'a, 'b>(builder: DispatcherBuilder<'a, 'b>) -> DispatcherBuilder<'a, 'b> {
    builder
        .with(BrainSystem::<PlayerBrain>::new(), "player_brain", &[])
        .with(PlayerBrainCommands, "player_brain_commands", &[])
}

pub struct BrainSystem<T>(PhantomData<T>);

impl<T> BrainSystem<T> {
    pub fn new() -> Self {
        BrainSystem(PhantomData)
    }
}

trait Brain {
    fn think(&mut self, delta: DirectedTime, entity: Entity);
}

impl<'a, T: Brain + Component> System<'a> for BrainSystem<T> {
    type SystemData = (Read<'a, Timekeeper>, Entities<'a>, WriteStorage<'a, T>);

    fn run(&mut self, (time, mut entity_s, mut brain_s): Self::SystemData) {
        let delta = time.delta();
        for (entity, brain) in (&*entity_s, &mut brain_s).join() {
            brain.think(delta, entity);
        }
    }
}

#[derive(Component, Debug)]
pub struct PlayerBrain {}

impl Brain for PlayerBrain {
    fn think(&mut self, delta: DirectedTime, entity: Entity) {
        if delta != DirectedTime::Still {
            trace!("{:?} is thinking... {:?}", entity, delta);
        }
    }
}

struct PlayerBrainCommands;

impl<'a> System<'a> for PlayerBrainCommands {
    type SystemData = (
        Write<'a, Timekeeper>,
        Write<'a, GameCommandQueue>,
        Entities<'a>,
        WriteStorage<'a, PlayerBrain>,
    );

    fn run(&mut self, (mut time, mut commands, entity_s, mut brain_s): Self::SystemData) {
        for (entity, brain) in (&*entity_s, &mut brain_s).join() {
            while let Some(command) = commands.pop() {
                match command {
                    GameCommand::Move(dir) => {
                        time.add_simulation_time(Duration::from_secs(1));
                        info!("Move {:?}", dir);
                    }
                }
            }
        }
    }
}
