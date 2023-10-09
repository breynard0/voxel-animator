use crate::graphics::vertex::Vertex;

use super::{layer::Layer, model::Model, normal::get_normal};

pub struct ModelGenTemp {
    pub top_condition: bool,
    pub bottom_condition: bool,
    pub right_condition: bool,
    pub left_condition: bool,
    pub front_condition: bool,
    pub back_condition: bool,
    pub left_up_back: Vertex,
    pub left_up_front: Vertex,
    pub right_up_front: Vertex,
    pub right_up_back: Vertex,
    pub left_down_back: Vertex,
    pub left_down_front: Vertex,
    pub right_down_front: Vertex,
    pub right_down_back: Vertex,
    pub left_up_back_dup: Option<usize>,
    pub left_up_front_dup: Option<usize>,
    pub right_up_front_dup: Option<usize>,
    pub right_up_back_dup: Option<usize>,
    pub left_down_back_dup: Option<usize>,
    pub left_down_front_dup: Option<usize>,
    pub right_down_front_dup: Option<usize>,
    pub right_down_back_dup: Option<usize>,
}

impl ModelGenTemp {
    pub fn new(
        model: &Model,
        ux: usize,
        uz: usize,
        layer_num: i32,
        layer: &Layer,
        vertices: &Vec<Vertex>,
    ) -> Self {
        let x = ux as f32;
        let y = layer_num as f32;
        let z = uz as f32;

        let top_condition = !is_filled_at_offset(&model.value, x, z, layer_num, 0, 0, 1);
        let bottom_condition = !is_filled_at_offset(&model.value, x, z, layer_num, 0, 0, -1);
        let right_condition = !is_filled_at_offset(&model.value, x, z, layer_num, 1, 0, 0);
        let left_condition = !is_filled_at_offset(&model.value, x, z, layer_num, -1, 0, 0);
        let front_condition = !is_filled_at_offset(&model.value, x, z, layer_num, 0, 1, 0);
        let back_condition = !is_filled_at_offset(&model.value, x, z, layer_num, 0, -1, 0);

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

        Self {
            top_condition,
            bottom_condition,
            right_condition,
            left_condition,
            front_condition,
            back_condition,
            left_up_back,
            left_up_front,
            right_up_front,
            right_up_back,
            left_down_back,
            left_down_front,
            right_down_front,
            right_down_back,
            left_up_back_dup,
            left_up_front_dup,
            right_up_front_dup,
            right_up_back_dup,
            left_down_back_dup,
            left_down_front_dup,
            right_down_front_dup,
            right_down_back_dup,
        }
    }
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
