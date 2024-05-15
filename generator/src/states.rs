use bevvy::prelude::*;

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
        app.add_state::<MachineState>();

    }
}