use std::time;

use super::{Camera, Ctx, Shader};

pub struct Renderer {
    pub shader: Shader,
    pub camera: Camera,
    pub start_time: time::Instant,
    pub last_frame: f32,
    pub delta_time: f32,
    pub textures: Vec<u32>,
    pub VAO: u32,
    pub context: Ctx,
    pub elements: Vec<glm::Vec3>,
}

impl Renderer {
    pub fn new(shader: Shader, textures: Vec<u32>, VAO: u32, context: Ctx, elements: Vec<glm::Vec3>, width: f32, height: f32) -> Self {
        Self {
            VAO,
            shader,
            textures,
            context,
            elements,
            last_frame: 0.0,
            delta_time: 0.0,
            start_time: get_time(),
            camera: Camera::new(width, height),
        }
    }

    pub fn draw(&mut self) {
        let new_time = self.start_time.elapsed().as_secs_f32();

        self.delta_time = new_time - self.last_frame;
        self.last_frame = new_time;

        self.camera.update_speed(self.delta_time);

        unsafe {
            self.shader.useProgram();
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            self.clear_window();
            self.set_uniforms();
            self.bind_textures();
            gl::BindVertexArray(self.VAO);
            self.draw_elements(new_time as f32);
            gl::BindVertexArray(0);
        }

        self.context.swap_buffers().expect("swap buffers");
    }

    unsafe fn clear_window(&self) {
        gl::ClearColor(27.0 / 255.0, 30.0 / 255.0, 43.0 / 255.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    unsafe fn set_uniforms(&self) {
        self.shader.setInt("texture1", 0);
        self.shader.setInt("texture2", 1);
        self.shader.setMat4("projection", &self.camera.projection);
        self.shader.setMat4("view", &self.camera.view);
    }

    unsafe fn bind_textures(&self) {
        for (i, texture) in self.textures.iter().enumerate() {
            gl::ActiveTexture(gl::TEXTURE0 + i as u32);
            gl::BindTexture(gl::TEXTURE_2D, *texture);
        }
    }

    unsafe fn draw_elements(&self, time: f32) {
        for el in &self.elements {
            let mut model = glm::identity();
            model = glm::translate(&model, el);
            model = glm::rotate(&model, time, &glm::vec3(1.0, 1.0, 0.0));
            self.shader.setMat4("model", &model);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    }
}

fn get_time() -> time::Instant {
    time::Instant::now()
}
