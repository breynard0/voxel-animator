use super::{layer::Layer, material::Material, voxel::Voxel};

pub struct Model {
    pub label: &'static str,
    pub value: Vec<Layer>,
}

const MAT: Material = Material {
    color: glam::vec4(0.3, 0.3, 0.6, 1.0),
};
const MAT2: Material = Material {
    color: glam::vec4(0.6, 0.3, 0.3, 1.0),
};

// Test model for now with hardcoded materials
pub fn get_model() -> Model {
    Model {
        label: "QuarterPyramid",
        value: vec![
            Layer {
                label: "layer_1",
                value: vec![
                    vec![
                        Voxel::new(true, MAT),
                        Voxel::new(true, MAT2),
                        Voxel::new(true, MAT),
                    ],
                    vec![
                        Voxel::new(true, MAT2),
                        Voxel::new(true, MAT),
                        Voxel::new(true, MAT2),
                    ],
                    vec![
                        Voxel::new(true, MAT),
                        Voxel::new(true, MAT2),
                        Voxel::new(true, MAT),
                    ],
                ],
            },
            Layer {
                label: "layer_2",
                value: vec![
                    vec![Voxel::new(true, MAT), Voxel::new(true, MAT2)],
                    vec![Voxel::new(true, MAT2), Voxel::new(true, MAT)],
                ],
            },
            Layer {
                label: "layer_3",
                value: vec![
                    vec![Voxel::new(true, MAT2), Voxel::new(false, MAT)],
                    vec![Voxel::new(false, MAT), Voxel::new(true, MAT)],
                ],
            },
        ],
    }
}
