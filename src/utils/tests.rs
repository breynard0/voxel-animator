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
