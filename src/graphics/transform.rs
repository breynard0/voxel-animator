use super::vertex;

pub fn translate(vertices: &Vec<vertex::Vertex>, translation: glam::Vec3) -> Vec<vertex::Vertex> {
    let mut output = vec![];

    for vertex in vertices {
        use glam::*;

        let mut vertex = vertex.clone();
        let transmat = mat4(
            vec4(1.0, 0.0, 0.0, 0.0),
            vec4(0.0, 1.0, 0.0, 0.0),
            vec4(0.0, 0.0, 1.0, 0.0),
            vec4(translation.x, translation.y, translation.z, 1.0),
        );
        let pos = [vertex.pos[0], vertex.pos[1], vertex.pos[2], 1.];
        let pos_out = transmat.mul_vec4(pos.into()).xyz().to_array();
        vertex.pos = pos_out;
        output.push(vertex);
    }

    output
}

pub fn rotate(vertices: &Vec<vertex::Vertex>, rotation: glam::Vec3) -> Vec<vertex::Vertex> {
    let mut output = vec![];
    let x = rotation.x;
    let y = rotation.y;
    let z = rotation.z;

    for vertex in vertices {
        use glam::*;

        let mut vertex = vertex.clone();
        let rotmatx = mat4(
            vec4(1.0, 0.0, 0.0, 0.0),
            vec4(0.0, x.cos(), x.sin(), 0.0),
            vec4(0.0, -x.sin(), x.cos(), 0.0),
            vec4(0.0, 0.0, 0.0, 1.0),
        );
        let rotmaty = mat4(
            vec4(y.cos(), 0.0, -y.sin(), 0.0),
            vec4(0.0, 1.0, 0.0, 0.0),
            vec4(y.sin(), 0.0, y.cos(), 0.0),
            vec4(0.0, 0.0, 0.0, 1.0),
        );
        let rotmatz = mat4(
            vec4(z.cos(), z.sin(), 0.0, 0.0),
            vec4(-z.sin(), z.cos(), 0.0, 0.0),
            vec4(0.0, 0.0, 1.0, 0.0),
            vec4(0.0, 0.0, 0.0, 1.0),
        );

        let rotmat = rotmatx * rotmaty * rotmatz;

        let pos = [vertex.pos[0], vertex.pos[1], vertex.pos[2], 1.];
        let pos_out = rotmat.mul_vec4(pos.into()).xyz().to_array();
        vertex.pos = pos_out;

        output.push(vertex);
    }

    output
}

pub fn scale(vertices: &Vec<vertex::Vertex>, scaling: glam::Vec3) -> Vec<vertex::Vertex> {
    let mut output = vec![];

    for vertex in vertices {
        use glam::*;

        let mut vertex = vertex.clone();
        let scalemat = mat4(
            vec4(scaling.x, 0.0, 0.0, 0.0),
            vec4(0.0, scaling.y, 0.0, 0.0),
            vec4(0.0, 0.0, scaling.z, 0.0),
            vec4(0.0, 0.0, 0.0, 1.0),
        );
        let pos = [vertex.pos[0], vertex.pos[1], vertex.pos[2], 1.];
        let pos_out = scalemat.mul_vec4(pos.into()).xyz().to_array();
        vertex.pos = pos_out;
        output.push(vertex);
    }

    output
}
