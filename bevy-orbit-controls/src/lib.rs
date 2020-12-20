use bevy::input::mouse::MouseMotion;
use bevy::input::mouse::MouseScrollUnit::Line;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::render::camera::Camera;

const ROTATE_SENSITIVITY: f32 = 0.2;
const ZOOM_SENSITIVITY: f32 = 0.8;

#[derive(Default)]
struct State {
    motion: EventReader<MouseMotion>,
    scroll: EventReader<MouseWheel>,
}

pub struct OrbitCamera {
    x: f32,
    y: f32,
    distance: f32,
    center: Vec3,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        OrbitCamera {
            x: 0.0,
            y: 0.0,
            distance: 100.0,
            center: Vec3::zero(),
        }
    }
}

pub struct CameraPlugin;
impl CameraPlugin {
    fn mouse_motion_system(
        time: Res<Time>,
        mut state: ResMut<State>,
        mouse_motion_events: Res<Events<MouseMotion>>,
        mouse_button_input: Res<Input<MouseButton>>,
        mut query: Query<(&mut OrbitCamera, &mut Transform, &mut Camera)>,
    ) {
        let mut delta = Vec2::zero();
        for event in state.motion.iter(&mouse_motion_events) {
            delta += event.delta;
        }
        for (mut camera, mut transform, _) in query.iter_mut() {
            if mouse_button_input.pressed(MouseButton::Middle) {
                camera.x -= delta.x * ROTATE_SENSITIVITY * time.delta_seconds();
                camera.y -= delta.y * ROTATE_SENSITIVITY * time.delta_seconds();

                camera.y = camera.y.clamp(0.01, 3.13);

                let rot = Quat::from_axis_angle(Vec3::unit_y(), camera.x)
                    * Quat::from_axis_angle(-Vec3::unit_x(), camera.y);
                transform.translation =
                    (rot * Vec3::new(0.0, 1.0, 0.0)) * camera.distance + camera.center;
                transform.look_at(camera.center, Vec3::unit_y());
            }
        }
    }

    fn zoom_system(
        mut state: ResMut<State>,
        mouse_wheel_events: Res<Events<MouseWheel>>,
        mut query: Query<(&mut OrbitCamera, &mut Transform, &mut Camera)>,
    ) {
        let mut total = 0.0;
        for event in state.scroll.iter(&mouse_wheel_events) {
            if let Line = event.unit {
                total += event.y;
            } else {
                panic!("Invalid Scroll Event: {:?}", event);
            }
        }
        for (mut camera, mut transform, _) in query.iter_mut() {
            camera.distance *= ZOOM_SENSITIVITY.powf(total);
            let translation = &mut transform.translation;
            *translation =
                (*translation - camera.center).normalize() * camera.distance + camera.center;
        }
    }
}
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<State>()
            .add_system(Self::mouse_motion_system.system())
            .add_system(Self::zoom_system.system());
    }
}
