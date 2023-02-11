use functions::{
    linspace::Linspace, polygonising::polygoniseScalarField,
    polygonising::Vertex,
};

#[macro_use]
extern crate glium;
mod cube;
mod functions;
fn main() {
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
    uniform mat4 PMatrix;
    in vec3 position;
    out vec3 vPos;
    void main()
    {
          gl_Position= PMatrix * MVMatrix * vec4(position.xyz,1.0);
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
    let light = [1.0, 1.0, 2.0f32];
    

    // Imgui
    

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(50 * 1000000);
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
            [0.04, 0.0, 0.0, 0.0],
            [0.0, 0.04, 0.0, 0.0],
            [0.0, 0.0, 0.04, 0.0],
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

      

        let mut shape: Vec<Vertex> = polygoniseScalarField(&linspace);
        let positions = glium::VertexBuffer::new(&display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        target.draw(
            &positions,
            &indices,
               &program,
                &uniform! { MVMatrix: matrix,PMatrix: perspective, u_light: light },
                &params,
            )
            .unwrap();
        target.finish().unwrap();
    });
}
