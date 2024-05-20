use glam::{Mat4, Quat, Vec3};

pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl From<Transform> for Mat4 {
    fn from(transfrom: Transform) -> Mat4 {
        transfrom.get_local()
    }
}

impl Transform {
    pub const IDENTITY: Self = Self {
        translation: Vec3::ZERO,
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    };

    pub fn create(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            translation: translation,
            rotation: rotation.normalize(),
            scale,
        }
    }

    pub fn get_local(&self) -> Mat4 {
        Mat4::from_translation(self.translation)
            * Mat4::from_quat(self.rotation)
            * Mat4::from_scale(self.scale)
    }

    pub fn from_translation(translation: Vec3) -> Self {
        Self {
            translation,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    pub fn from_rotation(rotation: Quat) -> Self {
        Self {
            translation: Vec3::new(0.0, 0.0, 0.0),
            rotation: rotation.normalize(),
            scale: Vec3::ONE,
        }
    }

    pub fn from_translation_rotation(translation: Vec3, rotation: Quat) -> Self {
        Self {
            translation,
            rotation: rotation.normalize(),
            scale: Vec3::ONE,
        }
    }

    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }

    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }

    pub fn forward(&self) -> Vec3 {
        self.rotation * -Vec3::Z
    }
}
