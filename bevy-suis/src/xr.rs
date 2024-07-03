use bevy::{
    app::Plugin,
    ecs::component::Component,
    math::{Quat, Ray3d, Vec3},
    prelude::App,
    transform::components::Transform,
};
use bevy_xr::{actions::ActionState, hands::LeftHand};

pub struct SUISXRPlugin;
impl Plugin for SUISXRPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, ());
    }
}

pub struct Joint {
    pos: Vec3,
    ori: Quat,
    radius: f32,
}
pub struct Finger {
    tip: Joint,
    distal: Joint,
    proximal: Joint,
    intermediate: Joint,
    metacarpal: Joint,
}
pub struct Thumb {
    tip: Joint,
    distal: Joint,
    intermediate: Joint,
    metacarpal: Joint,
}
pub struct Hand {
    thumb: Thumb,
    index: Finger,
    middle: Finger,
    ring: Finger,
    little: Finger,
}

pub enum InputDataType {
    Hand(LeftHand),
    // Controller() // lol gotta do this one using the openxr action system :/
    Pointer(Ray3d), // needs datamap tho
}

#[derive(Component, Debug)]
pub enum Field {
    Sphere(f32),
}
impl Field {
    pub fn closest_point(
        &self,
        this_transform: &Transform,
        reference_space: &Transform,
        point: Vec3,
    ) -> Vec3 {
        let reference_to_this_transform =
            reference_space.compute_matrix().inverse() * this_transform.compute_matrix();
        let local_point = reference_to_this_transform.transform_point3(point);

        let local_closest_point = match self {
            Field::Sphere(r) => local_point.normalize() * (local_point.length() - r),
        };

        reference_to_this_transform
            .inverse()
            .transform_point3(local_closest_point)
    }

    pub fn distance(
        &self,
        this_transform: &Transform,
        reference_space: &Transform,
        point: Vec3,
    ) -> f32 {
        self.closest_point(this_transform, reference_space, point)
            .length()
    }
}

// fn clear_handler_old_input<(mut handlers: Query<&mut InputHandler<T>>) {
//     for mut handler in handlers.iter_mut() {
//         handler.previous_frame_data = std::mem::take(&mut handler.current_data);
//     }
// }
