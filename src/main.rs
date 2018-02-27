extern crate glfw;
extern crate gl;
extern crate tga;

mod util;

use glfw::{Action, Context, Key};
use gl::types::{
    GLsizeiptr, GLenum, GLuint, GLint, GLsizei, GLfloat, GLushort, GLchar
};
use std::mem;
use std::ffi::CString;


struct Uniforms {
    fade_factor: GLint,
    textures: [GLint; 2],
}

struct Attributes {
    position: GLint,
}

struct GResources {
    vertex_buffer: GLuint,
    element_buffer: GLuint,
    textures: [GLuint; 2],
    uniforms: Uniforms,
    attributes: Attributes,
    fade_factor: GLfloat,
}

/*
 *  Data used to seed our vertex array and element array buffers.
 */
const G_VERTEX_BUFFER_DATA: [GLfloat; 8] = [
    -1.0, -1.0,
     1.0, -1.0,
    -1.0,  1.0,
     1.0,  1.0
];

const G_ELEMENT_BUFFER_DATA: [GLushort; 4] = [0, 1, 2, 3];

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}

fn render(window: &mut glfw::Window) {
    unsafe {    
        gl::ClearColor(0.0, 0.0, 0.0, 0.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
    // Swap front and back buffers
    window.swap_buffers();
}


fn make_buffer_glfloat(target: GLenum, buffer_data: &[GLfloat]) -> GLuint {
    let mut buffer = 0;
    let buffer_size = (buffer_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr;

    unsafe {
        gl::GenBuffers(1, &mut buffer);
        gl::BindBuffer(target, buffer);
        gl::BufferData(
            target,
            buffer_size,
            mem::transmute(&buffer_data[0]),
            gl::STATIC_DRAW
        );
    }

    buffer
}

fn make_buffer_glushort(target: GLenum, buffer_data: &[GLushort]) -> GLuint {
    let mut buffer = 0;
    let buffer_size = (buffer_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr;

    unsafe {
        gl::GenBuffers(1, &mut buffer);
        gl::BindBuffer(target, buffer);
        gl::BufferData(
            target,
            buffer_size,
            mem::transmute(&buffer_data[0]),
            gl::STATIC_DRAW
        );
    }

    buffer
}

fn make_resources() -> Option<GResources> {
    // Make buffers.
    let vertex_buffer = make_buffer_glfloat(
        gl::ARRAY_BUFFER,
        &G_VERTEX_BUFFER_DATA
    );
    let element_buffer = make_buffer_glushort(
        gl::ELEMENT_ARRAY_BUFFER,
        &G_ELEMENT_BUFFER_DATA
    );
    // Make textures.
    let mut textures = [0; 2];
    textures[0] = make_texture("assets/hello1.tga");
    textures[1] = make_texture("assets/hello2.tga");

    if textures[0] == 0 || textures[1] == 0 {
        return None;
    }

    // Make shaders.
    let vertex_shader = make_shader(gl::VERTEX_SHADER, "hello-gl.vertex.glsl");
    if vertex_shader == 0 {
        return None;
    }

    let fragment_shader = make_shader(gl::FRAGMENT_SHADER, "hello-gl.fragment.glsl");
    if fragment_shader == 0 {
        return None;
    }

    let program = make_program(vertex_shader, fragment_shader);
    if program == 0 {
        return None;
    }

    let fade_factor_cstr = CString::new("fade_factor").unwrap();
    let textures_0_cstr = CString::new("textures[0]").unwrap();
    let textures_1_cstr = CString::new("textures[1]").unwrap();
    let uniforms = Uniforms {
        fade_factor: unsafe { gl::GetUniformLocation(program, fade_factor_cstr.as_ptr()) },
        textures: [
            unsafe { gl::GetUniformLocation(program, textures_0_cstr.as_ptr()) },
            unsafe { gl::GetUniformLocation(program, textures_1_cstr.as_ptr()) },
        ],
    };

    let position_cstr = CString::new("position").unwrap();
    let attributes = Attributes {
        position: unsafe { gl::GetAttribLocation(program, position_cstr.as_ptr()) },
    };

    let fade_factor = 0.0;

    Some(GResources {
        vertex_buffer: vertex_buffer,
        element_buffer: element_buffer,
        textures: textures,
        uniforms: uniforms,
        attributes: attributes,
        fade_factor: fade_factor,
    })
}

fn make_texture(filename: &str) -> GLuint {
    let (pixels, height, width) = match util::read_tga(filename) {
        Ok(tuple) => tuple,
        Err(_) => return 0,
    };
    let mut texture = 0;
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexImage2D(
            gl::TEXTURE_2D, 0,
            gl::RGB8 as i32,
            width as i32, height as i32, 0,
            gl::BGR, gl::UNSIGNED_BYTE,
            pixels
        );
    }

    texture
}

fn make_shader(shader_type: GLenum, filename: &str) -> GLuint {
    let (source, length) = match util::file_contents(filename) {
        Ok(tuple) => tuple,
        Err(_) => return 0,
    };

    let mut shader_ok = 0;
    let length = length as i32;
    let source_ptr = source.as_ptr() as *const *const GLchar;
    let length_ptr = &length;
    let shader = unsafe { gl::CreateShader(shader_type) };
    unsafe {
        gl::ShaderSource(shader, 1, source_ptr, length_ptr);
        gl::CompileShader(shader);
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut shader_ok);
    
        if shader_ok == 0 {
            eprintln!("Failed to compile {}", filename);
            // BEGIN show_info_log.
            let mut log_length = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_length);
            let log: Vec<i8> = Vec::with_capacity(log_length as usize);
            gl::GetShaderInfoLog(shader, log_length, &mut 0, log.as_ptr() as *mut i8);
            eprintln!("{:?}", log);
            // END show_info_log.
            gl::DeleteShader(shader);
        
            return 0;
        }
    }
    
    shader
}

fn make_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
    let mut program_ok: GLint = 0;
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut program_ok);

        if program_ok == 0 {
            eprintln!("Failed to link shader program:");
            // BEGIN show_info_log.
            let mut log_length = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut log_length);
            let log: Vec<i8> = Vec::with_capacity(log_length as usize);
            gl::GetShaderInfoLog(program, log_length, &mut 0, log.as_ptr() as *mut i8);
            eprintln!("{:?}", log);
            // END show_info_log.
            gl::DeleteProgram(program);

            return 0;
        }

        program
    }
}


fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(400, 300, "Hello GL!", glfw::WindowMode::Windowed)
                                   .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);

    // Load the OpenGl function pointers.
    gl::load_with(|symbol| { window.get_proc_address(symbol) as *const _ });

    // Loop until the user closes the window
    while !window.should_close() {
        render(&mut window);

        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            handle_window_event(&mut window, event);
        }
    }
}

