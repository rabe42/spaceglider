#[macro_use]
extern crate glium;

extern crate glium_sdl2;
extern crate sdl2;

mod teapot;

use sdl2::keyboard::Keycode;
use glium::{glutin, Surface};

// TODO for users extending this example - remove the #[allow] item.
#[allow(unused_mut)]
fn main() {
    use glium_sdl2::DisplayBuild;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    video_subsystem.gl_attr().set_depth_size(24); // TODO: Depth buffer setting?

    let display = video_subsystem.window("Starglider", 800, 600)
        .resizable()
        .build_glium()
        .unwrap();


    // let event_loop = glutin::event_loop::EventLoop::new();
    // let wb = glutin::window::WindowBuilder::new();
    // let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    // let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                          &teapot::INDICES).unwrap();

    let vertex_shader_src = r#"
        #version 140

        in vec3 position;
        in vec3 normal;

        out vec3 v_normal;
        out vec3 v_position;

        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 model;

        void main() {
            mat4 modelview = view * model;
            v_normal = transpose(inverse(mat3(modelview))) * normal;
            gl_Position = perspective * modelview * vec4(position, 1.0);
            v_position = gl_Position.xyz / gl_Position.w;
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec3 v_normal;
        in vec3 v_position;

        out vec4 color;

        uniform vec3 u_light;

        const vec3 ambient_color = vec3(0.2, 0.0, 0.0);
        const vec3 diffuse_color = vec3(0.6, 0.0, 0.0);
        const vec3 specular_color = vec3(1.0, 1.0, 1.0);

        void main() {
            float diffuse = max(dot(normalize(v_normal), normalize(u_light)), 0.0);

            vec3 camera_dir = normalize(-v_position);
            vec3 half_direction = normalize(normalize(u_light) + camera_dir);
            float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);

            color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src,
                                              None).unwrap();


    // The SDL event loop.
    let mut running = true;
    let mut event_pump = sdl_context.event_pump().unwrap();

    while running {

        let mut target = display.draw();

        target.clear_color_and_depth((0.3, 0.3, 0.5, 0.0), 1.0);

        // do drawing here...
        target.finish().unwrap();

        // Event loop: polls for events sent to all windows

        for event in event_pump.poll_iter() {
            use sdl2::event::Event;

            match event {
                Event::Quit { .. } => {
                    running = false;
                },
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Escape => running = false,
                        _ => {},
                    }
                }
                _ => ()
            }
        }
    }
}
