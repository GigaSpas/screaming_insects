use std::{f32::consts, ops::RangeBounds};
use rand::random_range;

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

    fn move_pos(&self, speed: f32) -> Insect {
        let angle: f32 = random_range(-0.17..=0.17);

        let mut direction:[f32;3] = [0., 0., 0.];
        let mut pos:[f32;2] = [0., 0.];

        direction[0] = (self.direction[2] + angle).cos();
        direction[1] = (self.direction[2] + angle).sin();
        direction[2] = self.direction[2] + angle;

        pos[0] += self.direction[0] * speed;
        pos[1] += self.direction[1] * speed;

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

enum Object {
    Insect(Insect),
    FoodSource(FoodSource),
    Base(Base)
}

struct Cell{
    objects: Vec<Object>,
}

impl Cell {

    fn new (objects: Vec<Object>) -> Cell{
        Cell { objects }
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
            let mut objects: Vec<Object> = Vec::new();

            for _ in 0..=num_insects / num_cells {
                objects.push(Object::Insect(Insect::new_random(cell_bounds)));
            }

            for pos in &base_cells {
                if pos[0]+pos[1]*size[0] == cell_num{
                    objects.push(Object::Base(Base::new_random(cell_bounds, base_size)))
                }
            }

            for pos in &food_source_cells{
                if pos[0]+pos[1]*size[0] == cell_num{
                    objects.push(Object::FoodSource(FoodSource::new_random(cell_bounds, base_size)))
                }
            }

            cells.push(Cell::new(objects))


        }
        Map { grid: cells, height: size[0], width: size[1], cell_size}

    }

    pub fn move_insects (self, speed: f32) -> Map{
        let mut off_grid_insects: Vec<Insect> = Vec::new();
        let cells: Vec<Cell> = Vec::new();

        for cell in  &self.grid {
            
            let mut objects: Vec<Insect> = Vec::new();
            let cell_bounds =[((self.width-1) * self.cell_size) as f32 ..= (self.width * self.cell_size) as f32 , ((self.height - 1) * self.cell_size) as f32 ..= (self.height * self.cell_size) as f32];
            for object in &cell.objects {
                match object {
                    Object::Insect(prev)=> {
                        let insect = prev.move_pos(speed);
                        if cell_bounds[0].contains(&insect.pos[0]) || cell_bounds[1].contains(&insect.pos[1]){
                            objects.push(insect);
                            continue;
                        }
                        off_grid_insects.push(insect);
                        continue;
                    },
                    _ => {},
                }

            }
            
        }

        self

    }
    
}

