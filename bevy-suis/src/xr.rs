use bevy::prelude::*;
use bevy_mod_xr::hands::{HandBone, HandBoneRadius, HAND_JOINT_COUNT};

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
impl Joint {
    const fn empty() -> Self {
        Self {
            pos: Vec3::ZERO,
            ori: Quat::IDENTITY,
            radius: 0.0,
        }
    }
}
pub struct Finger {
    tip: Joint,
    distal: Joint,
    proximal: Joint,
    intermediate: Joint,
    metacarpal: Joint,
}
impl Finger {
    const fn empty() -> Self {
        Self {
            tip: Joint::empty(),
            distal: Joint::empty(),
            proximal: Joint::empty(),
            intermediate: Joint::empty(),
            metacarpal: Joint::empty(),
        }
    }
    fn set_joint(&mut self, bone: &HandBone, joint: Joint) {
        match bone {
            HandBone::IndexMetacarpal
            | HandBone::MiddleMetacarpal
            | HandBone::RingMetacarpal
            | HandBone::LittleMetacarpal => self.metacarpal = joint,
            HandBone::IndexProximal
            | HandBone::MiddleProximal
            | HandBone::RingProximal
            | HandBone::LittleProximal => self.proximal = joint,
            HandBone::IndexIntermediate
            | HandBone::MiddleIntermediate
            | HandBone::RingIntermediate
            | HandBone::LittleIntermediate => self.intermediate = joint,
            HandBone::IndexDistal
            | HandBone::MiddleDistal
            | HandBone::RingDistal
            | HandBone::LittleDistal => self.distal = joint,
            HandBone::IndexTip | HandBone::MiddleTip | HandBone::RingTip | HandBone::LittleTip => {
                self.tip = joint
            }
            _ => (),
        }
    }
}
pub struct Thumb {
    tip: Joint,
    distal: Joint,
    proximal: Joint,
    metacarpal: Joint,
}
impl Thumb {
    fn set_joint(&mut self, bone: &HandBone, joint: Joint) {
        match bone {
            HandBone::ThumbMetacarpal => self.metacarpal = joint,
            HandBone::ThumbProximal => self.proximal = joint,
            HandBone::ThumbDistal => self.distal = joint,
            HandBone::ThumbTip => self.tip = joint,
            _ => (),
        }
    }
}
impl Thumb {
    const fn empty() -> Self {
        Self {
            tip: Joint::empty(),
            distal: Joint::empty(),
            proximal: Joint::empty(),
            metacarpal: Joint::empty(),
        }
    }
}
pub struct Hand {
    thumb: Thumb,
    index: Finger,
    middle: Finger,
    ring: Finger,
    little: Finger,
}
impl Hand {
    pub fn from_query(
        entities: &[Entity; HAND_JOINT_COUNT],
        query: &Query<(&GlobalTransform, &HandBoneRadius, &HandBone)>,
    ) -> Option<Hand> {
        let mut hand = Hand::empty();
        for e in entities.iter() {
            let (transform, radius, bone) = query.get(*e).ok()?;
            let (_, rot, pos) = transform.to_scale_rotation_translation();
            let joint = Joint {
                pos,
                ori: rot,
                radius: radius.0,
            };
            if bone.is_thumb() {
                hand.thumb.set_joint(bone, joint);
                continue;
            }
            if bone.is_index() {
                hand.index.set_joint(bone, joint);
                continue;
            }
            if bone.is_middle() {
                hand.middle.set_joint(bone, joint);
                continue;
            }
            if bone.is_ring() {
                hand.ring.set_joint(bone, joint);
                continue;
            }
            if bone.is_little() {
                hand.little.set_joint(bone, joint);
                continue;
            }
        }
        Some(hand)
    }
    const fn empty() -> Hand {
        Hand {
            thumb: Thumb::empty(),
            index: Finger::empty(),
            middle: Finger::empty(),
            ring: Finger::empty(),
            little: Finger::empty(),
        }
    }
}

pub enum InputDataType {
    Hand(Hand),
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
