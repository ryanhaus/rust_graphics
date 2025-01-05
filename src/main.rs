use std::num::NonZeroU32;
use std::rc::Rc;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

mod winit_app;
mod triangles;
use triangles::*;

fn main() {
    let triangle = Triangle2D::new(
        Point2D::new(0.25, 0.75),
        Point2D::new(0.5, 0.25),
        Point2D::new(0.75, 0.75)
    );

    let event_loop = EventLoop::new().unwrap();

    let mut app = winit_app::WinitAppBuilder::with_init(
        |elwt| {
            let window = {
                let window = elwt.create_window(Window::default_attributes());
                Rc::new(window.unwrap())
            };
            let context = softbuffer::Context::new(window.clone()).unwrap();

            (window, context)
        },
        |_elwt, (window, context)| softbuffer::Surface::new(context, window.clone()).unwrap(),
    )
    .with_event_handler(|(window, _context), surface, event, elwt| {
        elwt.set_control_flow(ControlFlow::Wait);

        match event {
            Event::WindowEvent { window_id, event: WindowEvent::RedrawRequested } if window_id == window.id() => {
                let Some(surface) = surface else {
                    eprintln!("RedrawRequested fired before Resumed or after Suspended");
                    return;
                };
                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };
                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();


                let (tri_x_range, tri_y_range) = triangle.get_bounding_box_px(width, height);

                let mut buffer = surface.buffer_mut().unwrap();

                for y in tri_y_range {
                    for x in tri_x_range.clone() {
                        let index = (x + y * width) as usize;
                        let x = (x as f32) / (width as f32);
                        let y = (y as f32) / (height as f32);
                        let p = Point2D::new(x, y);

                        buffer[index] = if triangle.contains_point(p) { 0xFFFFFF } else { 0x000000 };
                    }
                }

                buffer.present().unwrap();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                elwt.exit();
            }
            _ => {}
        }
    });

    event_loop.run_app(&mut app).unwrap();
}
