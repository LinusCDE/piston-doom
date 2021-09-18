use glutin_window::GlutinWindow as Window;
use graphics::Transformed;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

mod doom;

fn main() {
    doom::init();

    //let opengl = OpenGL::V3_2;
    let opengl = OpenGL::V2_1;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Piston-Doom", [640, 480])
        .graphics_api(opengl)
        .vsync(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, gl| {
                // Clear the screen.
                graphics::clear([1.0, 0.0, 1.0, 1.0], gl);

                graphics::rectangle(
                    [0.0, 1.0, 0.0, 1.0],
                    graphics::rectangle::square(50.0, 50.0, 100.0),
                    c.transform.trans(50.0, 50.0),
                    gl,
                );
            });
        }

        if let Some(_args) = e.update_args() {
            // TODO
        }
    }
}
