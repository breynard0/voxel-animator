use crate::graphics::vertex::Vertex;

pub mod consts;
pub mod log;
pub mod tests;

pub fn push_if_absent<T>(vector: &mut Vec<T>, element: T)
where
    T: PartialEq,
{
    if !vector.contains(&element) {
        vector.push(element);
    }
}

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

pub fn gv3_to_cgv3(input: glam::Vec3) -> cgmath::Vector3<f32> {
    cgmath::Vector3 {
        x: input.x,
        y: input.y,
        z: input.z,
    }
}

pub fn cgv3_to_gv3(input: cgmath::Vector3<f32>) -> glam::Vec3 {
    glam::Vec3 {
        x: input.x,
        y: input.y,
        z: input.z,
    }
}

pub fn uniform_buffer_to_bytes<U>(uniform: U) -> Vec<u8>
where
    U: encase::ShaderType + encase::internal::WriteInto,
{
    let mut buffer = encase::UniformBuffer::new(Vec::new());
    buffer.write(&uniform).unwrap();
    let byte_buffer = buffer.into_inner();
    byte_buffer
}

pub fn empty_buffer(device: &wgpu::Device) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("undefined buffer"),
        size: 0,
        usage: wgpu::BufferUsages::STORAGE,
        mapped_at_creation: false,
    })
}
