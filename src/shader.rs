extern crate nalgebra_glm as glm;

use gl::types::*;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::str;

#[allow(non_snake_case)]
pub struct Shader {
    pub ID: u32,
}

#[allow(non_snake_case)]
impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Self {
        let mut shader = Self { ID: 0 };
        let mut vshader_file = File::open(vertex_path).unwrap_or_else(|_| panic!("Failed to open {}", vertex_path));
        let mut fshader_file = File::open(fragment_path).unwrap_or_else(|_| panic!("Failed to open {}", fragment_path));
        let mut vertex_code = String::new();
        let mut fragment_code = String::new();
        vshader_file.read_to_string(&mut vertex_code).expect("Failed to read vertex shader");
        fshader_file.read_to_string(&mut fragment_code).expect("Failed to read fragment shader");

        let vshader_code = CString::new(vertex_code.as_bytes()).unwrap();
        let fshader_code = CString::new(fragment_code.as_bytes()).unwrap();

        unsafe {
            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(512);

            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &vshader_code.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
            gl::GetShaderiv(vertex, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(vertex, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
            };

            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &fshader_code.as_ptr(), ptr::null());
            gl::CompileShader(fragment);
            gl::GetShaderiv(fragment, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(fragment, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
            };

            shader.ID = gl::CreateProgram();
            gl::AttachShader(shader.ID, vertex);
            gl::AttachShader(shader.ID, fragment);
            gl::LinkProgram(shader.ID);
            gl::GetProgramiv(shader.ID, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(shader.ID, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::PROGRAM::LINKING_FAILED\n{}", str::from_utf8(&info_log).unwrap());
            }

            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
        }

        shader
    }

    pub unsafe fn useProgram(&self) {
        gl::UseProgram(self.ID);
    }

    pub unsafe fn setBool(&self, name: &str, value: bool) {
        let name = std::ffi::CString::new(name).unwrap();
        gl::Uniform1i(gl::GetUniformLocation(self.ID, name.as_ptr()), value as i32);
    }

    pub unsafe fn setInt(&self, name: &str, value: i32) {
        let name = std::ffi::CString::new(name).unwrap();
        gl::Uniform1i(gl::GetUniformLocation(self.ID, name.as_ptr()), value);
    }

    pub unsafe fn setFloat(&self, name: &str, value: f32) {
        let name = std::ffi::CString::new(name).unwrap();
        gl::Uniform1f(gl::GetUniformLocation(self.ID, name.as_ptr()), value);
    }

    pub unsafe fn setMat4(&self, name: &str, value: &glm::Mat4) {
        let name = std::ffi::CString::new(name).unwrap();
        gl::UniformMatrix4fv(gl::GetUniformLocation(self.ID, name.as_ptr()), 1, gl::FALSE, glm::value_ptr(&value).as_ptr());
    }
}
