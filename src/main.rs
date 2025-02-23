use std::fmt::Formatter;
use std::thread::sleep;
use std::time::Duration;

use flo_canvas::*;
use flo_draw::*;
use screaming_inscts::*;

const CELL_SIZE: u32 = 30;
const MAP_SIZE: [u32; 2] = [5, 5];
const NUM_FOOD: u16 = 1;
const NUM_BASE: u16 = 1;
const BASE_SIZE: u16 = 6;
const NUM_INSECTS: u32 = 025;
const INSECT_SPEED: f32 = 1.0;
const FRAME_DELAY_MILISEC: u64 = 00;

const MAP_SIZE_PXL: [u32; 2] = [MAP_SIZE[0] * CELL_SIZE, MAP_SIZE[1] * CELL_SIZE];

fn main() {
    with_2d_graphics(|| {
        let canvas = create_canvas_window("Hello, triangle");
        let mut map = screaming_inscts::Map::new(
            CELL_SIZE,
            MAP_SIZE,
            NUM_FOOD,
            NUM_BASE,
            BASE_SIZE,
            NUM_INSECTS,
        );
        loop {
            map = map.move_insects(INSECT_SPEED);
            map = map.collisions();
            map = map.screaming();
            map = map.settle_insects();
            let insects = map.get_insects();
            let bases = map.get_bases();
            let food_sources = map.get_food_sources();

            canvas.draw(|gc| {
                gc.clear_canvas(Color::Rgba(0.0, 0.4, 0.4, 1.0));
                gc.canvas_height(MAP_SIZE_PXL[0] as f32);
                gc.center_region(0.0, 00.0, MAP_SIZE_PXL[0] as f32, -(MAP_SIZE_PXL[1] as f32));
                gc.transform(Transform2D::rotate_degrees(-90.));

                for i in 0..=MAP_SIZE[1] {
                    gc.new_path();
                    gc.move_to(0., (i * CELL_SIZE) as f32);
                    gc.line_to(MAP_SIZE_PXL[0] as f32, (i * CELL_SIZE) as f32);
                    gc.line_width(1.);
                    gc.stroke_color(Color::Rgba(1., 1., 1., 0.4));
                    gc.stroke();
                }

                for i in 0..=10 {
                    gc.new_path();
                    gc.move_to((i * CELL_SIZE) as f32, 0.);
                    gc.line_to((i * CELL_SIZE) as f32, MAP_SIZE_PXL[0] as f32);
                    gc.line_width(1.);
                    gc.stroke_color(Color::Rgba(1., 1., 1., 0.4));
                    gc.stroke();
                }

                for i in insects {
                    gc.new_path();
                    gc.move_to(i.pos[0], i.pos[1]);

                    gc.circle(i.pos[0], i.pos[1], 0.5);
                    gc.line_to(
                        i.pos[0] + i.direction[0] * 3.,
                        i.pos[1] + i.direction[1] * 3.,
                    );

                    gc.fill_color(Color::Rgba(1.0, 0.8, 0.0, 1.0));
                    gc.stroke_color(Color::Rgba(1.0, 0.8, 0.0, 1.0));
                    gc.line_width(1.5);
                    gc.line_cap(LineCap::Round);

                    gc.stroke();
                    gc.fill();
                }

                for i in bases {
                    gc.new_path();

                    gc.circle(i.pos[0], i.pos[1], BASE_SIZE as f32);

                    gc.fill_color(Color::Rgba(0.1, 0.0, 0.1, 1.0));

                    gc.fill();
                }

                for i in food_sources {
                    gc.new_path();

                    gc.circle(i.pos[0], i.pos[1], BASE_SIZE as f32);

                    gc.fill_color(Color::Rgba(0.0, 1.0, 0.2, 1.0));

                    gc.fill();
                }
            });
            sleep(Duration::from_millis(FRAME_DELAY_MILISEC));
        }
    });
}
