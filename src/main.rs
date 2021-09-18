use std::time::Duration;

use glutin_window::GlutinWindow as Window;
use graphics::{Text, Transformed};
use opengl_graphics::{CreateTexture, GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::EventLoop;

mod doom;

struct Game {
    window: Window,
    gl: GlGraphics,

    start: std::time::Instant,
}

impl doom::Doom for Game {
    fn draw_frame(&mut self, screen_buffer: &[u32], xres: usize, yres: usize) {
        println!("draw_frame(<screen_buffer>, {:?}, {:?})", xres, yres);
        //self.window.ctx.window().
        let inner_size = self.window.ctx.window().inner_size();
        let outer_size = self.window.ctx.window().outer_size();
        let viewport = graphics::Viewport {
            rect: [0, 0, inner_size.width as i32, inner_size.height as i32],
            window_size: [outer_size.width as f64, outer_size.height as f64],
            draw_size: [inner_size.width, inner_size.height],
        };

        let x_pos = f64::from((self.start.elapsed().as_millis() % 300) as u32);
        let mut events = Events::new(EventSettings::new());
        events.set_max_fps(1000);
        while let Some(e) = events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                self.gl.draw(args.viewport(), |c, gl| {
                    // Clear the screen.
                    /*graphics::clear([1.0, 0.0, 1.0, 1.0], gl);

                    graphics::rectangle(
                        [0.0, 1.0, 0.0, 1.0],
                        graphics::rectangle::square(50.0, 50.0, 100.0),
                        c.transform.trans(x_pos, 50.0),
                        gl,
                    );*/
                    let image = graphics::Image::new().rect([
                        0.0,
                        0.0,
                        f64::from(c.get_view_size()[0]),
                        f64::from(c.get_view_size()[1]),
                    ]);
                    //let texture = Texture::from_memory_alpha(, TextureSettings::new());
                    /*let texture =
                    Texture::from_path("/tmp/test.png", &TextureSettings::new()).unwrap();*/
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

                    //std::fs::write("/tmp/buffer", &screen_buffer_rgba).unwrap();

                    // No image without this useless call!
                    graphics::rectangle(
                        [0.0, 1.0, 0.0, 1.0],
                        graphics::rectangle::square(0.0, 0.0, 0.0),
                        c.transform.trans(0.0, 0.0),
                        gl,
                    );

                    //println!("View size: {:?}", c.get_view_size());
                });
            }

            if let Some(_args) = e.update_args() {
                break;
            }
        }
    }
    fn get_key(&mut self) -> Option<doom::KeyData> {
        println!("get_key() -> None");
        None
    }
    fn set_window_title(&mut self, title: &str) {
        println!("set_window_title({:?})", title);
    }
}

fn u32_to_u8(arr: &[u32]) -> &[u8] {
    let len = 4 * arr.len();
    let ptr = arr.as_ptr() as *const u8;
    unsafe { std::slice::from_raw_parts(ptr, len) }
}

fn main() {
    //let opengl = OpenGL::V3_2;
    let opengl = OpenGL::V2_1;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
        "Piston-Doom",
        [doom::DOOMGENERIC_RESX as u32, doom::DOOMGENERIC_RESY as u32],
    )
    .graphics_api(opengl)
    .vsync(true)
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut gl = GlGraphics::new(opengl);

    doom::init(Game {
        window,
        gl,
        start: std::time::Instant::now(),
    });

    /*
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, gl| {
                // Clear the screen.
                graphics::clear([1.0, 0.0, 1.0, 1.0], gl);

                let image = graphics::Image::new().rect([
                    0.0,
                    0.0,
                    f64::from(c.get_view_size()[0]),
                    f64::from(c.get_view_size()[1]),
                ]);
                //let texture = Texture::from_memory_alpha(, TextureSettings::new());
                let texture =
                    Texture::from_path(std::path::Path::new("test.png"), &TextureSettings::new())
                        .unwrap();
                /*let texture = Texture::create(
                    &mut (),
                    opengl_graphics::Format::Rgba8,
                    u32_to_u8(screen_buffer),
                    [xres as u32, yres as u32],
                    &TextureSettings::new(),
                )
                .unwrap();*/
                image.draw(&texture, &Default::default(), c.transform, gl);

                graphics::rectangle(
                    [0.0, 1.0, 0.0, 1.0],
                    graphics::rectangle::square(0.0, 0.0, 0.0),
                    c.transform.trans(0.0, 0.0),
                    gl,
                );
            });
        }

        if let Some(_args) = e.update_args() {
            // TODO
        }
    }*/
}
