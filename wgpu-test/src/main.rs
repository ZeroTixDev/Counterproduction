use bevy::prelude::*;

/// This example illustrates how to create a texture for use with a texture2DArray shader uniform variable.
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(loop_system.system())
        .run();
}

fn setup(commands: &mut Commands) {
    
}

fn loop_system() {

}