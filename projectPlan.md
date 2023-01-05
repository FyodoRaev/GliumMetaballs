## Записки

- Сейчас я создал linspace в котором есть доступ к кубам(8 вершин с данными координатами) и к точкам ( вроде они мне больше не понадобятся)
- Я скопировал код для FloatIterator https://stackoverflow.com/questions/47867519/how-to-do-a-for-loop-with-boundary-values-and-step-as-floating-point-values

### Будущий план :
---
* [x] (linspace.rs) рассмотреть триангуляцию для конкретной cube позиции (какие значения функции какой вершины больше заданного порога а какие нет)
* [x] (linspace.rs) рассмотреть по этой триангуляции какие ребра нужно соединить
* [x] (linspace.rs) рассмотреть две вершины которые формируют данное ребро
* [x] (linspace.rs) найти координаты середины этого ребра, которые как раз и будут координатами вершины треугольника которой должен быть отрисован
* [x] (linspace.rs) добавить эту вершину
  
  ----

* [ ] Теперь процесс триангуляции создан (но не проверен), нужно запустить вычисления, связав OpenGL и вычисления из Linspace используя Compute shader:
  * [ ] См совет Николая Вадимовича __#🦄__ ниже и реализуй 


-----
Как это реализовать на глиуме?

 1. Можно в общем то запустить для main'а цикл и перед итерацией запуска pipelin'а openglя пересоздавать вершины основываясь на marching_cubes но пока это выглядит как лютый кастыль.
 2. Про верхний пункт: есть в OpenGL так называемый Compute Shader 
   
    "A Compute Shader is a Shader Stage that is used entirely for computing arbitrary information. While it can do rendering, it is generally used for tasks not directly related to drawing triangles and pixels." 

    Совет, который я получил от Николая Вадимовича: 
    ```
    Nikolai Poliarnyi [#🦄], 
    Но как минимум самый дубовый вариант - запускать compute shader чтобы создать вершины и треугольники в видеопамяти (просто массив координат и массив индексов)
    А дальше на них запустить обычный пайплайн, это наверное проще выйдет
    Плюс отлаживать проще, всегда можно будет посмотреть на промежуточный результат (отгрузив эти массивы и посмотрев на их данные)
    ```


Code that I currently cutted out
```
...
#[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    
    let positions = glium::VertexBuffer::new(&display, &cube::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &cube::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                          &cube::INDICES).unwrap();

    
    let compute_shader_src = glium::program::ComputeShader::from_source(&display, r#"\
            #version 430
            layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
            layout(std140) buffer MyBlock {
                float power;
                vec4 values[4096/4];
            };
            void main() {
                vec4 val = values[gl_GlobalInvocationID.x];
                values[gl_GlobalInvocationID.x] = pow(val, vec4(power));
            }
        "#).unwrap();
    let vertex_shader_src = r#"
        #version 150
        in vec3 position;
        in vec3 normal;
        out vec3 v_normal;
        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 model;
        void main() {
            mat4 modelview = view * model;
            v_normal = transpose(inverse(mat3(modelview))) * normal;
            gl_Position = perspective * modelview * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 150
        in vec3 v_normal;
        out vec4 color;
        uniform vec3 u_light;
        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.6, 0.0, 0.0);
            vec3 regular_color = vec3(1.0, 0.0, 0.0);
            color = vec4(mix(dark_color, regular_color, brightness), 1.0);
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
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let model = [
            [0.02, 0.0, 0.0, 0.0],
            [0.0, 0.02, 0.0, 0.0],
            [0.0, 0.0, 0.02, 0.0],
            [0.0, 0.0, 2.0, 1.0f32]
        ];

        let view = view_matrix(&[2.5, 0.5, 1.5], &[-2.0, 0.0, 0.2], &[0.0, 0.2, 0.0]);

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

        let light = [2.0, 1.0, 1.0f32];

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            polygon_mode: glium::draw_parameters::PolygonMode::Fill,
            //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        target.draw((&positions, &normals), &indices, &program,
                    &uniform! { model: model, view: view, perspective: perspective, u_light: light },
                    &params).unwrap();
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

```rust