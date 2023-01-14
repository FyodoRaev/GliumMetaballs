use functions::{linspace::Linspace, polygonising::polygoniseScalarField, polygonising::Vertex};

#[macro_use]
extern crate glium;
mod cube;
mod functions;
fn main() {
    let limit = 10.0;
    let linspace = Linspace::new(0.3, limit);
    let mut metaBallsCenters = vec![(-1.0,2.0,-1.0), (1.0, 2.0, 1.0)];
    let metaBallsRads = vec![1.0, 2.0];

    let mut stmetaball_vel = (0.1, 0.05, 0.01);
    let mut ndmetaball_vel = (0.1, 0.20, 0.15);
    
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertex_shader_src = r#"
        #version 140
        in vec3 position;
        uniform mat4 matrix;
        void main() {
            gl_Position = matrix * vec4(position, 1.0);
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
            [0.035, 0.0, 0.0, 0.0],
            [0.0, 0.035, 0.0, 0.0],
            [0.0, 0.0, 0.035, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ];

        move_point(&mut metaBallsCenters[0], &stmetaball_vel);
        move_point(&mut metaBallsCenters[1], &ndmetaball_vel);

        repulsion(&metaBallsCenters[0], &mut stmetaball_vel,&metaBallsRads[0], limit/2.0 , limit/2.0, limit/2.0);
        repulsion(&metaBallsCenters[1], &mut ndmetaball_vel,&metaBallsRads[1], limit/2.0 , limit/2.0, limit/2.0);
        let testVertices = polygoniseScalarField(&linspace, &metaBallsCenters, &metaBallsRads);

        let mut shape: Vec<Vertex> = Vec::new();
        let mut list_of_indices: Vec<u32> = Vec::new();
        for vertex in testVertices {
        shape.push(vertex.0);
        list_of_indices.push(vertex.1 as u32);
        }
        let positions = glium::VertexBuffer::new(&display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        /*let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                [         0.0         ,     f ,              0.0              ,   0.0],
                [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
            ]
        }; */

        target.draw(&positions, &indices, &program, &uniform! { matrix: matrix, },
                    &Default::default()).unwrap();
        target.finish().unwrap();
    });
}



// Plotting

pub fn move_point(coordinate: &mut(f64, f64, f64), velocity: &(f64,f64, f64)) {
    coordinate.0 += velocity.0;
    coordinate.1 += velocity.1;
    coordinate.2 += velocity.2;
  }
  
  pub fn repulsion(coordinate: &(f64,f64, f64), velocity: &mut(f64, f64, f64), radius: &f64, x_limit: f64, y_limit: f64, z_limit: f64) {
      let x = coordinate.0;
      let y = coordinate.1;
      let z = coordinate.2;
      if x > x_limit-radius {
        velocity.0=-velocity.0;}
      else if x < -x_limit + radius {
        velocity.0=-velocity.0;}
      if y > y_limit - radius {
            velocity.1=-velocity.1;}
      else if y < -y_limit + radius {
            velocity.1=-velocity.1;}
      if z > z_limit - radius {
        velocity.2=-velocity.2;}
      else if z < -z_limit + radius {
        velocity.2=-velocity.2;}
  
  }
  