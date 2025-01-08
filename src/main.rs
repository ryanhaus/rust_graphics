use std::fs::File;
use std::io::BufReader;
use std::num::NonZeroU32;
use std::rc::Rc;
use obj::{load_obj, Obj};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;
use std::time::{Instant, Duration};
use rand::Rng;

mod winit_app;
mod triangles;
use triangles::*;

fn main() {
    let start = Instant::now();

    let obj_input = BufReader::new(File::open("res/dragon.obj").unwrap());
    let model: Obj = load_obj(obj_input).unwrap();

    let vertices = model.vertices
        .into_iter()
        .map(|v| (Point3D::new(v.position[0] as f64, v.position[1] as f64, v.position[2] as f64), Point3D::new(v.normal[0] as f64, v.normal[1] as f64, v.normal[2] as f64)))
        .collect::<Vec::<(Point3D, Point3D)>>();

    let triangles = model.indices
        .chunks(3)
        .map(|indices| (indices[0] as usize, indices[1] as usize, indices[2] as usize))
        .map(|(a, b, c)| (vertices[a], vertices[b], vertices[c]))
        .map(|(a, b, c)| (Triangle3D::new(a.0, b.0, c.0), Triangle3D::new(a.1, b.1, c.1)))
        .map(|(tri, normal_tri)| ColorTriangle::new(0xFFFFFF, tri, normal_tri))
        .collect::<Vec<ColorTriangle>>();

    let mut object = Object3D::new(triangles);

    let mut camera = Camera::new(Point3D::new(0.0, 0.0, -5.0), Point3D::new(0.0, 0.0, -1.0));
    let mut light = Light::new(Point3D::new(2.0, 0.75, -0.5), (1.0, 0.3, 0.0));

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
                    if buffer.len() as u32 != width * height {
                        return;
                    }
                    
                    let time = (start.elapsed().as_millis() as f64) / 1000.0;
                    object.rotation = time;

                    let scene = Scene::new(camera, light);

                    let mut paint_buffer = PaintBuffer::new(width, height);

                    for i in 0..paint_buffer.pixel_buffer.len() {
                        paint_buffer.pixel_buffer[i] = 0x111111; //background color
                    }

                    object.paint_to_buffer(&mut paint_buffer, scene);
                    
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
