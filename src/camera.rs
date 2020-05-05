extern crate nalgebra_glm as glm;

pub struct Camera {
    pub position: glm::Vec3,
    pub front: glm::Vec3,
    pub up: glm::Vec3,
    pub view: glm::Mat4,
    pub projection: glm::Mat4,
    pub speed: f32,
    pub direction: glm::Vec3,
    fov: f32,
    yaw: f32,
    pitch: f32,
    height: f32,
    width: f32,
    pub init_mouse: bool,
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        let cameraPos = glm::vec3(0.0, 0.0, 3.0);
        let cameraFront = glm::vec3(0.0, 0.0, -1.0);
        let cameraUp = glm::vec3(0.0, 1.0, 0.0);
        let view = glm::look_at(&cameraPos, &(cameraPos + cameraFront), &cameraUp);
        let yaw = -90.0;
        let pitch = 0.0;
        let yaw_rad = (yaw as f64).to_radians() as f32;
        let pitch_rad = (pitch as f64).to_radians() as f32;
        let direction = glm::vec3(yaw_rad.cos() * pitch_rad.cos(), pitch_rad.sin(), yaw_rad.sin() * pitch_rad.cos());
        let projection = glm::perspective(width / height, (45.0 as f64).to_radians() as f32, 0.1, 100.0);
        Self {
            position: cameraPos,
            front: cameraFront,
            up: cameraUp,
            direction,
            width,
            height,
            projection,
            view,
            speed: 0.05,
            fov: 45.0,
            yaw,
            pitch,
            init_mouse: true,
        }
    }

    pub fn euler_update(&mut self, xoffset: f32, yoffset: f32) {
        let sensitivity = 0.2;

        self.yaw += xoffset * sensitivity;
        self.pitch += yoffset * sensitivity;

        if self.pitch > 89.0 {
            self.pitch = 89.0;
        } else if self.pitch < -89.0 {
            self.pitch = -89.0;
        }

        let yaw_rad = (self.yaw as f64).to_radians() as f32;
        let pitch_rad = (self.pitch as f64).to_radians() as f32;
        self.direction = glm::vec3(yaw_rad.cos() * pitch_rad.cos(), pitch_rad.sin(), yaw_rad.sin() * pitch_rad.cos());
        self.front = self.direction.normalize();
        self.update_view();
    }

    pub fn forward(&mut self) {
        self.position += self.speed * self.front;
        self.update_view();
    }

    pub fn backward(&mut self) {
        self.position -= self.speed * self.front;
        self.update_view();
    }

    pub fn left(&mut self) {
        self.position -= self.front.cross(&self.up).normalize() * self.speed;
        self.update_view();
    }

    pub fn right(&mut self) {
        self.position += self.front.cross(&self.up).normalize() * self.speed;
        self.update_view();
    }

    pub fn update_viewport(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        self.update_projection();
    }

    pub fn zoom_in(&mut self) {
        if self.fov < 65.0 {
            self.fov += 1.0;
            self.update_projection();
        }
    }

    pub fn zoom_out(&mut self) {
        if self.fov > 1.0 {
            self.fov -= 1.0;
            self.update_projection();
        }
    }

    fn update_projection(&mut self) {
        self.projection = glm::perspective(self.width / self.height, (self.fov as f64).to_radians() as f32, 0.1, 100.0);
    }

    pub fn update_speed(&mut self, deltaTime: f32) {
        self.speed = 6.0 * deltaTime;
    }

    fn update_view(&mut self) {
        self.view = glm::look_at(&self.position, &(self.position + self.front), &self.up);
    }
}
