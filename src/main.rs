use functions::{
    linspace::Linspace, polygonising::polygoniseScalarField,
    polygonising::Vertex,
};
use rand::{Rng};
use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use fltk::{prelude::*, app::App, window::Window, valuator, button};


#[macro_use]
extern crate glium;
mod cube;
mod functions;
fn main() {
    let (tx, rx) = mpsc::channel();
    

    thread::spawn(move || {
    let a = App::default();
    let mut wind = Window::new(100, 100, 400, 300, "My Window");

    let mut slider1 = valuator::HorNiceSlider::default().with_size(400, 20).center_of_parent();
    slider1.set_minimum(-3.14);
    slider1.set_maximum(3.14);
    slider1.set_step(0.1, 1); // increment by 1.0 at each 1 step
    slider1.set_value(0.); // start in the middle
    slider1.set_callback(move |s| {
        tx.send(s.value() as f32).unwrap();
    });
    wind.end();
    wind.show();
    a.run().unwrap();
    thread::sleep(Duration::from_millis(1));});
    let limit = 20.0;
    let linspace = Linspace::new(1.0, limit);
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertex_shader_src = r#"
    #version 300 es

    precision highp float;
    precision highp int;
    
    uniform mat4 MVMatrix;  //Model View Matrix 
    uniform mat4 view;
    uniform mat4 PMatrix;
    in vec3 position;
    out vec3 vPos;
    void main()
    {
          gl_Position= PMatrix * view * MVMatrix * vec4(position.xyz,1.0);
          vPos = (MVMatrix * vec4(position.xyz,1.0)).xyz;
    }
    "#;

    let fragment_shader_src = r#"
        #version 300 es
        precision highp float;
        precision highp int;

        in vec3 vPos;
        out vec4 color;
        uniform vec3 u_light;

        void main() {
            vec3 fdx = vec3(dFdx(vPos.x),dFdx(vPos.y),dFdx(vPos.z));    
            vec3 fdy = vec3(dFdy(vPos.x),dFdy(vPos.y),dFdy(vPos.z));
            vec3 normal = normalize(cross(fdx,fdy));
            float brightness = dot(normalize(normal), normalize(u_light));
            vec3 dark_color = vec3(0.156, 0.29, 0.38);
            vec3 regular_color = vec3(0.26, 0.40, 0.55);
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
    let light = [-1.0, -1.0, -2.0f32];
    

    let mut time: f64 = 0.0;
    let mut rng = rand::thread_rng();

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(10000);
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
        target.clear_color_and_depth((0.95, 0.90, 0.85, 1.0), 1.0);
        let alpha = rx.recv().unwrap();
        println!("The angle is: {}", alpha);
        let view = view_matrix(&[alpha.sin(), 0.0, alpha.cos() + 2.5], &[-alpha.sin(), 0.0, - (alpha.cos()+2.5)/3.5], &[0.0, 0.5, 0.0]);
        
        let matrix = [
            [0.05, 0.0, 0.0, 0.0],
            [0.0, 0.05, 0.0, 0.0],
            [0.0, 0.0, 0.05, 0.0],
            [0.0, 0.0, 2.0, 1.0f32]
        ];

        let perspective = {
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
        };

      
        time+=  0.5 * rng.gen::<f64>();
        let mut shape: Vec<Vertex> = polygoniseScalarField(&linspace, time);
        let positions = glium::VertexBuffer::new(&display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        target.draw(
            &positions,
            &indices,
               &program,
                &uniform! { MVMatrix: matrix,PMatrix: perspective, u_light: light, view: view},
                &params,
            )
            .unwrap();
        target.finish().unwrap();
    });
}



fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}


