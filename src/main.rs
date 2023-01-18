use functions::{
    linspace::Linspace, polygonising::polygoniseScalarField, polygonising::Normal,
    polygonising::Vertex,
};

#[macro_use]
extern crate glium;
mod cube;
mod functions;
fn main() {
    let limit = 25.0;
    let linspace = Linspace::new(1.0, limit);
    let mut metaBallsCenters = vec![(-1.0, 2.0, -1.0), (1.0, 2.0, 1.0)];
    let metaBallsRads = vec![2.0, 3.0];

    let mut stmetaball_vel = (0.7, 0.60, 0.4);
    let mut ndmetaball_vel = (0.5, 0.70, 0.4);

    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertex_shader_src = r#"
        #version 150

        in vec3 position;
        in vec3 normal;

        out vec3 v_normal;

        uniform mat4 matrix;
        void main() {
            v_normal = transpose(inverse(mat3(matrix))) * normal;  
            gl_Position =   matrix * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec3 v_normal;
        out vec4 color;
        uniform vec3 u_light;

        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.6, 0.0, 0.0);
            vec3 regular_color = vec3(1.0, 0.0, 0.0);
            color = vec4(mix(dark_color, regular_color, brightness), 1.0);
        } "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();
    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let light = [-1.0, 0.4, 0.9f32];

     

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
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
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let matrix = [
            [0.03, 0.0, 0.0, 0.0],
            [0.0, 0.03, 0.0, 0.0],
            [0.0, 0.0, 0.03, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ];

        move_point(&mut metaBallsCenters[0], &stmetaball_vel);
        move_point(&mut metaBallsCenters[1], &ndmetaball_vel);

        repulsion(
            &metaBallsCenters[0],
            &mut stmetaball_vel,
            &metaBallsRads[0],
            limit / 2.0,
            limit / 2.0,
            limit / 2.0,
        );
        repulsion(
            &metaBallsCenters[1],
            &mut ndmetaball_vel,
            &metaBallsRads[1],
            limit / 2.0,
            limit / 2.0,
            limit / 2.0,
        );

        let mut shape: Vec<Vertex> = polygoniseScalarField(&linspace, &metaBallsCenters, &metaBallsRads);
        let positions = glium::VertexBuffer::new(&display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let mut norms = Vec::new();
        for _ in 0..positions.len() {
            norms.push(Normal {
                normal: (0.614804, 0.493997, 0.614804),
            });
        }
        let normals = glium::VertexBuffer::new(&display, &norms).unwrap();
        target.draw(
                (&positions, &normals),
                &indices,
                &program,
                &uniform! { matrix: matrix, u_light: light },
                &params,
            )
            .unwrap();
        target.finish().unwrap();
    });
}

// Plotting

pub fn move_point(coordinate: &mut (f64, f64, f64), velocity: &(f64, f64, f64)) {
    coordinate.0 += velocity.0;
    coordinate.1 += velocity.1;
    coordinate.2 += velocity.2;
}

pub fn repulsion(
    coordinate: &(f64, f64, f64),
    velocity: &mut (f64, f64, f64),
    radius: &f64,
    x_limit: f64,
    y_limit: f64,
    z_limit: f64,
) {
    let x = coordinate.0;
    let y = coordinate.1;
    let z = coordinate.2;
    if x > x_limit - radius {
        velocity.0 = -velocity.0;
    } else if x < -x_limit + radius {
        velocity.0 = -velocity.0;
    }
    if y > y_limit - radius {
        velocity.1 = -velocity.1;
    } else if y < -y_limit + radius {
        velocity.1 = -velocity.1;
    }
    if z > z_limit - radius {
        velocity.2 = -velocity.2;
    } else if z < -z_limit + radius {
        velocity.2 = -velocity.2;
    }
}
