use specs::prelude::*;
use std::marker::PhantomData;

use super::command::{GameCommand, GameCommandQueue};
use super::time::{DirectedTime, Duration, Timekeeper};

pub struct BrainSystem<T>(PhantomData<T>);

impl<T> BrainSystem<T> {
    pub fn new() -> Self {
        BrainSystem(PhantomData)
    }
}

trait Brain {}

impl<'a, T: Brain + Component> System<'a> for BrainSystem<T> {
    type SystemData = (Read<'a, Timekeeper>, Entities<'a>, WriteStorage<'a, T>);

    fn run(&mut self, (time, entity_s, brain_s): Self::SystemData) {}
}

#[derive(Component, Debug)]
pub struct PlayerBrain {}

impl Brain for PlayerBrain {}

impl<'a> System<'a> for PlayerBrain {
    type SystemData = (
        Write<'a, Timekeeper>,
        Write<'a, GameCommandQueue>,
        Entities<'a>,
        WriteStorage<'a, PlayerBrain>,
    );

    fn run(&mut self, (mut time, mut commands, entity_s, mut brain_s): Self::SystemData) {
        for (entity, brain) in (&*entity_s, &mut brain_s).join() {
            if let DirectedTime::Future(delta) = time.sim_delta() {
                info!("sim delta: {:?}", delta);
            }
            while let Some(command) = commands.pop() {
                match command {
                    GameCommand::Move(dir) => {
                        time.add_sim_time(Duration::from_secs(1));
                        info!("Move {:?}", dir);
                    }
                }
            }
        }
    }
}
