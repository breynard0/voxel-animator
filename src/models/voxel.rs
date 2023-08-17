use super::material::Material;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Voxel {
    pub filled: bool,
    pub material: Material,
}

impl Voxel {
    pub fn new(filled: bool, material: Material) -> Self {
        Self { filled, material }
    }

    pub fn filled(&self) -> Self {
        Self {
            filled: true,
            material: self.material,
        }
    }

    pub fn empty(&self) -> Self {
        Self {
            filled: false,
            material: self.material,
        }
    }
}
