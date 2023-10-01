use crate::graphics::vertex::Vertex;

pub mod consts;
pub mod tests;
pub mod log;

pub fn normalize_scale(vertices: &Vec<Vertex>, min: f32, max: f32) -> Vec<Vertex> {
    let mut max_x: f32 = 0.0;
    let mut min_x: f32 = 0.0;
    let mut max_y: f32 = 0.0;
    let mut min_y: f32 = 0.0;
    let mut max_z: f32 = 0.0;
    let mut min_z: f32 = 0.0;

    for vertex in vertices.iter() {
        if vertex.pos[0] > max_x {
            max_x = vertex.pos[0];
        }

        if vertex.pos[0] < min_x {
            min_x = vertex.pos[0];
        }

        if vertex.pos[1] > max_y {
            max_y = vertex.pos[1];
        }

        if vertex.pos[1] < min_y {
            min_y = vertex.pos[1];
        }

        if vertex.pos[2] > max_z {
            max_z = vertex.pos[2];
        }

        if vertex.pos[2] < min_z {
            min_z = vertex.pos[2];
        }
    }

    let largest = (max_x - min_x).max(max_y - min_y).max(max_z - min_z);
    // Print all variables then largest
    // println!("Max: {}, Min: {}, Largest: {}", max, min, largest);
    // // Print all other variables
    // println!(
    //     "Max X: {}, Min X: {}, Max Y: {}, Min Y: {}, Max Z: {}, Min Z: {}",
    //     max_x, min_x, max_y, min_y, max_z, min_z
    // );

    let mut out = vec![];
    for vertex in vertices.iter() {
        let mut v = vertex.clone();

        if largest == (max_x - min_x) {
            v.pos[0] = (max - min) / (max_x - min_x) * vertex.pos[0] + min;
            v.pos[1] = (max - min) / (max_x - min_x) * vertex.pos[1] + min;
            v.pos[2] = (max - min) / (max_x - min_x) * vertex.pos[2] + min;
        }

        if largest == (max_y - min_y) {
            v.pos[0] = (max - min) / (max_y - min_y) * vertex.pos[0] + min;
            v.pos[1] = (max - min) / (max_y - min_y) * vertex.pos[1] + min;
            v.pos[2] = (max - min) / (max_y - min_y) * vertex.pos[2] + min;
        }

        if largest == (max_z - min_z) {
            v.pos[0] = (max - min) / (max_z - min_z) * vertex.pos[0] + min;
            v.pos[1] = (max - min) / (max_z - min_z) * vertex.pos[1] + min;
            v.pos[2] = (max - min) / (max_z - min_z) * vertex.pos[2] + min;
        }

        out.push(v);
    }

    out
}
