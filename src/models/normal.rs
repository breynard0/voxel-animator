pub fn get_normal(normal: [f32; 3], adj1: bool, adj2: bool, vertical: bool) -> [f32; 3] {
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
