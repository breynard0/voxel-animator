use crate::{graphics::vertex::Vertex, utils};

use super::{layer::Layer, material::Material, voxel::Voxel};

pub struct Model {
    pub label: &'static str,
    pub value: Vec<Layer>,
}

const MAT: Material = Material {
    color: glam::vec4(0.3, 0.3, 0.6, 1.0),
};
pub fn get_model() -> Model {
    Model {
        label: "QuarterPyramid",
        value: vec![
            Layer {
                label: "layer_1",
                value: vec![
                    vec![
                        Voxel::new(true, MAT),
                        Voxel::new(true, MAT),
                        Voxel::new(true, MAT),
                    ],
                    vec![
                        Voxel::new(true, MAT),
                        Voxel::new(true, MAT),
                        Voxel::new(true, MAT),
                    ],
                    vec![
                        Voxel::new(true, MAT),
                        Voxel::new(true, MAT),
                        Voxel::new(true, MAT),
                    ],
                ],
            },
            Layer {
                label: "layer_2",
                value: vec![
                    vec![Voxel::new(true, MAT), Voxel::new(true, MAT)],
                    vec![Voxel::new(true, MAT), Voxel::new(true, MAT)],
                ],
            },
            Layer {
                label: "layer_3",
                value: vec![
                    vec![Voxel::new(true, MAT), Voxel::new(false, MAT)],
                    vec![Voxel::new(false, MAT), Voxel::new(true, MAT)],
                ],
            },
        ],
    }
}

pub fn gen_vert_idx(model: &Model) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices = vec![];
    let mut indices = vec![];

    #[rustfmt::skip]
    const INDICES_TOP: &[u32] = &[
        0, 1, 3,
        3, 1, 2
    ];

    #[rustfmt::skip]
    const INDICES_BOTTOM: &[u32] = &[
        4, 5, 7,
        5, 6, 7
    ];

    #[rustfmt::skip]
    const INDICES_FRONT: &[u32] = &[
        1, 5, 2,
        5, 6, 2
    ];
    #[rustfmt::skip]
    const INDICES_BACK: &[u32] = &[
        4, 7, 3,
        0, 4, 3
    ];
    #[rustfmt::skip]
    const INDICES_LEFT: &[u32] = &[
        0, 1, 4,
        4, 5, 1
    ];
    #[rustfmt::skip]
    const INDICES_RIGHT: &[u32] = &[
        7, 6, 2,
        7, 2, 3
    ];

    let mut layer_num = 0;
    let mut iteration = 0;
    for layer in &model.value {
        // println!("{:?}", layer);
        // Sides
        for ux in 0..layer.value.len() {
            for uz in 0..layer.value[ux].len() {
                if !layer.value[ux][uz].filled {
                    continue;
                }

                let x = ux as f32;
                let y = layer_num as f32;
                let z = uz as f32;

                // Push vertices
                vertices.push(Vertex::new(
                    [x, y + 1., z],
                    layer.value[ux][uz].material.color.into(),
                ));
                vertices.push(Vertex::new(
                    [x, y + 1., z + 1.],
                    layer.value[ux][uz].material.color.into(),
                ));
                vertices.push(Vertex::new(
                    [x + 1., y + 1., z + 1.],
                    layer.value[ux][uz].material.color.into(),
                ));
                vertices.push(Vertex::new(
                    [x + 1., y + 1., z],
                    layer.value[ux][uz].material.color.into(),
                ));
                vertices.push(Vertex::new(
                    [x, y, z],
                    layer.value[ux][uz].material.color.into(),
                ));
                vertices.push(Vertex::new(
                    [x, y, z + 1.],
                    layer.value[ux][uz].material.color.into(),
                ));
                vertices.push(Vertex::new(
                    [x + 1., y, z + 1.],
                    layer.value[ux][uz].material.color.into(),
                ));
                vertices.push(Vertex::new(
                    [x + 1., y, z],
                    layer.value[ux][uz].material.color.into(),
                ));

                // Push culled indices
                if !is_filled_at_offset(&model.value, x, z, layer_num, 0, 0, 1) {
                    push_indices(&mut indices, INDICES_TOP, iteration)
                }

                if !is_filled_at_offset(&model.value, x, z, layer_num, 0, 0, -1) {
                    push_indices(&mut indices, INDICES_BOTTOM, iteration)
                }

                if !is_filled_at_offset(&model.value, x, z, layer_num, 1, 0, 0) {
                    push_indices(&mut indices, INDICES_RIGHT, iteration)
                }

                if !is_filled_at_offset(&model.value, x, z, layer_num, -1, 0, 0) {
                    push_indices(&mut indices, INDICES_LEFT, iteration)
                }

                if !is_filled_at_offset(&model.value, x, z, layer_num, 0, 1, 0) {
                    push_indices(&mut indices, INDICES_FRONT, iteration)
                }

                if !is_filled_at_offset(&model.value, x, z, layer_num, 0, -1, 0) {
                    push_indices(&mut indices, INDICES_BACK, iteration)
                }

                iteration += 1;
            }
        }

        layer_num += 1;
    }

    // let indices = INDICES
    //     .iter()
    //     .cycle()
    //     .take(1
    //     .map(|(idx, val)| val + ((idx / 36) * 8) as u32)
    //     .collect::<Vec<_>>();

    (utils::normalize_scale(&vertices, -1.0, 1.0), indices)
}

fn is_filled_at_offset(
    tiles: &Vec<Layer>,
    x: f32,
    z: f32,
    layer: i32,
    offset_x: i32,
    offset_z: i32,
    offset_layer: i32,
) -> bool {
    let x = x as i32;
    let z = z as i32;

    let a = match tiles.get((layer + offset_layer) as usize) {
        Some(x) => x,
        None => return false,
    };

    let b = match a.value.get((x + offset_x) as usize) {
        Some(x) => x,
        None => return false,
    };

    let c = match b.get((z + offset_z) as usize) {
        Some(x) => x,
        None => return false,
    };

    c.filled
}

fn push_indices(vector: &mut Vec<u32>, indices: &[u32], iter: u32) {
    vector.push(indices[0]+(iter*8));
    vector.push(indices[1]+(iter*8));
    vector.push(indices[2]+(iter*8));
    vector.push(indices[3]+(iter*8));
    vector.push(indices[4]+(iter*8));
    vector.push(indices[5]+(iter*8));
}
