// mmio.rs
// for io operations iteracted with using mmio
use std::{collections::HashMap, process::exit, time::Duration};

use sdl2::{
    self,
    event::Event,
    keyboard::{Keycode, Mod},
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureQuery},
    surface::Surface,
    ttf::{Font, Sdl2TtfContext},
    video::Window,
    EventPump, Sdl, VideoSubsystem,
};

use crate::constant;
// const PATH: &str = "/home/kyle/CodeSync/rust/nimcode/sdltest/Glass_TTY_VT220.ttf";
struct Display {
    sdl_context: Sdl,
    // ttf_context: Sdl2TtfContext,
    video_subsystem: VideoSubsystem,
    pub canvas: Canvas<Window>,
}
impl Display {
    fn new(title: &str, dimensions: (u32, u32)) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window(title, dimensions.0, dimensions.1)
            .position_centered()
            .hidden()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        // let ttf_context = sdl2::ttf::init().unwrap();
        Ok(Display {
            sdl_context,
            // ttf_context,
            video_subsystem,
            canvas,
        })
    }
}

// struct FontManager<'a> {
//     surface_cache: HashMap<char, Surface<'a>>,
// }
// impl FontManager {}

struct TextModeDisplay {
    display: Display,
    columns: u32,
    rows: u32,

    ttf_context: Sdl2TtfContext,
    font_size: u16,
    font_path: String,
    cell_height: u32,
    cell_width: u32,
    key_stack: Vec<u8>, // each read of the key MMIO address pops from this stack
    event_pump: EventPump,
}
impl TextModeDisplay {
    fn new(
        title: &str,
        columns: u32,
        rows: u32,
        screen_width: u32,
        screen_height: u32,
        font_path: &str,
    ) -> Result<Self, String> {
        // let screen_width = 400;
        // let screen_height = 300;
        let cell_width = screen_width / columns;
        let cell_height = screen_height / rows;
        let font_size = (cell_height as f32 * 0.8) as u16;

        let display = Display::new(title, (screen_width, screen_height))?;
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
        let event_pump = display.sdl_context.event_pump()?;
        // let font = ttf_context
        //     .load_font(PATH, font_size)
        //     .map_err(|e| e.to_string())?;
        Ok(TextModeDisplay {
            display,
            columns,
            rows,
            cell_height,
            cell_width,
            ttf_context,
            font_path: font_path.to_string(),
            font_size: 20,
            key_stack: vec![],
            event_pump,
        })
    }
    fn load_font(&self) -> Result<Font<'_, 'static>, String> {
        let font = self
            .ttf_context
            .load_font(self.font_path.as_str(), self.font_size)
            .map_err(|e| e.to_string())?;
        Ok(font)
    }
    fn map_grid_coord(&self, cursor: (u32, u32)) -> (u32, u32) {
        let x = cursor.0 * self.cell_width;
        let y = cursor.1 * self.cell_height;
        (x, y)
    }
    /// writes a character to a grid position
    pub fn write(&mut self, ascii_code: u8, cursor: (u32, u32)) -> Result<(), String> {
        let abs_coord = self.map_grid_coord(cursor);
        // println!("write :: '{}' to {abs_coord:?}", ascii_code as char);
        let surface = self.ascii_to_surface(ascii_code)?;
        let texture_creator = self.display.canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(surface)
            .map_err(|e| e.to_string())?;
        let TextureQuery { width, height, .. } = texture.query();
        let target = Rect::new(abs_coord.0 as i32, abs_coord.1 as i32, width, height);
        self.display.canvas.copy(&texture, None, Some(target))?;
        Ok(())
    }
    fn ascii_to_surface(&self, ascii_code: u8) -> Result<Surface, String> {
        let font = self.load_font()?;
        // let font = self.font.as_ref();
        let surface = font
            .render(&(ascii_code as char).to_string())
            .blended(Color::RGB(255, 255, 255))
            .map_err(|e| e.to_string())?;
        Ok(surface)
    }
    /// listens for key presses and returns a translated real value.
    pub fn key_processor(&mut self) {
        for event in self.event_pump.poll_iter() {
            let (keycode, keymod) = match event {
                Event::KeyDown {
                    timestamp,
                    window_id,
                    keycode,
                    scancode,
                    keymod,
                    repeat,
                } => (keycode.unwrap(), keymod),
                Event::Quit { .. } => exit(0),
                _ => continue,
            };
            let key = match keycode {
                Keycode::Backspace => {
                    println!("backspace");
                    '\x08'
                }
                Keycode::Return => {
                    println!("return");
                    '\n'
                }
                Keycode::LShift => continue,
                Keycode::RShift => continue,
                Keycode::Tab => '\t',
                Keycode::Space => ' ',
                Keycode::Left => {
                    println!("left");
                    128 as char
                }
                Keycode::Right => {
                    println!("right");
                    129 as char
                }
                Keycode::Up => {
                    println!("up");
                    130 as char
                }
                Keycode::Down => {
                    println!("down");
                    131 as char
                }
                _ => keycode
                    .to_string()
                    .chars()
                    .next()
                    .unwrap()
                    .to_ascii_lowercase(),
                // Keycode::Space => ' ',
            };
            // handle mod
            let modded_key = match keymod {
                Mod::LSHIFTMOD => {
                    println!("{keycode} upper to {}", key.to_ascii_uppercase());
                    match key {
                        '/' => '?',
                        ',' => '<',
                        '.' => '>',
                        ';' => ':',
                        '\'' => '"',
                        '[' => '{',
                        ']' => '}',
                        '-' => '_',
                        '=' => '+',
                        '\\' => '|',
                        '`' => '~',

                        _ => {
                            if !key.is_ascii_digit() {
                                key.to_ascii_uppercase()
                            } else {
                                match key {
                                    '1' => '!',
                                    '2' => '@',
                                    '3' => '#',
                                    '4' => '$',
                                    '5' => '%',
                                    '6' => '^',
                                    '7' => '&',
                                    '8' => '*',
                                    '9' => '(',
                                    '0' => ')',
                                    _ => continue,
                                }
                            }
                        }
                    }
                }
                Mod::NOMOD => key,
                _ => {
                    println!("idk man. {keymod:?}");
                    key
                }
            };
            println!("detected {modded_key} :: {}", modded_key as u8);
            self.key_stack.push(modded_key as u8)
        }
    }
}
struct Cursor {
    pub x: u32,
    pub y: u32,
    pub x_bound: u32,
    pub y_bound: u32,
}
impl Cursor {
    fn to_tuple(&self) -> (u32, u32) {
        (self.x, self.y)
    }
    fn new(x_bound: u32, y_bound: u32) -> Self {
        Cursor {
            x: 0,
            y: 0,
            x_bound,
            y_bound,
        }
    }
    fn set(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }
    fn left(&mut self) {
        self.x = (self.x.saturating_sub(1)) % self.x_bound;
        if self.x == 0 {
            self.y = self.y.saturating_sub(1);
        };
    }
    fn right(&mut self) {
        self.x = (self.x.saturating_add(1)) % self.x_bound;
        if self.x == 0 {
            self.y = self.y.saturating_add(1);
        };
    }
    fn up(&mut self) {
        self.y = self.y.saturating_add(1);
    }
    fn down(&mut self) {
        self.y = self.y.saturating_sub(1);
    }

    fn new_line(&mut self) {
        // (0, cursor.1 + 1)
        self.x = 0;
        self.y += 1;
    }
}
const TITLE: &str = "NISVC";
const COLUMNS: u32 = 40;
const ROWS: u32 = 30;
const SCREEN_WIDTH: u32 = 400;
const SCREEN_HEIGHT: u32 = 300;
const FONT_PATH: &str = "./assets/Glass_TTY_VT220.ttf";

pub struct MMIO {
    display: TextModeDisplay,
    cursor: Cursor,
}
impl MMIO {
    pub fn new() -> Result<Self, String> {
        let display =
            TextModeDisplay::new(TITLE, COLUMNS, ROWS, SCREEN_WIDTH, SCREEN_HEIGHT, FONT_PATH)?;
        let cursor = Cursor::new(display.columns, display.rows);
        Ok(MMIO { display, cursor })
    }
    /// read addr 0x0 summons this function
    /// pops a value off the key_stack, if no values return 0
    fn read_key_mmio(&mut self) -> u8 {
        self.display.key_processor();
        let key = self.display.key_stack.pop();
        if let Some(key_u8) = key {
            return key_u8;
        } else {
            return 0;
        }
    }
    fn write_display_mmio_addr() {}

    pub fn mmio_read_handler(&mut self, address: constant::RegisterWidth) -> u8 {
        match address {
            0x0 => self.read_key_mmio(),
            _ => 0,
        }
    }
    pub fn mmio_write_handler(
        &mut self,
        address: constant::RegisterWidth,
        byte: u8,
    ) -> Result<(), String> {
        match address {
            // keyboard input address
            0x0 => self.display.key_stack.push(byte),
            // display state control address
            0x1 => match byte {
                0 => {
                    self.display.display.canvas.window_mut().hide();
                    println!("hide display")
                }
                1 => {
                    self.display.display.canvas.window_mut().show();
                    println!("show display")
                }
                2 => {
                    self.display.display.canvas.present();
                    self.display.display.canvas.clear();
                    println!("refresh display")
                }
                _ => (),
            },
            // cursor manual setting addresses
            0x2 => self.cursor.x = byte as u32,
            0x3 => self.cursor.y = byte as u32,
            // cursor control address
            0x4 => match byte {
                0 => self.cursor.left(),
                1 => self.cursor.right(),
                2 => self.cursor.up(),
                3 => self.cursor.down(),
                _ => (),
            },
            // display write at cursor
            0x5 => {
                if byte != 0 {
                    self.display.write(byte, self.cursor.to_tuple())?
                }
            }

            _ => (),
        };
        Ok(())
    }
}
