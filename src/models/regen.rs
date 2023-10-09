use crate::{graphics::vertex::Vertex, models::regen_temp, utils, utils::*};

use super::model::Model;

pub fn gen_vert_idx(model: &Model) -> (Vec<Vertex>, Vec<u32>) {
    let pretime = std::time::Instant::now();
    let mut vertices = vec![];
    let mut indices = vec![];

    let mut layer_num = 0;
    let mut offset = 0;
    let mut culled = vec![];

    for layer in &model.value {
        // Sides
        for ux in 0..layer.value.len() {
            for uz in 0..layer.value[ux].len() {
                // If voxel is empty
                if !layer.value[ux][uz].filled {
                    continue;
                }

                let temp =
                    regen_temp::ModelGenTemp::new(model, ux, uz, layer_num, layer, &vertices);

                // Push vertices and duplicate list
                // The value of the index it's REPLACING must be appeneded to 'culled'
                let duplicate_list = &vec![
                    match temp.left_up_back_dup {
                        Some(v) => {
                            // Push vertex
                            utils::push_if_absent(&mut culled, (0 + offset) as usize);
                            // Push duplicate index
                            v as i32
                        }
                        None => {
                            // Push vertex
                            vertices.push(temp.left_up_back);
                            // Push duplicate index (-1 means no duplicate)
                            -1
                        }
                    },
                    match temp.left_up_front_dup {
                        Some(v) => {
                            utils::push_if_absent(&mut culled, (1 + offset) as usize);
                            v as i32
                        }
                        None => {
                            vertices.push(temp.left_up_front);
                            -1
                        }
                    },
                    match temp.right_up_front_dup {
                        Some(v) => {
                            utils::push_if_absent(&mut culled, (2 + offset) as usize);
                            v as i32
                        }
                        None => {
                            vertices.push(temp.right_up_front);
                            -1
                        }
                    },
                    match temp.right_up_back_dup {
                        Some(v) => {
                            utils::push_if_absent(&mut culled, (3 + offset) as usize);
                            v as i32
                        }
                        None => {
                            vertices.push(temp.right_up_back);
                            -1
                        }
                    },
                    match temp.left_down_back_dup {
                        Some(v) => {
                            utils::push_if_absent(&mut culled, (4 + offset) as usize);
                            v as i32
                        }
                        None => {
                            vertices.push(temp.left_down_back);
                            -1
                        }
                    },
                    match temp.left_down_front_dup {
                        Some(v) => {
                            utils::push_if_absent(&mut culled, (5 + offset) as usize);
                            v as i32
                        }
                        None => {
                            vertices.push(temp.left_down_front);
                            -1
                        }
                    },
                    match temp.right_down_front_dup {
                        Some(v) => {
                            utils::push_if_absent(&mut culled, (6 + offset) as usize);
                            v as i32
                        }
                        None => {
                            vertices.push(temp.right_down_front);
                            -1
                        }
                    },
                    match temp.right_down_back_dup {
                        Some(v) => {
                            utils::push_if_absent(&mut culled, (7 + offset) as usize);
                            v as i32
                        }
                        None => {
                            vertices.push(temp.right_down_back);
                            -1
                        }
                    },
                ];

                // Push culled indices
                use crate::utils::consts::*;

                if temp.top_condition {
                    push_indices(&mut indices, INDICES_TOP, offset, &culled, duplicate_list);
                }

                if temp.bottom_condition {
                    push_indices(
                        &mut indices,
                        INDICES_BOTTOM,
                        offset,
                        &culled,
                        duplicate_list,
                    );
                }

                if temp.right_condition {
                    push_indices(&mut indices, INDICES_RIGHT, offset, &culled, duplicate_list);
                }

                if temp.left_condition {
                    push_indices(&mut indices, INDICES_LEFT, offset, &culled, duplicate_list);
                }

                if temp.front_condition {
                    push_indices(&mut indices, INDICES_FRONT, offset, &culled, duplicate_list);
                }

                if temp.back_condition {
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
            a => a as u32,
        })
        .collect::<Vec<_>>();

    vector.push(indices[0]);
    vector.push(indices[1]);
    vector.push(indices[2]);
    vector.push(indices[3]);
    vector.push(indices[4]);
    vector.push(indices[5]);
}
