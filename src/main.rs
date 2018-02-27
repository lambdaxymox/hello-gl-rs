extern crate glfw;
extern crate gl;
extern crate tga;

mod util;

use glfw::{Glfw, Action, Context, Key};
use gl::types::{
    GLsizeiptr, GLenum, GLuint, GLint, GLfloat, GLushort, GLchar
};
use std::mem;
use std::ffi::CString;
use std::os::raw;


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
    program: GLuint,
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

fn make_texture(filename: &str) -> GLuint {
    let (pixels, height, width) = match util::read_tga(filename) {
        Ok(tuple) => tuple,
        Err(_) => return 0,
    };
    let mut texture = 0;
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
        gl::TexImage2D(
            gl::TEXTURE_2D, 0,
            gl::RGB8 as GLint,
            width as GLint, height as GLint, 0,
            gl::BGR, gl::UNSIGNED_BYTE,
            pixels
        );
    }

    texture
}

fn make_shader(shader_type: GLenum, filename: &str) -> GLuint {
    let source = match util::file_contents(filename) {
        Ok(val) => val,
        Err(_) => return 0,
    };
    
    let mut shader_ok = 0;
    let length = [source.len() as GLint];
    let source_ptr = source.as_ptr() as *const *const GLchar;
    let length_ptr = &length as *const GLint;
    unsafe {
        let shader = gl::CreateShader(shader_type);
        println!("Creating shader.");
        gl::ShaderSource(shader, 1, source_ptr, length_ptr);
        println!("Shader created.");
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

        shader
    }
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

    println!("Loading vertex shader.");
    // Make shaders.
    let vertex_shader = make_shader(gl::VERTEX_SHADER, "src/shaders/hello-gl.vertex.glsl");
    if vertex_shader == 0 {
        return None;
    }
    println!("Vertex shader loaded.");
    let fragment_shader = make_shader(gl::FRAGMENT_SHADER, "src/shaders/hello-gl.fragment.glsl");
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
        program: program,
        textures: textures,
        uniforms: uniforms,
        attributes: attributes,
        fade_factor: fade_factor,
    })
}

fn render(window: &mut glfw::Window, g_resources: &GResources) {
    unsafe {
        gl::UseProgram(g_resources.program);
        gl::Uniform1f(g_resources.uniforms.fade_factor, g_resources.fade_factor);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, g_resources.textures[0]);
        gl::Uniform1i(g_resources.uniforms.textures[0], 0);

        gl::ActiveTexture(gl::TEXTURE1);
        gl::BindTexture(gl::TEXTURE_2D, g_resources.textures[1]);
        gl::Uniform1i(g_resources.uniforms.textures[1], 1);

        gl::BindBuffer(gl::ARRAY_BUFFER, g_resources.vertex_buffer);
        gl::VertexAttribPointer(
            g_resources.attributes.position as GLuint,
            2,
            gl::FLOAT,
            gl::FALSE,
            (mem::size_of::<GLfloat>() * 2) as GLint,
            0 as *const raw::c_void
        );
        gl::EnableVertexAttribArray(g_resources.attributes.position as GLuint);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, g_resources.element_buffer);
        gl::DrawElements(
            gl::TRIANGLE_STRIP,
            4,
            gl::UNSIGNED_SHORT,
            0 as *const raw::c_void
        );

        gl::DisableVertexAttribArray(g_resources.attributes.position as GLuint);
    }
    window.swap_buffers();
}

fn update_fade_factor(glfw: &Glfw, g_resources: &mut GResources) {
    let milliseconds = glfw.get_time() as f32;
    g_resources.fade_factor = f32::sin(milliseconds * 0.001) * 0.5 + 0.5;
}

fn main() {
    // Initialize our resources.
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(400, 300, "Hello GL!", glfw::WindowMode::Windowed)
                                   .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);

    // Load the OpenGl function pointers.
    gl::load_with(|symbol| { window.get_proc_address(symbol) as *const _ });

    let mut g_resources = make_resources().unwrap();

    // Loop until the user closes the window
    while !window.should_close() {
        render(&mut window, &g_resources);
        update_fade_factor(&glfw, &mut g_resources);

        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            handle_window_event(&mut window, event);
        }
    }
}

