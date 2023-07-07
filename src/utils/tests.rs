use crate::models::{layer::Layer, material::Material, voxel::Voxel};

#[test]
fn layer() {
    {
        let filled = Voxel {
            filled: true,
            material: Material::default(),
        };

        let empty = Voxel {
            filled: false,
            material: Material::default(),
        };

        // 6x8 layer with somewhat circular shape and some variations, completely filled in
        let layer = Layer {
            label: "testlayer1".to_string(),
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
        let layer2 = Layer {
            label: "testlayer2".to_string(),
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

        assert_eq!(layer.get_outer(), layer2);
    }
}

#[test]
fn log() {
    super::logging::log("info", super::logging::LogLevel::INFO);
    super::logging::log("warning", super::logging::LogLevel::WARNING);
    super::logging::log("error", super::logging::LogLevel::ERROR);
    super::logging::log("fatal", super::logging::LogLevel::FATAL);
    assert_eq!(
        super::logging::get_logs(),
        vec![
            (crate::utils::logging::LogLevel::INFO, "info".to_string()),
            (
                crate::utils::logging::LogLevel::WARNING,
                "warning".to_string()
            ),
            (crate::utils::logging::LogLevel::ERROR, "error".to_string()),
            (crate::utils::logging::LogLevel::FATAL, "fatal".to_string())
        ]
    );
}
