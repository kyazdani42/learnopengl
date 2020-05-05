#![allow(non_upper_case_globals, non_snake_case)]
extern crate nalgebra_glm as glm;

use gl::types::*;

use image::GenericImage;

use glutin::dpi::{PhysicalPosition, PhysicalSize};
use glutin::event::{DeviceEvent, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::ControlFlow;
use glutin::event_loop::EventLoop;
use glutin::window::Window;
use glutin::window::WindowBuilder;
use glutin::Api;
use glutin::ContextBuilder;
use glutin::ContextWrapper;
use glutin::GlRequest;
use glutin::PossiblyCurrent;

use std::ffi::c_void;
use std::{mem, str, time};

mod shader;
use shader::Shader;

mod camera;
use camera::Camera;

mod renderer;
use renderer::Renderer;

static vertex_shader_path: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/res/vertex.glsl");
static fragment_shader_path: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/res/fragment.glsl");
static imagePath: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/container.jpg");
static image2Path: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/awesomeface.png");

const WIDTH: i32 = 1024;
const HEIGHT: i32 = 768;

fn main() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title("dev")
        .with_visible(true)
        .with_resizable(true)
        .with_inner_size(PhysicalSize::new(WIDTH, HEIGHT));

    let context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .build_windowed(wb, &el);

    let context = match context {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    context.window().set_cursor_visible(false);
    context.window().set_cursor_grab(true).unwrap();

    let context = unsafe { context.make_current().expect("Make context current") };
    gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);
    unsafe { gl::Viewport(0, 0, WIDTH, HEIGHT) }
    unsafe { gl::Enable(gl::DEPTH_TEST) }

    let (VAO, texture1, texture2) = unsafe {
        let (mut VAO, mut VBO) = (0, 0);
        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO);

        gl::BindVertexArray(VAO);

        #[cfg_attr(rustfmt, rustfmt::skip)]
        let vertices: [f32; 180] = [
            // positions       // texture coords
            -0.5, -0.5, -0.5,  0.0, 0.0,
             0.5, -0.5, -0.5,  1.0, 0.0,
             0.5,  0.5, -0.5,  1.0, 1.0,
             0.5,  0.5, -0.5,  1.0, 1.0,
            -0.5,  0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 0.0,

            -0.5, -0.5,  0.5,  0.0, 0.0,
             0.5, -0.5,  0.5,  1.0, 0.0,
             0.5,  0.5,  0.5,  1.0, 1.0,
             0.5,  0.5,  0.5,  1.0, 1.0,
            -0.5,  0.5,  0.5,  0.0, 1.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,

            -0.5,  0.5,  0.5,  1.0, 0.0,
            -0.5,  0.5, -0.5,  1.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,
            -0.5,  0.5,  0.5,  1.0, 0.0,

             0.5,  0.5,  0.5,  1.0, 0.0,
             0.5,  0.5, -0.5,  1.0, 1.0,
             0.5, -0.5, -0.5,  0.0, 1.0,
             0.5, -0.5, -0.5,  0.0, 1.0,
             0.5, -0.5,  0.5,  0.0, 0.0,
             0.5,  0.5,  0.5,  1.0, 0.0,

            -0.5, -0.5, -0.5,  0.0, 1.0,
             0.5, -0.5, -0.5,  1.0, 1.0,
             0.5, -0.5,  0.5,  1.0, 0.0,
             0.5, -0.5,  0.5,  1.0, 0.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,

            -0.5,  0.5, -0.5,  0.0, 1.0,
             0.5,  0.5, -0.5,  1.0, 1.0,
             0.5,  0.5,  0.5,  1.0, 0.0,
             0.5,  0.5,  0.5,  1.0, 0.0,
            -0.5,  0.5,  0.5,  0.0, 0.0,
            -0.5,  0.5, -0.5,  0.0, 1.0
        ];

        let float_size = mem::size_of::<GLfloat>();
        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * float_size) as GLsizeiptr,
            vertices.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        let stride = 5 * float_size as GLsizei;
        // position
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        // texture
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (3 * float_size) as *const c_void);
        gl::EnableVertexAttribArray(2);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        gl::EnableVertexAttribArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        let img = image::open(imagePath).expect("Failed to load texture");
        let data_container = img.raw_pixels();

        let img2 = image::open(image2Path).expect("Failed to load texture");
        let img2 = img2.flipv();
        let data_face = img2.raw_pixels();

        let mut texture1: u32 = 0;
        gl::GenTextures(1, &mut texture1);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, texture1);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            data_container.as_ptr() as *const c_void,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);

        let mut texture2: u32 = 0;
        gl::GenTextures(1, &mut texture2);
        gl::ActiveTexture(gl::TEXTURE1);
        gl::BindTexture(gl::TEXTURE_2D, texture2);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            img2.width() as i32,
            img2.height() as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            data_face.as_ptr() as *const c_void,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);

        (VAO, texture1, texture2)
    };

    let cubes = [
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(2.0, 5.0, -15.0),
        glm::vec3(-1.5, -2.2, -2.5),
        glm::vec3(-3.8, -2.0, -12.3),
        glm::vec3(2.4, -0.4, -3.5),
        glm::vec3(-1.7, 3.0, -7.5),
        glm::vec3(1.3, -2.0, -2.5),
        glm::vec3(1.5, 2.0, -2.5),
        glm::vec3(1.5, 0.2, -1.5),
        glm::vec3(-1.3, 1.0, -1.5),
    ];
    let shader = Shader::new(vertex_shader_path, fragment_shader_path);
    let textures = vec![texture1, texture2];

    let mut renderer = Renderer::new(shader, textures, VAO, context, cubes.to_vec(), WIDTH as f32, HEIGHT as f32);

    let frame_time = 1000 / 60;
    let mut timer = time::Instant::now();
    el.run(move |event, _, control_flow| {
        if timer.elapsed().as_millis() > frame_time {
            renderer.draw();
            timer = time::Instant::now();
        }
        *control_flow = match event {
            Event::WindowEvent { event, .. } => window_events(event, &mut renderer),
            Event::DeviceEvent { event, .. } => device_events(event, &mut renderer),
            _ => ControlFlow::Poll,
        };
    });
}

pub type Ctx = ContextWrapper<PossiblyCurrent, Window>;

fn window_events(event: WindowEvent, renderer: &mut Renderer) -> ControlFlow {
    match event {
        WindowEvent::Resized(size) => resize(size, renderer),
        WindowEvent::KeyboardInput { input, .. } => handle_keycodes(input, renderer),
        _ => ControlFlow::Poll,
    }
}

fn device_events(event: DeviceEvent, renderer: &mut Renderer) -> ControlFlow {
    match event {
        DeviceEvent::MouseMotion { delta } => handle_cursor((delta.0 as f32, delta.1 as f32), renderer),
        _ => ControlFlow::Poll,
    }
}

fn resize(size: PhysicalSize<u32>, renderer: &mut Renderer) -> ControlFlow {
    renderer.context.resize(size);
    let (width, height) = (size.cast().width, size.cast().height);
    renderer.camera.update_viewport(width, height);
    unsafe { gl::Viewport(0, 0, width as i32, height as i32) }
    ControlFlow::Poll
}

fn handle_cursor(delta: (f32, f32), renderer: &mut Renderer) -> ControlFlow {
    let (xpos, ypos) = delta;
    renderer.camera.euler_update(xpos, -ypos);
    ControlFlow::Poll
}

fn handle_keycodes(input: KeyboardInput, renderer: &mut Renderer) -> ControlFlow {
    if let glutin::event::ElementState::Released = input.state {
        return ControlFlow::Poll;
    }
    if let Some(keycode) = input.virtual_keycode {
        match keycode {
            VirtualKeyCode::Escape | VirtualKeyCode::Q => return ControlFlow::Exit,
            VirtualKeyCode::J => renderer.camera.zoom_out(),
            VirtualKeyCode::K => renderer.camera.zoom_in(),
            VirtualKeyCode::W => renderer.camera.forward(),
            VirtualKeyCode::A => renderer.camera.left(),
            VirtualKeyCode::S => renderer.camera.backward(),
            VirtualKeyCode::D => renderer.camera.right(),
            _ => (),
        }
    };
    ControlFlow::Poll
}
