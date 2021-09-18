use std::collections::VecDeque;

use glutin_window::GlutinWindow as Window;
use graphics::Transformed;
use opengl_graphics::{CreateTexture, GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, EventLoop, Key, PressEvent, ReleaseEvent};

mod doom;

use doom::KeyData;

struct Game {
    window: Window,
    gl: GlGraphics,

    input_queue: VecDeque<KeyData>,
}

fn button_to_doom_key(button: Button) -> Option<u8> {
    match button {
        Button::Keyboard(key) => match key {
            // Map keyboard keys from m_controller.c
            Key::Right => Some(unsafe { doom::key_right as u8 }),
            Key::Left => Some(unsafe { doom::key_left as u8 }),
            Key::Up => Some(unsafe { doom::key_up as u8 }),
            Key::Down => Some(unsafe { doom::key_down as u8 }),
            Key::Comma => Some(unsafe { doom::key_strafeleft as u8 }),
            Key::Period => Some(unsafe { doom::key_straferight as u8 }),
            Key::RCtrl => Some(unsafe { doom::key_fire as u8 }),
            Key::Space => Some(unsafe { doom::key_use as u8 }),
            Key::LAlt | Key::RAlt => Some(unsafe { doom::key_strafe as u8 }),
            Key::LShift | Key::RShift => Some(unsafe { doom::key_speed as u8 }),
            // Let doom deal with the rest
            _ => Some(key as u8),
        },
        _ => None,
    }
}

impl doom::Doom for Game {
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
    fn get_key(&mut self) -> Option<doom::KeyData> {
        self.input_queue.pop_front()
    }
    fn set_window_title(&mut self, title: &str) {
        self.window.ctx.window().set_title(title);
    }
}

fn main() {
    //let opengl = OpenGL::V3_2;
    let opengl = OpenGL::V2_1;

    // Create an Glutin window.
    let window: Window = WindowSettings::new(
        "Piston-Doom",
        [doom::DOOMGENERIC_RESX as u32, doom::DOOMGENERIC_RESY as u32],
    )
    .graphics_api(opengl)
    .vsync(true)
    .build()
    .unwrap();

    let gl = GlGraphics::new(opengl);

    doom::init(Game {
        window,
        gl,
        input_queue: VecDeque::new(),
    });
}
