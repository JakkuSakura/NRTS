use bevy::app::App;
use bevy::prelude::{IntoSystem, Plugin};
use crate::movement::system_movement;

pub struct NrtsCore {

}
impl Plugin for NrtsCore {
    fn build(&self, app: &mut App) {
        app.add_system(system_movement.system());
    }
}