use std::num::NonZeroU32;
use std::rc::Rc;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

mod winit_app;
mod triangles;
use triangles::*;

fn main() {
    let mut triangle = Triangle2D::new(
        Point2D::new(0.25, 0.75),
        Point2D::new(0.5, 0.25),
        Point2D::new(0.75, 0.75)
    );

    let mut counter: f32 = 0.0;

    let event_loop = EventLoop::new().unwrap();

    let mut app = winit_app::WinitAppBuilder::with_init(
        |event_loop| {
            let window = winit_app::make_window(event_loop, |w| w);
            let context = softbuffer::Context::new(window.clone()).unwrap();

            (window, context)
        },
        |_elwft, (window, context)| softbuffer::Surface::new(context, window.clone()).unwrap(),
    )
    .with_event_handler(move |(window, _context), surface, event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);

        match event {
            Event::WindowEvent { window_id, event: WindowEvent::Resized(size) } if window_id == window.id() => {
                let Some(surface) = surface else {
                    eprintln!("Resized fired before Resumed or after Suspended");
                    return;
                };
                
                if let (Some(width), Some(height)) = (NonZeroU32::new(size.width), NonZeroU32::new(size.height)) {
                    surface
                        .resize(width, height)
                        .unwrap();
                }
            }
            
            Event::WindowEvent { window_id, event: WindowEvent::RedrawRequested } if window_id == window.id() => {
                let Some(surface) = surface else {
                    eprintln!("RedrawRequested fired before Resumed or after Suspended");
                    return;
                };
                let size = window.inner_size();
                if let (Some(width), Some(height)) =
                    (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                {
                    let width = u32::from(width);
                    let height = u32::from(height);

                    let mut buffer = surface.buffer_mut().unwrap();

                    counter += 0.05;
                    triangle.b.y = 0.25 + 0.1 * counter.sin();

                    let mut paint_buffer = PaintBuffer::new(width, height);
                    triangle.paint_to_buffer(&mut paint_buffer, 0xFF0000);


                    if buffer.len() == paint_buffer.pixel_buffer.len() {
                        buffer.copy_from_slice(&paint_buffer.pixel_buffer);
                        buffer.present().unwrap();
                    }
                }
            }

            Event::AboutToWait => {
               window.request_redraw();
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
