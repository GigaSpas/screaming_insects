use std::f32::consts::{self, PI};
use rand::{random_range, seq::IndexedRandom};
    

#[derive(Clone, Copy)]
pub struct Insect{
    pos: [f32; 2],
    //0,1 are vector, 2 is angle of said vector
    direction: [f32; 3],
    steps_from_food: u32,
    steps_from_base: u32,
    //false: searching for food source, true: searching for base
    fed: bool,
}

impl Insect{
    fn new_random(limits: [f32; 4]) -> Insect{

        let mut direction: [f32; 3] = [0.0, 0.0, 0.0];
        let angle =  random_range(-consts::PI..consts::PI);

        direction[0] = angle.cos();
        direction[1] = angle.sin();
        direction[2] = angle;

        Insect { pos: [random_range(limits[0]..limits[1]), random_range(limits[2]..limits[3])], direction, steps_from_food: 0, steps_from_base: 0,fed: false}
    }

    fn move_pos(&self, speed: f32, map_limits: [u32; 2]) -> Insect {
        let angle: f32 = random_range(-0.17..=0.17);

        let mut direction:[f32;3] = [0., 0., 0.];
        let mut pos:[f32;2] = [0., 0.];
        let map_limits = [map_limits[0] as f32, map_limits[1] as f32];

        direction[0] = (self.direction[2] + angle).cos();
        direction[1] = (self.direction[2] + angle).sin();
        direction[2] = self.direction[2] + angle;

        pos[0] += direction[0] * speed;
        pos[1] += direction[1] * speed;

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

        Insect { pos, direction, steps_from_food, steps_from_base, fed: self.fed}


    }
    // true: object is base, false: object is food_source
    fn collide(self, object: bool, object_size: f32, distance_to_object: f32) -> Insect{
        let angle: f32 = - PI;

        let mut direction:[f32;3] = [0., 0., 0.];
        let mut pos:[f32;2] = [0., 0.];

        direction[0] = (self.direction[2] + angle).cos();
        direction[1] = (self.direction[2] + angle).sin();
        direction[2] = self.direction[2] + angle;

        pos[0] += direction[0] * (object_size - distance_to_object + 1.);
        pos[1] += direction[1] * (object_size - distance_to_object + 1.);

        let mut steps_from_food = self.steps_from_food;
        let mut steps_from_base = self.steps_from_food;

        if object {
             steps_from_base = 0;
        }

        if !object {
             steps_from_food = 0;
        }

        Insect { pos, direction, steps_from_food, steps_from_base, fed: self.fed}
        
    }

}

#[derive(Clone, Copy)]
struct FoodSource{
    pos: [f32; 2],
    size: u16
}
impl FoodSource {

    fn new_random(limits: [f32; 4], size: u16) -> FoodSource{
        FoodSource{pos: [random_range(limits[0]/2.0..limits[1]/2.), random_range(limits[2]/2.0..limits[3]/2.)], size}
    }
    
}

#[derive(Clone, Copy)]
struct Base{
    pos: [f32; 2],
    size: u16
}

impl Base {
    fn new_random(limits: [f32; 4], size: u16) -> Base{
        Base{pos: [random_range(limits[0]/2.0..limits[1]/2.), random_range(limits[2]/2.0..limits[3]/2.)], size}
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

            let cell_bounds =[((cell_x - 1) * cell_size) as f32, (cell_x * cell_size) as f32, ((cell_y - 1) * cell_size) as f32, (cell_y * cell_size) as f32];
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
        let mut cells: Vec<Cell> = Vec::new();

        for cell in self.grid {
            
            let mut insects: Vec<Insect> = Vec::new();

            for insect in &cell.insects{
                let moved_insect = insect.move_pos(speed, [self.width, self.height]);
                    insects.push(moved_insect);
            }

            cells.push(Cell::new(insects, cell.bases, cell.food_sources, cell.pos));
        }

        Map { grid: cells, height: self.height, width: self.height, cell_size: self.cell_size}
    }


   pub fn collisions (self) -> Map{

       let mut cells: Vec<Cell> = Vec::new();

       for cell in &self.grid{

           let mut insects: Vec<Insect> = Vec::new();
           //let mut bases: Vec<Base> = Vec::new();
           //let mut food_sources: Vec<FoodSource> = Vec::new();

           if !cell.food_sources.is_empty(){ for food_source in &cell.food_sources{

                    for insect in &cell.insects{

                        let distance = ((insect.pos[0] - food_source.pos[0]).powi(2) + (insect.pos[1] - food_source.pos[1]).powi(2)).sqrt();

                        if  distance < food_source.size as f32 {
                            insects.push (insect.collide(false, food_source.size as f32, distance,));
                        }
                    }
                }
           }

           if !cell.bases.is_empty(){
                for base in &cell.bases{

                    for insect in &cell.insects{

                        let distance = ((insect.pos[0] - base.pos[0]).powi(2) + (insect.pos[1] - base.pos[1]).powi(2)).sqrt();

                        if  distance < base.size as f32 {
                            insects.push (insect.collide(false, base.size as f32, distance,));
                        }
                    }
                }
           }

           cells.push(Cell{insects, bases: cell.bases.clone(), food_sources: cell.food_sources.clone(), pos: cell.pos});
       }

       Map { grid: cells, height: self.height, width: self.width, cell_size: self.cell_size}
   } 

   pub fn settle_insects(mut self) -> Map {
       let mut insects = Vec::new();

       for i in 0..self.grid.len(){
           insects.append(&mut self.grid[i].insects);
       }

       for cell in &mut self.grid {
           for i in 0..insects.len(){
            let cell_bounds =[((cell.pos[0]-1) * self.cell_size) as f32 ..= (cell.pos[0] * self.cell_size) as f32 , ((cell.pos[1]- 1) * self.cell_size) as f32 ..= (cell.pos[1] * self.cell_size) as f32];

            if cell_bounds[0].contains(&insects[i].pos[0]) || cell_bounds[1].contains(&insects[i].pos[1]){
                cell.insects.push(insects[i]);
            }
           }
       }

       self
   }


   pub fn screaming(mut self) -> Map{

       for cell in 0..self.grid.len() {

           let cells_to_check = get_neightboring_cells(&self.grid[cell].pos, &[self.width, self.height]);
           let mut insects_to_check: Vec<Insect> = Vec::new();
           
           for i in cells_to_check {

                match i {
                    
                    Some(i) => {
                        insects_to_check.append(&mut self.grid[i].insects);
                    },
                    None => continue,
                }

                for insect in 0..insects_to_check.len(){
                    scream(insect, &mut insects_to_check, &(self.cell_size as f32));
                }

           }

       }
       
       self
   }
    
}

    fn scream (index: usize, insects_to_check: &mut Vec<Insect>, scream_size: &f32){
        let screamer = insects_to_check[index];

        for i in 0..insects_to_check.len(){

            let hearer = &mut insects_to_check[i];
            let distance = ((screamer.pos[0] - hearer.pos[0]).powi(2) + (screamer.pos[1] - hearer.pos[1]).powi(2)).sqrt();

            if &distance < scream_size{
                if screamer.steps_from_base < hearer.steps_from_base{
                    hearer.steps_from_base = screamer.steps_from_base;
                    if hearer.fed{
                        hearer.direction[2] = f32::atan2(hearer.pos[1] - screamer.pos[1], hearer.pos[0] - screamer.pos[0]);
                        hearer.direction[0] = hearer.direction[2].cos();
                        hearer.direction[1] = hearer.direction[2].sin();
                    }
                }

                if screamer.steps_from_food < hearer.steps_from_food{
                    hearer.steps_from_food = screamer.steps_from_food;
                    if !hearer.fed{
                        hearer.direction[2] = f32::atan2(hearer.pos[1] - screamer.pos[1], hearer.pos[0] - screamer.pos[0]);
                        hearer.direction[0] = hearer.direction[2].cos();
                        hearer.direction[1] = hearer.direction[2].sin();
                    }
                }
            }

        }
    }


   fn get_neightboring_cells( cell_pos: &[u32; 2], map_size: &[u32; 2]) -> [Option<usize>; 9]{

       let mut cells: [Option<usize>; 9] = [None; 9];
       cells[5] = Some((cell_pos[0] + map_size[0] * cell_pos[1]) as usize);
       let cell_pos = [cell_pos[0] as i32, cell_pos[1] as i32];
       let map_size = [map_size[0] as i32, map_size[1] as i32];


       for y in -1..=1{
           if cell_pos[1] + y >= 0 && cell_pos[1] + y <= map_size[1]{

               for x in -1..=1{
                   if cell_pos[0] + x >= 0 && cell_pos[0] + x <= map_size[0]{

                       let cell_index = (cell_pos[0] + x) + map_size[0] * (cell_pos[1] + y);
                       cells[((x+1) + map_size[0] + (y+1)) as usize] = Some(cell_index as usize);
                        
                   }
               }

           }
       }
       cells

   }
