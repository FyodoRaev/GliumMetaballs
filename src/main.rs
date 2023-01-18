use functions::{linspace::Linspace, polygonising::polygoniseScalarField, polygonising::Vertex};

#[macro_use]
extern crate glium;
mod cube;
mod functions;
fn main() {
    let linspace = Linspace::new(10.0, 20.0);
    let mut metaBallsCenters = vec![(0.0,0.0,0.0)];
    let radius= 6.0;
    let testVertices = polygoniseScalarField(&linspace, &metaBallsCenters, &radius);
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();


    let mut shape: Vec<Vertex> = Vec::new();
    let mut list_of_indices: Vec<u32> = Vec::new();
    for vertex in testVertices {
        shape.push(vertex.0);
        list_of_indices.push(vertex.1 as u32);
    }
    let positions = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec3 position;
        uniform mat4 matrix;
        void main() {
            gl_Position =  matrix * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src,
                                              None).unwrap();
    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let matrix = [
            [0.02, 0.0, 0.0, 0.0],
            [0.0, 0.02, 0.0, 0.0],
            [0.0, 0.0, 0.02, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ];

        let params = glium::DrawParameters {
            polygon_mode: glium::draw_parameters::PolygonMode::Fill,
            //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        target.draw(&positions, &indices, &program, &uniform! { matrix: matrix,}, &params).unwrap();
        target.finish().unwrap();
    });
}
