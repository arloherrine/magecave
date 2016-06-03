extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use piston::input::keyboard::{ ModifierKey, SHIFT, NO_MODIFIER };
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use opengl_graphics::glyph_cache::GlyphCache;
use std::path::Path;

const WORLD_X: u32 = 30;
const WORLD_Y: u32 = 30;
const WORLD_Z: u32 = 10;

const WORLD_DIMENSIONS: [u32; 3] = [WORLD_X, WORLD_Y, WORLD_Z];

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    tiles: [[[Tile; WORLD_X as usize]; WORLD_Y as usize]; WORLD_Z as usize],
    viewport: [u32; 3],
    screen_size_pixels: [u32; 3],
    screen_size_tiles: [u32; 3]
}

#[derive(Copy, Clone)]
pub struct Tile {
    discovered: bool,
    floor: Floor,
    wall: Wall
}

#[derive(Copy, Clone)]
pub enum Floor {
    Open,
    Stair(Material),
    Floor(Material)
}

#[derive(Copy, Clone)]
pub enum Wall {
    Open,
    Ramp(Material),
    Stair(Material),
    Wall(Material)
}

#[derive(Copy, Clone)]
pub enum Material {
    Stone,
    Wood
}

impl Tile {
    fn new_solid() -> Tile {
        Tile {
            discovered: false,
            floor: Floor::Floor(Material::Stone),
            wall: Wall::Wall(Material::Stone)
        }
    }

    fn new_wall() -> Tile {
        Tile {
            discovered: true,
            floor: Floor::Floor(Material::Stone),
            wall: Wall::Wall(Material::Stone)
        }
    }

    fn new_floor() -> Tile {
        Tile {
            discovered: true,
            floor: Floor::Floor(Material::Stone),
            wall: Wall::Open
        }
    }
}

impl App {
    fn render(&mut self, args: &RenderArgs, cache: &mut GlyphCache) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let tile_width = self.screen_size_pixels[0] / self.screen_size_tiles[0];
        let tile_height = self.screen_size_pixels[1] / self.screen_size_tiles[1];
        let num_x_tiles = self.screen_size_tiles[0];
        let num_y_tiles = self.screen_size_tiles[1];
        let view_x = self.viewport[0] as usize;
        let view_y = self.viewport[1] as usize;
        let view_z = self.viewport[2] as usize;
        let tiles = self.tiles;
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            for i in 0..num_x_tiles {
                for j in 0..num_y_tiles {
                    let tile = tiles[view_z]
                            [view_y + j as usize]
                            [view_x + i as usize]; 
                    let square = rectangle::square(0.0, 0.0, 50.0);
                    let transform = c.transform.trans((i * tile_width) as f64, 
                                                      (j * tile_height) as f64)
                                               .trans(-25.0, -25.0);

                    if !tile.discovered {
                        rectangle(BLACK, square, transform, gl);
                    } else {
                        match tile.wall {
                            Wall::Ramp(_) => rectangle(RED, square, transform, gl),
                            Wall::Stair(_) => rectangle(RED, square, transform, gl),
                            Wall::Wall(_) => rectangle(RED, square, transform, gl),
                            Wall::Open => match tile.floor {
                                Floor::Stair(_) => rectangle(GREEN, square, transform, gl),
                                Floor::Floor(_) => rectangle(GREEN, square, transform, gl),
                                Floor::Open => (),
                            },
                        }
                    }
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
    }

    fn key_pressed(&mut self, key: &Key, mod_key: &ModifierKey) {
        match *key {
            Key::Left => self.decrement_viewport(0, 1),
            Key::Right => self.increment_viewport(0, 1),
            Key::Down => self.decrement_viewport(1, 1),
            Key::Up => self.increment_viewport(1, 1),
            Key::Comma => if mod_key.contains(SHIFT) { self.decrement_viewport(2, 1) },
            Key::Period => if mod_key.contains(SHIFT) { self.increment_viewport(2, 1) },
            _ => ()
        }
    }

    fn increment_viewport(&mut self, dimension: usize, diff: u32) {
        if self.viewport[dimension] <
            WORLD_DIMENSIONS[dimension] - self.screen_size_tiles[dimension] {
            self.viewport[dimension] += diff;
        }
    }

    fn decrement_viewport(&mut self, dimension: usize, diff: u32) {
        if self.viewport[dimension] != 0 {
            self.viewport[dimension] -= diff;
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create tile array
    let tiles = [
        [
            [Tile::new_solid(); 30],
            [Tile::new_solid(); 30],
            [Tile::new_solid(); 30],
            [Tile::new_solid(); 30],
            [Tile::new_solid(); 30],
            [Tile::new_solid(); 30],
            [Tile::new_solid(); 30],
            [Tile::new_solid(); 30],
            [Tile::new_solid(); 30],
            [Tile::new_solid(); 30],
            [
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
            ],
            [
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_wall(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_wall(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
            ],
            [
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_wall(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_wall(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
            ],
            [
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_wall(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_wall(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
            ],
            [
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_wall(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_floor(),
                Tile::new_wall(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
            ],
            [
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_wall(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
                Tile::new_solid(),
            ],
            [Tile::new_solid(); 30],
            [Tile::new_solid(); 30],
            [Tile::new_solid(); 30],
            [Tile::new_solid(); 30],
            [Tile::new_solid(); 30],
            [Tile::new_solid(); 30],
            [Tile::new_solid(); 30],
            [Tile::new_solid(); 30],
            [Tile::new_solid(); 30],
            [Tile::new_solid(); 30],
            [Tile::new_solid(); 30],
            [Tile::new_solid(); 30],
            [Tile::new_solid(); 30],
            [Tile::new_solid(); 30]
        ],
        [[Tile::new_solid(); 30]; 30],
        [[Tile::new_solid(); 30]; 30],
        [[Tile::new_solid(); 30]; 30],
        [[Tile::new_solid(); 30]; 30],
        [[Tile::new_solid(); 30]; 30],
        [[Tile::new_solid(); 30]; 30],
        [[Tile::new_solid(); 30]; 30],
        [[Tile::new_solid(); 30]; 30],
        [[Tile::new_solid(); 30]; 30]
    ];

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "Mage Cave",
            [600,400]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        tiles: tiles,
        viewport: [0, 0, 0],
        screen_size_pixels: [600, 400, 1],
        screen_size_tiles: [30, 20, 1]
    };
    
    let font_path = Path::new("./assets/NotoSans/NotoSans-Regular.ttf");
    let mut glyph_cache = GlyphCache::new(&font_path).unwrap();

    let mut events = window.events();
    let mut mod_keys = NO_MODIFIER;
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r, &mut glyph_cache);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
 
        if let Some(Button::Keyboard(key)) = e.press_args() {
            mod_keys.event(&e);
            app.key_pressed(&key, &mod_keys);
        }
    }
}
