use std::{clone, f32::consts, ops::RangeBounds};
use rand::{random_range, seq::IndexedRandom};
    

#[derive(Clone, Copy)]
pub struct Insect{
    pos: [f32; 2],
    //0,1 are vector, 2 is angle of said vector
    direction: [f32; 3],
    steps_from_food: u32,
    steps_from_base: u32
}

impl Insect{
    fn new_random(limits: [u32; 4]) -> Insect{

        let mut direction: [f32; 3] = [0.0, 0.0, 0.0];
        let angle =  random_range(-consts::PI..consts::PI);

        direction[0] = angle.cos();
        direction[1] = angle.sin();
        direction[2] = angle;

        Insect { pos: [random_range(limits[0]..limits[1]) as f32, random_range(limits[2]..limits[3]) as f32], direction, steps_from_food: 0, steps_from_base: 0}
    }

    fn move_pos(&self, speed: f32, map_limits: [u32; 2]) -> Insect {
        let angle: f32 = random_range(-0.17..=0.17);

        let mut direction:[f32;3] = [0., 0., 0.];
        let mut pos:[f32;2] = [0., 0.];
        let map_limits = [map_limits[0] as f32, map_limits[1] as f32];

        direction[0] = (self.direction[2] + angle).cos();
        direction[1] = (self.direction[2] + angle).sin();
        direction[2] = self.direction[2] + angle;

        pos[0] += self.direction[0] * speed;
        pos[1] += self.direction[1] * speed;

        if pos[0] > map_limits[0]{
            pos[0] -= map_limits[0] 
        }
        if pos[0] < 0.{
            pos[0] += map_limits[0]
        }
        if pos[1] > map_limits[1]{
            pos[1] -= map_limits[1]
        }
        if pos[1] < 0.{
            pos[1] += map_limits[1]
        }

        let steps_from_base = self.steps_from_base + 1;
        let steps_from_food = self.steps_from_food + 1;

        Insect { pos, direction, steps_from_food, steps_from_base }


    }
}

struct FoodSource{
    pos: [u32; 2],
    size: u16
}
impl FoodSource {

    fn new_random(limits: [u32; 4], size: u16) -> FoodSource{
        FoodSource{pos: [random_range(limits[0]..limits[1]), random_range(limits[2]..limits[3])], size}
    }
    
}

struct Base{
    pos_x: u32,
    pos_y: u32,
    size: u16
}

impl Base {
    fn new_random(limits: [u32; 4], size: u16) -> Base{
        Base{pos_x: random_range(limits[0]..limits[1]), pos_y: random_range(limits[2]..limits[3]), size}
    }
    
}

struct Cell{
    insects: Vec<Insect>,
    bases: Vec<Base>,
    food_sources: Vec<FoodSource>,
    pos: [u32; 2]
}

impl Cell {

    fn new (insects: Vec<Insect>, bases: Vec<Base>, food_sources: Vec<FoodSource>, pos: [u32; 2]) -> Cell{
        Cell {insects, bases, food_sources, pos}
    }
    
}

struct Map{
    grid: Vec<Cell>,
    height: u32,
    width: u32,
    cell_size: u32,
}

impl Map {

    //width and height are mesured in cells
    pub fn new (cell_size: u32, size: [u32; 2], num_food: u16, num_base: u16, base_size: u16, num_insects: u32) -> Map{

        let mut base_cells: Vec<[u32; 2]> = Vec::new();
        let mut food_source_cells: Vec<[u32; 2]> = Vec::new();

        let num_cells =  size[0] * size[1];

        for _ in 0..=num_base {
            base_cells.push([random_range(0..=num_cells), random_range(0..=num_cells)]);

        }
        for _ in 0..=num_food{
            food_source_cells.push([random_range(0..=num_cells), random_range(0..=num_cells)]);
        }


        let mut cells = Vec::new();
        for cell_num in 0..= num_cells{
            
            let cell_x= cell_num%size[0];
            let cell_y= cell_num/size[0];

            let cell_bounds =[(cell_x - 1) * cell_size, cell_x * cell_size, (cell_y - 1) * cell_size, cell_y * cell_size];
            let mut insects: Vec<Insect> = Vec::new();
            let mut bases: Vec<Base> = Vec::new();
            let mut food_sources: Vec<FoodSource> = Vec::new();

            for _ in 0..=num_insects / num_cells {
                insects.push(Insect::new_random(cell_bounds));
            }

            for pos in &base_cells {
                if pos[0]+pos[1]*size[0] == cell_num{
                    bases.push(Base::new_random(cell_bounds, base_size));
                }
            }

            for pos in &food_source_cells{
                if pos[0]+pos[1]*size[0] == cell_num{
                    food_sources.push(FoodSource::new_random(cell_bounds, base_size));
                }
            }

            cells.push(Cell::new(insects, bases, food_sources, [cell_x, cell_y]));


        }
        Map { grid: cells, height: size[0], width: size[1], cell_size}

    }

    pub fn move_insects (self, speed: f32) -> Map{
        let mut off_grid_insects: Vec<Insect> = Vec::new();
        let mut cells: Vec<Cell> = Vec::new();

        for cell in self.grid {
            
            let mut insects: Vec<Insect> = Vec::new();
            let cell_bounds =[((cell.pos[0]-1) * self.cell_size) as f32 ..= (cell.pos[0] * self.cell_size) as f32 , ((cell.pos[1]- 1) * self.cell_size) as f32 ..= (cell.pos[1] * self.cell_size) as f32];

            for insect in &cell.insects{
                let moved_insect = insect.move_pos(speed, [self.width, self.height]);
                if cell_bounds[0].contains(&moved_insect.pos[0]) || cell_bounds[1].contains(&moved_insect.pos[1]){
                    insects.push(moved_insect);
                    continue;
                }
                off_grid_insects.push(moved_insect);
                continue;
            }

            cells.push(Cell::new(insects, cell.bases, cell.food_sources, cell.pos));
        }

        for i in 0..cells.len() {

            let cell = &cells[i];
            let cell_bounds =[((cell.pos[0]-1) * self.cell_size) as f32 ..= (cell.pos[0] * self.cell_size) as f32 , ((cell.pos[1]- 1) * self.cell_size) as f32 ..= (cell.pos[1] * self.cell_size) as f32];

            for insect in &off_grid_insects{
                if cell_bounds[0].contains(&insect.pos[0]) || cell_bounds[1].contains(&insect.pos[1]){
                    cells[i].insects.push(*insect);
                }
            }


        }

        Map { grid: cells, height: self.height, width: self.height, cell_size: self.cell_size}
    }

   pub fn collisions (self) -> Map{

   } 
    
}

