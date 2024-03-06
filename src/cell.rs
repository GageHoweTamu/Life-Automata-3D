// defines the cells the organisms are made of
use rand::Rng;

pub struct Brain { // the brain is the core of the organism
    pub aggression: f32,
                            // 0.0: never attacks
                            // 0.5: attacks smaller organisms (default)
                            // 1.0: attacks everything
    pub hunger: f32,
                        // 0.0: never pursues food
                        // 0.5: pursues food if not in danger (default)
                        // 1.0: always pursues food
}
pub enum CellType {
    Eye,
    Armor,
    Damager,
    Eater,
    Producer,
}

pub struct Cell {
    cell_type: CellType,
    local_x: i8,
    local_y: i8,
    local_z: i8,
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
        let cell_type = match rng.gen_range(0..6) {
            0 => CellType::Body,
            1 => CellType::Eye,
            2 => CellType::Armor,
            3 => CellType::Damager,
            4 => CellType::Producer,
            _ => CellType::Eater,
        };
        self.cell_type = cell_type;
    }
    pub fn shift(&mut self, x: i8, y: i8, z: i8) {
        self.local_x += x;
        self.local_y += y;
        self.local_z += z;
    }
}