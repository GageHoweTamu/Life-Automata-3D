// defines the cells the organisms are made of
use rand::Rng;

pub struct Brain {
    pub aggression: f32,    // How likely the organism is to attack
                            // 0.0: never attacks
                            // 0.5: attacks smaller organisms (default)
                            // 1.0: attacks everything
    pub hunger: f32, // How likely the organism is to pursue food in spite of danger
                        // 0.0: never pursues food
                        // 0.5: pursues food if not in danger (default)
                        // 1.0: always pursues food
}
pub enum CellType {
    Brain(Brain), // The brain cell is the first cell in the organism, and cannot be removed
    Eye,
    Armor,
    Damager,
    Eater,
    Producer,
}

pub struct Cell {
    cell_type: CellType,
    pub local_x: i8,
    pub local_y: i8,
    pub local_z: i8,
}

impl Cell {
    pub fn new(cell_type: CellType, local_x: i8, local_y: i8, local_z: i8) -> Cell {
        Cell {
            cell_type,
            local_x,
            local_y,
            local_z,
        }
    }
    pub fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        match &mut self.cell_type {
            CellType::Brain(x) => {
                x.aggression = (x.aggression + rng.gen_range(-0.2..0.2)).clamp(0.0, 1.0);
                x.hunger = (x.hunger + rng.gen_range(-0.2..0.2)).clamp(0.0, 1.0);
            }
            _ => {
                let mutated_type = match rng.gen_range(0..6) {
                    1 => CellType::Eye,
                    2 => CellType::Armor,
                    3 => CellType::Damager,
                    4 => CellType::Producer,
                    _ => CellType::Eater,
                };
                self.cell_type = mutated_type;
            }
        }
    }
    pub fn shift(&mut self, x: i8, y: i8, z: i8) {
        self.local_x += x;
        self.local_y += y;
        self.local_z += z;
    }
}