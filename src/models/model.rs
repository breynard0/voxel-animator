use crate::{
    graphics::vertex::Vertex,
    utils::{self, log},
};

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

pub fn gen_vert_idx(model: &Model) -> (Vec<Vertex>, Vec<u32>) {
    let pretime = std::time::Instant::now();
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
    let mut offset = 0;
    let mut culled = vec![];

    for layer in &model.value {
        // Sides
        for ux in 0..layer.value.len() {
            for uz in 0..layer.value[ux].len() {
                if !layer.value[ux][uz].filled {
                    continue;
                }

                let x = ux as f32;
                let y = layer_num as f32;
                let z = uz as f32;

                let top_condition = !is_filled_at_offset(&model.value, x, z, layer_num, 0, 0, 1);
                let bottom_condition =
                    !is_filled_at_offset(&model.value, x, z, layer_num, 0, 0, -1);
                let right_condition = !is_filled_at_offset(&model.value, x, z, layer_num, 1, 0, 0);
                let left_condition = !is_filled_at_offset(&model.value, x, z, layer_num, -1, 0, 0);
                let front_condition = !is_filled_at_offset(&model.value, x, z, layer_num, 0, 1, 0);
                let back_condition = !is_filled_at_offset(&model.value, x, z, layer_num, 0, -1, 0);

                // Push vertices
                let left_up_back = Vertex::new(
                    [x, y + 1., z],
                    layer.value[ux][uz].material.color.into(),
                    get_normal(
                        [0.0, 1.0, 0.0],
                        left_condition,
                        back_condition,
                        is_filled_at_offset(&model.value, x, z, layer_num, 0, 0, 1),
                    ),
                );
                let left_up_back_dup = vertices
                    .iter()
                    .position(|v: &Vertex| v.pos == left_up_back.pos);

                let left_up_front = Vertex::new(
                    [x, y + 1., z + 1.],
                    layer.value[ux][uz].material.color.into(),
                    get_normal(
                        [0.0, 1.0, 1.0],
                        left_condition,
                        front_condition,
                        is_filled_at_offset(&model.value, x, z, layer_num, 0, 0, 1),
                    ),
                );
                let left_up_front_dup = vertices
                    .iter()
                    .position(|v: &Vertex| v.pos == left_up_front.pos);

                let right_up_front = Vertex::new(
                    [x + 1., y + 1., z + 1.],
                    layer.value[ux][uz].material.color.into(),
                    get_normal(
                        [1.0, 1.0, 1.0],
                        right_condition,
                        front_condition,
                        is_filled_at_offset(&model.value, x, z, layer_num, 0, 0, 1),
                    ),
                );
                let right_up_front_dup = vertices
                    .iter()
                    .position(|v: &Vertex| v.pos == right_up_front.pos);

                let right_up_back = Vertex::new(
                    [x + 1., y + 1., z],
                    layer.value[ux][uz].material.color.into(),
                    get_normal(
                        [1.0, 1.0, 0.0],
                        right_condition,
                        back_condition,
                        is_filled_at_offset(&model.value, x, z, layer_num, 0, 0, 1),
                    ),
                );
                let right_up_back_dup = vertices
                    .iter()
                    .position(|v: &Vertex| v.pos == right_up_back.pos);

                let left_down_back = Vertex::new(
                    [x, y, z],
                    layer.value[ux][uz].material.color.into(),
                    get_normal(
                        [0.0, 0.0, 0.0],
                        left_condition,
                        back_condition,
                        is_filled_at_offset(&model.value, x, z, layer_num, 0, 0, -1),
                    ),
                );
                let left_down_back_dup = vertices
                    .iter()
                    .position(|v: &Vertex| v.pos == left_down_back.pos);

                let left_down_front = Vertex::new(
                    [x, y, z + 1.],
                    layer.value[ux][uz].material.color.into(),
                    get_normal(
                        [0.0, 0.0, 1.0],
                        left_condition,
                        front_condition,
                        is_filled_at_offset(&model.value, x, z, layer_num, 0, 0, -1),
                    ),
                );
                let left_down_front_dup = vertices
                    .iter()
                    .position(|v: &Vertex| v.pos == left_down_front.pos);

                let right_down_front = Vertex::new(
                    [x + 1., y, z + 1.],
                    layer.value[ux][uz].material.color.into(),
                    get_normal(
                        [1.0, 0.0, 1.0],
                        right_condition,
                        front_condition,
                        is_filled_at_offset(&model.value, x, z, layer_num, 0, 0, -1),
                    ),
                );
                let right_down_front_dup = vertices
                    .iter()
                    .position(|v: &Vertex| v.pos == right_down_front.pos);

                let right_down_back = Vertex::new(
                    [x + 1., y, z],
                    layer.value[ux][uz].material.color.into(),
                    get_normal(
                        [1.0, 0.0, 0.0],
                        right_condition,
                        back_condition,
                        is_filled_at_offset(&model.value, x, z, layer_num, 0, 0, -1),
                    ),
                );
                let right_down_back_dup = vertices
                    .iter()
                    .position(|v: &Vertex| v.pos == right_down_back.pos);

                // Represents vertex size afterwards

                // Push vertices
                // The value of the index it's REPLACING must be appeneded to 'culled'
                if left_up_back_dup.is_none() {
                    vertices.push(left_up_back);
                } else {
                    utils::push_if_absent(&mut culled, (0 + offset) as usize)
                }
                if left_up_front_dup.is_none() {
                    vertices.push(left_up_front);
                } else {
                    utils::push_if_absent(&mut culled, (1 + offset) as usize)
                }
                if right_up_front_dup.is_none() {
                    vertices.push(right_up_front);
                } else {
                    utils::push_if_absent(&mut culled, (2 + offset) as usize)
                }
                if right_up_back_dup.is_none() {
                    vertices.push(right_up_back);
                } else {
                    utils::push_if_absent(&mut culled, (3 + offset) as usize)
                }
                if left_down_back_dup.is_none() {
                    vertices.push(left_down_back);
                } else {
                    utils::push_if_absent(&mut culled, (4 + offset) as usize)
                }
                if left_down_front_dup.is_none() {
                    vertices.push(left_down_front);
                } else {
                    utils::push_if_absent(&mut culled, (5 + offset) as usize)
                }
                if right_down_front_dup.is_none() {
                    vertices.push(right_down_front);
                } else {
                    utils::push_if_absent(&mut culled, (6 + offset) as usize)
                }
                if right_down_back_dup.is_none() {
                    vertices.push(right_down_back);
                } else {
                    utils::push_if_absent(&mut culled, (7 + offset) as usize)
                }

                // Push culled indices
                let duplicate_list = &vec![
                    match left_up_back_dup {
                        Some(v) => v as i32,
                        None => -1,
                    },
                    match left_up_front_dup {
                        Some(v) => v as i32,
                        None => -1,
                    },
                    match right_up_front_dup {
                        Some(v) => v as i32,
                        None => -1,
                    },
                    match right_up_back_dup {
                        Some(v) => v as i32,
                        None => -1,
                    },
                    match left_down_back_dup {
                        Some(v) => v as i32,
                        None => -1,
                    },
                    match left_down_front_dup {
                        Some(v) => v as i32,
                        None => -1,
                    },
                    match right_down_front_dup {
                        Some(v) => v as i32,
                        None => -1,
                    },
                    match right_down_back_dup {
                        Some(v) => v as i32,
                        None => -1,
                    },
                ];

                if top_condition {
                    push_indices(&mut indices, INDICES_TOP, offset, &culled, duplicate_list);
                }

                if bottom_condition {
                    push_indices(
                        &mut indices,
                        INDICES_BOTTOM,
                        offset,
                        &culled,
                        duplicate_list,
                    );
                }

                if right_condition {
                    push_indices(&mut indices, INDICES_RIGHT, offset, &culled, duplicate_list);
                }

                if left_condition {
                    push_indices(&mut indices, INDICES_LEFT, offset, &culled, duplicate_list);
                }

                if front_condition {
                    push_indices(&mut indices, INDICES_FRONT, offset, &culled, duplicate_list);
                }

                if back_condition {
                    push_indices(&mut indices, INDICES_BACK, offset, &culled, duplicate_list);
                }

                // Lines up with the INITIAL indices, NOT the transformed ones
                offset += 8;
            }
        }

        layer_num += 1;
    }

    log::log(
        format!(
            "Mesh created and optimized in {:?}ms",
            std::time::Instant::now()
                .duration_since(pretime)
                .as_secs_f32()
                * 1000.0
        ),
        log::LogLevel::INFO,
    );

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

fn push_indices(
    vector: &mut Vec<u32>,
    indices: &[u32],
    index_offset: u32,
    culled: &Vec<usize>,
    duplicate_list: &Vec<i32>,
) {
    let offset = index_offset;
    let indices = indices
        .iter()
        .map(|i| match duplicate_list[*i as usize] {
            -1 => {
                let cull = culled
                    .iter()
                    .filter(|x| **x < (*i + offset) as usize)
                    .count() as u32;
                *i + offset as u32 - cull
            }
            a => {
                a as u32
            }
        })
        .collect::<Vec<_>>();

    vector.push(indices[0]);
    vector.push(indices[1]);
    vector.push(indices[2]);
    vector.push(indices[3]);
    vector.push(indices[4]);
    vector.push(indices[5]);
}

fn get_normal(normal: [f32; 3], adj1: bool, adj2: bool, vertical: bool) -> [f32; 3] {
    let mut normal = glam::Vec3::from_array(normal);
    if normal.x == 0.0 {
        normal.x = -1.0
    }
    if normal.y == 0.0 {
        normal.y = -1.0
    }
    if normal.z == 0.0 {
        normal.z = -1.0
    }

    // Vertices
    if adj1 && adj2 && !vertical {
        return normal.normalize().to_array();
    }

    // Edges
    if adj1 || adj2 {
        if adj1 {
            // Left/Right
            return glam::vec3(normal.x, normal.y, 0.0).normalize().to_array();
        } else if adj2 {
            // Forwards/Backwards
            return glam::vec3(0.0, normal.y, normal.z).normalize().to_array();
        } else if vertical {
            // Up/Down
            return glam::vec3(normal.x, 0.0, normal.z).normalize().to_array();
        }
    }

    // Internals
    return glam::vec3(0.0, normal.y, 0.0).normalize().to_array();
}
