use bevy::prelude::*;

#[derive(Debug,Clone,Copy,Default,States,Hash, Eq, PartialEq )]
pub enum MachineState {
    #[default]
    Idle,
    Running,
    Paused,
    EmergencyShutdown,
    EmergencyIdle,
    Shutdown,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self,app: &mut App) {
        app.add_state::<MachineState>()
            .add_systems(update,game_state_input_events);
    }
}
pub fn game_state_input_events(mut next_state:ResMut<NextState<MachineState>>,state:Res <State<Machinestate>>){
    println!("State changed to: {}",next_state);
}