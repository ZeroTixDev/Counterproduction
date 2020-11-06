use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::render::camera::Camera;
use derive_new::*;

const SENSITIVITY: f32 = 0.2;

#[derive(Default)]
struct State {
    reader: EventReader<MouseMotion>,
}

#[derive(new)]
pub struct CameraLook {
    x: f32,
    y: f32,
    distance: f32,
    center: Vec3,
}

impl Default for CameraLook {
    fn default() -> Self {
        CameraLook {
            x: 0.0,
            y: 0.0,
            distance: 100.0,
            center: Vec3::zero(),
        }
    }
}

pub struct CameraPlugin;
impl CameraPlugin {
    fn setup(mut windows: ResMut<Windows>) {
        let window = windows.get_primary_mut().unwrap();
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }

    fn toggle_cursor(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
        let window = windows.get_primary_mut().unwrap();
        if input.just_pressed(KeyCode::Space) {
            window.set_cursor_lock_mode(!window.cursor_locked());
            window.set_cursor_visibility(!window.cursor_visible());
        }
    }

    fn mouse_motion_system(
        time: Res<Time>,
        mut state: ResMut<State>,
        mut windows: ResMut<Windows>,
        mouse_motion_events: Res<Events<MouseMotion>>,
        mut query: Query<(&mut CameraLook, &mut Transform, &mut Camera)>,
    ) {
        if !windows.get_primary_mut().unwrap().cursor_locked() {
            return;
        }
        let mut delta: Vec2 = Vec2::zero();
        for event in state.reader.iter(&mouse_motion_events) {
            delta += event.delta;
        }

        for (mut look, mut transform, _) in query.iter_mut() {
            look.x -= delta.x() * SENSITIVITY * time.delta_seconds;
            look.y -= delta.y() * SENSITIVITY * time.delta_seconds;

            look.y = look.y.clamp(0.01, 3.13);

            let rot = Quat::from_axis_angle(Vec3::unit_y(), look.x)
                * Quat::from_axis_angle(-Vec3::unit_x(), look.y);
            transform.translation = (rot * Vec3::new(0.0, 1.0, 0.0)) * look.distance + look.center;
            transform.look_at(Vec3::zero(), Vec3::unit_y());
        }
    }
}
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<State>()
            .add_startup_system(Self::setup.system())
            .add_system(Self::mouse_motion_system.system())
            .add_system(Self::toggle_cursor.system());
    }
}
