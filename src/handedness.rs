use std::fmt::Debug;

use bevy_math::{vec3, Vec3};
//use glam::Vec3;

pub trait Handedness: Clone + Copy + Debug + 'static {
    const FORWARD_Z_SIGN: f32;
    const FORWARD: Vec3 = vec3(0.0, 0.0, Self::FORWARD_Z_SIGN);

    fn right_from_up_and_forward<V, U>(up: V, forward: V) -> U
    where
        V: Into<Vec3>,
        U: From<Vec3>;
    fn up_from_right_and_forward<V, U>(right: V, forward: V) -> U
    where
        V: Into<Vec3>,
        U: From<Vec3>;
}

#[derive(Clone, Copy, Debug)]
pub struct LeftHanded;

impl Handedness for LeftHanded {
    const FORWARD_Z_SIGN: f32 = 1.0;

    fn right_from_up_and_forward<V, U>(up: V, forward: V) -> U
    where
        V: Into<Vec3>,
        U: From<Vec3>,
    {
        let up: Vec3 = up.into();
        let forward: Vec3 = forward.into();

        let result = up.cross(forward);
        From::from(result)
    }

    fn up_from_right_and_forward<V, U>(right: V, forward: V) -> U
    where
        V: Into<Vec3>,
        U: From<Vec3>,
    {
        let right: Vec3 = right.into();
        let forward: Vec3 = forward.into();

        let result = forward.cross(right);
        From::from(result)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RightHanded;

impl Handedness for RightHanded {
    const FORWARD_Z_SIGN: f32 = -1.0;

    fn right_from_up_and_forward<V, U>(up: V, forward: V) -> U
    where
        V: Into<Vec3>,
        U: From<Vec3>,
    {
        let up: Vec3 = up.into();
        let forward: Vec3 = forward.into();

        let result = forward.cross(up);
        From::from(result)
    }

    fn up_from_right_and_forward<V, U>(right: V, forward: V) -> U
    where
        V: Into<Vec3>,
        U: From<Vec3>,
    {
        let right: Vec3 = right.into();
        let forward: Vec3 = forward.into();

        let result = right.cross(forward);
        From::from(result)
    }
}
