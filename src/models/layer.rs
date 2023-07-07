use super::voxel::Voxel;

#[derive(Debug, Default, Clone)]
pub struct Layer {
    pub label: String,
    pub value: Vec<Vec<Voxel>>,
}

impl PartialEq for Layer {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Layer {
    pub fn get_outer(&self) -> Layer {
        let value = &self.value;
        let mut out: Vec<Vec<Voxel>> = value.clone();
        let height = value.len() - 1;
        let width = value.get(0).expect("Zero width layer").len() - 1;
        for x in 0..height {
            for y in 0..width {
                if (x | y != 0) || x == width || y == height {
                    out[x][y] = value[x][y].filled();
                }

                out[x][y] = Voxel {
                    filled: match (x, y) {
                        (x, y) if !value[x+1][y].filled => false,
                        (x, y) if !value[x-1][y].filled => false,
                        (x, y) if !value[x][y+1].filled => false,
                        (x, y) if !value[x][y-1].filled => false,
                        _ => true
                    },
                    material: value[x][y].material,
                }
            }
        }

        Layer {
            label: self.label.clone(),
            value: out,
        }
    }
}
