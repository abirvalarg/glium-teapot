extern crate glium;
mod teapot;
use glium::glutin::{self, event_loop::ControlFlow, event::{self, Event, StartCause}};
use glium::Surface as _;

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let ev_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &ev_loop)?;

    let shader = glium::Program::from_source(&display, include_str!("vertex.glsl"),
        include_str!("fragment.glsl"), None)?;

    let shape = glium::VertexBuffer::new(&display, &teapot::VERTICES)?;
    let indices = glium::index::IndexBuffer::new(&display,
                                                 glium::index::PrimitiveType::TrianglesList,
                                                 &teapot::INDICES
                                                 )?;
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS)?;
    let cam: [[f32; 4]; 4] = [
        [0.01, 0., 0., 0.],
        [0., 0.01, 0., 0.],
        [0., 0., 0.01, 0.],
        [0., 0., 0., 1.]
    ];
        let start_time = std::time::Instant::now();

    ev_loop.run(move |ev, _, control_flow| {
        let next_frame_time = std::time::Instant::now()
            + std::time::Duration::from_secs_f32(1. / 30.);
        *control_flow = ControlFlow::WaitUntil(next_frame_time);

        match ev {
            Event::WindowEvent { event, .. } => {
                if event == event::WindowEvent::CloseRequested {
                    *control_flow = ControlFlow::Exit;
                }
                return;
            }
            Event::NewEvents(ev) => match ev {
                StartCause::ResumeTimeReached { .. }
                    | StartCause::WaitCancelled { .. }
                    | StartCause::Init => (),
                _ => return
            },
            _ => return,
        }

        let mut target = display.draw();
        target.clear_color(0., 0., 0., 1.);
        target.draw((&shape, &normals), &indices, &shader,
            &glium::uniform! { cam: cam, light: [-1., 0., 0.] as [f32; 3], time: start_time.elapsed().as_secs_f32() },
            &Default::default()).unwrap();
        target.finish().unwrap();
    });
}
