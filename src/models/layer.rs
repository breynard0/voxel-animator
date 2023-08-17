use std::fmt::Debug;

use super::voxel::Voxel;

#[derive(Default, Clone)]
pub struct Layer {
    pub label: &'static str,
    pub value: Vec<Vec<Voxel>>,
}

impl Layer {
    pub fn get_outer(&self) -> Layer {
        let value = &self.value;
        let mut out: Vec<Vec<Voxel>> = value.clone();
        let height = value.len() - 1;
        let width = value.get(0).expect("Zero width layer").len() - 1;
        for a in 0..height + 1 {
            for b in 0..width + 1 {
                if a == 0 || b == 0 || b == width || a == height {
                    out[a][b] = value[a][b];
                    continue;
                }

                out[a][b] = Voxel {
                    filled: match value[a][b].filled {
                        true => match (a, b) {
                            (a, b) if !value[a + 1][b].filled => true,
                            (a, b) if !value[a - 1][b].filled => true,
                            (a, b) if !value[a][b + 1].filled => true,
                            (a, b) if !value[a][b - 1].filled => true,
                            _ => false,
                        },
                        false => false,
                    },
                    material: value[a][b].material,
                };
            }
        }

        Layer {
            label: self.label.clone(),
            value: out,
        }
    }
}

impl PartialEq for Layer {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Debug for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = "\n\n".to_string();

        for row in &self.value {
            for voxel in row {
                s.push_str(format!("{} ", voxel.filled as i32).as_str())
            }
            s.push('\n')
        }

        f.write_str(s.as_str())
    }
}
