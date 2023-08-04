use glam::Vec4Swizzles;

use crate::graphics::vertex::Vertex;

#[test]
fn layer() {
    let filled = crate::models::voxel::Voxel {
        filled: true,
        material: crate::models::material::Material::default(),
    };

    let empty = crate::models::voxel::Voxel {
        filled: false,
        material: crate::models::material::Material::default(),
    };

    {
        // 6x8 layer with somewhat circular shape and some variations, completely filled in
        let layer = crate::models::layer::Layer {
            label: "testlayer1".to_string(),
            #[rustfmt::skip]
            value: vec![
                vec![empty, empty, filled, filled, filled, empty],
                vec![empty, filled, filled, filled, filled, empty],
                vec![filled, filled, filled, filled, filled, filled],
                vec![filled, filled, filled, filled, filled, filled],
                vec![empty, filled, filled, filled, filled, empty],
                vec![empty, filled, filled, filled, filled, empty],
                vec![empty, empty, filled, filled, empty, empty],
                vec![empty, empty, filled, empty, empty, empty],
            ],
        };

        // Same as last layer, but only outside vertices filled
        let layer2 = crate::models::layer::Layer {
            label: "testlayer2".to_string(),
            #[rustfmt::skip]
            value: vec![
                vec![empty, empty, filled, filled, filled, empty],
                vec![empty, filled, empty, empty, filled, empty],
                vec![filled, empty, empty, empty, empty, filled],
                vec![filled, empty, empty, empty, empty, filled],
                vec![empty, filled, empty, empty, filled, empty],
                vec![empty, filled, empty, empty, filled, empty],
                vec![empty, empty, filled, filled, empty, empty],
                vec![empty, empty, filled, empty, empty, empty],
            ],
        };

        println!("{:?}", layer);
        assert_eq!(layer2, layer.get_outer());
    }

    {
        let layer = crate::models::layer::Layer {
            label: "testlayer3".to_string(),
            #[rustfmt::skip]
            value: vec![
                vec![filled, filled, empty, empty, empty, empty, empty, empty, empty],
                vec![filled, filled, filled, filled, filled, empty, filled, filled, empty],
                vec![empty, filled, filled, filled, filled, filled, filled, filled, filled],
                vec![empty, filled, filled, filled, filled, filled, filled, filled, empty],
                vec![empty, empty, filled, filled, filled, filled, filled, empty, empty],
                vec![empty, empty, empty, filled, filled, filled, empty, empty, empty],
                vec![empty, empty, empty, filled, empty, empty, empty, empty, empty],
                vec![empty, empty, empty, empty, empty, empty, filled, empty, empty],
                vec![empty, empty, filled, filled, empty, filled, filled, filled, empty],
                vec![empty, empty, empty, empty, empty, empty, filled, empty, empty],
            ],
        };

        let layer2 = crate::models::layer::Layer {
            label: "testlayer4".to_string(),
            #[rustfmt::skip]
            value: vec![
                vec![filled, filled, empty, empty, empty, empty, empty, empty, empty],
                vec![filled, empty, filled, filled, filled, empty, filled, filled, empty],
                vec![empty, filled, empty, empty, empty, filled, empty, empty, filled],
                vec![empty, filled, empty, empty, empty, empty, empty, filled, empty],
                vec![empty, empty, filled, empty, empty, empty, filled, empty, empty],
                vec![empty, empty, empty, filled, filled, filled, empty, empty, empty],
                vec![empty, empty, empty, filled, empty, empty, empty, empty, empty],
                vec![empty, empty, empty, empty, empty, empty, filled, empty, empty],
                vec![empty, empty, filled, filled, empty, filled, empty, filled, empty],
                vec![empty, empty, empty, empty, empty, empty, filled, empty, empty],
            ],
        };

        println!("{:?}", layer);
        assert_eq!(layer2, layer.get_outer());
    }
    {
        let layer = crate::models::layer::Layer {
            label: "testlayer5".to_string(),
            #[rustfmt::skip]
            value: vec![
                vec![empty, empty, filled, filled, filled, filled, filled, filled, empty, empty, empty, filled],
                vec![empty, empty, filled, filled, empty, empty, empty, filled, empty, filled, filled, empty],
                vec![filled, filled, empty, empty, empty, filled, filled, empty, filled, empty, empty, filled],
                vec![filled, filled, filled, filled, empty, filled, empty, filled, filled, empty, filled, empty],
                vec![filled, filled, empty, empty, filled, filled, filled, filled, empty, empty, empty, filled],
                vec![filled, empty, empty, empty, empty, empty, filled, filled, filled, filled, empty, empty],
                vec![empty, filled, empty, empty, empty, empty, filled, filled, filled, filled, filled, empty],
                vec![empty, filled, filled, empty, filled, empty, filled, filled, empty, empty, empty, filled],
                vec![empty, empty, filled, empty, empty, empty, filled, empty, filled, empty, filled, filled],
                vec![empty, empty, empty, filled, empty, empty, empty, empty, empty, empty, empty, empty],
                vec![filled, empty, empty, filled, filled, filled, empty, filled, filled, empty, empty, filled],
                vec![filled, filled, empty, empty, empty, empty, filled, filled, empty, filled, empty, filled],
                vec![empty, filled, filled, empty, filled, filled, filled, filled, empty, empty, empty, filled],
                vec![filled, filled, empty, filled, filled, filled, empty, filled, filled, filled, filled, empty],
                vec![empty, filled, empty, empty, filled, filled, empty, empty, filled, empty, filled, filled],
                vec![empty, empty, filled, filled, empty, empty, empty, empty, empty, filled, filled, empty],
                vec![filled, empty, empty, filled, empty, filled, filled, empty, empty, filled, empty, empty],
                vec![filled, filled, empty, empty, filled, empty, filled, filled, empty, empty, empty, filled],
                vec![empty, filled, filled, filled, filled, filled, empty, empty, filled, empty, empty, empty],
                vec![filled, empty, empty, filled, filled, filled, empty, empty, filled, filled, filled, empty],
                vec![filled, empty, empty, empty, empty, empty, empty, filled, filled, empty, empty, filled],
                vec![empty, filled, empty, filled, filled, filled, filled, filled, filled, empty, empty, empty],
                vec![filled, filled, filled, empty, filled, empty, empty, empty, filled, filled, filled, empty],
                vec![empty, empty, filled, filled, filled, empty, empty, empty, empty, empty, filled, empty],
                vec![filled, filled, filled, filled, filled, filled, filled, filled, filled, empty, filled, empty],
            ],
        };

        let layer2 = crate::models::layer::Layer {
            label: "testlayer6".to_string(),
            #[rustfmt::skip]
            value: vec![
                vec![empty, empty, filled, filled, filled, filled, filled, filled, empty, empty, empty, filled],
                vec![empty, empty, filled, filled, empty, empty, empty, filled, empty, filled, filled, empty],
                vec![filled, filled, empty, empty, empty, filled, filled, empty, filled, empty, empty, filled],
                vec![filled, empty, filled, filled, empty, filled, empty, filled, filled, empty, filled, empty],
                vec![filled, filled, empty, empty, filled, filled, filled, filled, empty, empty, empty, filled],
                vec![filled, empty, empty, empty, empty, empty, filled, empty, filled, filled, empty, empty],
                vec![empty, filled, empty, empty, empty, empty, filled, empty, filled, filled, filled, empty],
                vec![empty, filled, filled, empty, filled, empty, filled, filled, empty, empty, empty, filled],
                vec![empty, empty, filled, empty, empty, empty, filled, empty, filled, empty, filled, filled],
                vec![empty, empty, empty, filled, empty, empty, empty, empty, empty, empty, empty, empty],
                vec![filled, empty, empty, filled, filled, filled, empty, filled, filled, empty, empty, filled],
                vec![filled, filled, empty, empty, empty, empty, filled, filled, empty, filled, empty, filled],
                vec![empty, filled, filled, empty, filled, filled, filled, filled, empty, empty, empty, filled],
                vec![filled, filled, empty, filled, empty, filled, empty, filled, filled, filled, filled, empty],
                vec![empty, filled, empty, empty, filled, filled, empty, empty, filled, empty, filled, filled],
                vec![empty, empty, filled, filled, empty, empty, empty, empty, empty, filled, filled, empty],
                vec![filled, empty, empty, filled, empty, filled, filled, empty, empty, filled, empty, empty],
                vec![filled, filled, empty, empty, filled, empty, filled, filled, empty, empty, empty, filled],
                vec![empty, filled, filled, filled, empty, filled, empty, empty, filled, empty, empty, empty],
                vec![filled, empty, empty, filled, filled, filled, empty, empty, filled, filled, filled, empty],
                vec![filled, empty, empty, empty, empty, empty, empty, filled, filled, empty, empty, filled],
                vec![empty, filled, empty, filled, filled, filled, filled, filled, filled, empty, empty, empty],
                vec![filled, filled, filled, empty, filled, empty, empty, empty, filled, filled, filled, empty],
                vec![empty, empty, filled, filled, filled, empty, empty, empty, empty, empty, filled, empty],
                vec![filled, filled, filled, filled, filled, filled, filled, filled, filled, empty, filled, empty],
            ],
        };

        println!("{:?}", layer);
        assert_eq!(layer2, layer.get_outer());
    }
}

#[test]
fn matrix_application_translation() {
    let mut vertex = Vertex {
        pos: [3., 7., 1.],
        color: [1., 1., 1., 1.],
    };
    let matrix = glam::mat4(
        glam::vec4(1., 0., 0., 0.),
        glam::vec4(0., 1., 0., 0.),
        glam::vec4(0., 0., 1., 0.),
        glam::vec4(3., 4., 1., 0.),
    );

    let pos = [vertex.pos[0], vertex.pos[1], vertex.pos[2], 1.];
    let out = matrix.mul_vec4(pos.into());

    assert_eq!(out.xyz().to_array(), [6., 11., 2.])
}

#[test]
fn matrix_application_scaling() {
    let mut vertex = Vertex {
        pos: [3., 7., 1.],
        color: [1., 1., 1., 1.],
    };
    let matrix = glam::mat4(
        glam::vec4(6., 0., 0., 0.),
        glam::vec4(0., 2., 0., 0.),
        glam::vec4(0., 0., 4., 0.),
        glam::vec4(0., 0., 0., 0.),
    );

    let pos = [vertex.pos[0], vertex.pos[1], vertex.pos[2], 1.];
    let out = matrix.mul_vec4(pos.into());

    assert_eq!(out.xyz().to_array(), [18., 14., 4.])
}

#[test]
fn matrix_application_rotation() {
    let vertex = Vertex {
        pos: [3., 7., 1.],
        color: [1., 1., 1., 1.],
    };
    let angle: f32 = 30.0;
    let matrix = glam::mat4(
        glam::vec4(1.0, 0.0, 0.0, 0.0),
        glam::vec4(0.0, angle.cos(), angle.sin(), 0.0),
        glam::vec4(0.0, -angle.sin(), angle.cos(), 0.0),
        glam::vec4(0.0, 0.0, 0.0, 1.0),
    );

    let pos = [vertex.pos[0], vertex.pos[1], vertex.pos[2], 1.];
    let out = matrix.mul_vec4(pos.into());

    println!("{:?}", out)
}
