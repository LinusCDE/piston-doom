use std::collections::VecDeque;
use std::ffi::c_void;

use glutin_window::GlutinWindow as Window;
use graphics::Transformed;
use opengl_graphics::{CreateTexture, GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::OpenGLWindow;
use piston::window::WindowSettings;
use piston::{Button, EventLoop, Key, PressEvent, ReleaseEvent};

use doomgeneric::game;
use doomgeneric::input::{keys, KeyData};

struct Game {
    window: Window,
    gl: GlGraphics,

    input_queue: VecDeque<KeyData>,
}

fn button_to_doom_key(button: Button) -> Option<u8> {
    match button {
        Button::Keyboard(key) => match key {
            // Map keyboard keys from m_controller.c
            Key::Right => Some(*keys::KEY_RIGHT),
            Key::Left => Some(*keys::KEY_LEFT),
            Key::Up => Some(*keys::KEY_UP),
            Key::Down => Some(*keys::KEY_DOWN),
            Key::Comma => Some(*keys::KEY_STRAFELEFT),
            Key::Period => Some(*keys::KEY_STRAFERIGHT),
            Key::RCtrl => Some(*keys::KEY_FIRE),
            Key::Space => Some(*keys::KEY_USE),
            Key::LAlt | Key::RAlt => Some(*keys::KEY_STRAFE),
            Key::LShift | Key::RShift => Some(*keys::KEY_SPEED),
            // Let doom deal with the rest
            _ => Some(key as u8),
        },
        _ => None,
    }
}

impl game::DoomGeneric for Game {
    fn draw_frame(&mut self, screen_buffer: &[u32], xres: usize, yres: usize) {
        let mut events = Events::new(EventSettings::new());
        events.set_max_fps(1000);
        while let Some(e) = events.next(&mut self.window) {
            if let Some(button) = e.press_args() {
                if let Some(key) = button_to_doom_key(button) {
                    let keydata = KeyData { pressed: true, key };
                    self.input_queue.push_back(keydata);
                }
            } else if let Some(button) = e.release_args() {
                if let Some(key) = button_to_doom_key(button) {
                    let keydata = KeyData {
                        pressed: false,
                        key,
                    };
                    self.input_queue.push_back(keydata);
                }
            } else if let Some(args) = e.render_args() {
                self.gl.draw(args.viewport(), |c, gl| {
                    // Clear the screen.
                    graphics::clear([0.0, 0.0, 0.0, 1.0], gl);

                    let image = graphics::Image::new().rect([
                        0.0,
                        0.0,
                        f64::from(c.get_view_size()[0]),
                        f64::from(c.get_view_size()[1]),
                    ]);
                    let mut screen_buffer_rgba: Vec<u8> = Vec::with_capacity(xres * yres * 4);
                    for argb in screen_buffer {
                        screen_buffer_rgba.push(((argb >> 16) & 0xFF) as u8);
                        screen_buffer_rgba.push(((argb >> 8) & 0xFF) as u8);
                        screen_buffer_rgba.push(((argb >> 0) & 0xFF) as u8);
                        // Alpha seems to be opacity. Inverting it.
                        screen_buffer_rgba.push(255 - ((argb >> 24) & 0xFF) as u8);
                    }
                    let texture = Texture::create(
                        &mut (),
                        opengl_graphics::Format::Rgba8,
                        &screen_buffer_rgba,
                        [xres as u32, yres as u32],
                        &TextureSettings::new(),
                    )
                    .unwrap();
                    image.draw(&texture, &Default::default(), c.transform, gl);

                    // No image without this useless call!
                    graphics::rectangle(
                        [0.0, 1.0, 0.0, 1.0],
                        graphics::rectangle::square(0.0, 0.0, 0.0),
                        c.transform.trans(0.0, 0.0),
                        gl,
                    );
                });
            } else if let Some(_args) = e.update_args() {
                break;
            }
        }
    }
    fn get_key(&mut self) -> Option<KeyData> {
        self.input_queue.pop_front()
    }
    fn set_window_title(&mut self, title: &str) {
        self.window.window.set_title(title);
    }
}

fn main() {
    //let opengl = OpenGL::V3_2;
    let opengl = OpenGL::V2_1;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
        "Piston-Doom",
        [game::DOOMGENERIC_RESX as u32, game::DOOMGENERIC_RESY as u32],
    )
    .graphics_api(opengl)
    .vsync(true)
    .build()
    .unwrap();

    gl::load_with(|s| window.get_proc_address(s) as *const c_void);

    let gl = GlGraphics::new(opengl);

    game::init(Game {
        window,
        gl,
        input_queue: VecDeque::new(),
    });
    loop {
        doomgeneric::game::tick();
    }
}
