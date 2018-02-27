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
use std::ptr;
use std::env;


/*
 *  Data used to seed our vertex array and element array buffers.
 */
const G_VERTEX_BUFFER_DATA: [GLfloat; 16] = [
    -1.0, -1.0, 0.0, 1.0,
     1.0, -1.0, 0.0, 1.0,
    -1.0,  1.0, 0.0, 1.0,
     1.0,  1.0, 0.0, 1.0
];

const G_ELEMENT_BUFFER_DATA: [GLushort; 4] = [0, 1, 2, 3];

const G_DEFAULT_VERTEX_SHADER: &str = "src/shaders/hello-gl.vertex.glsl";
const G_DEFAULT_FRAGMENT_SHADER: &str = "src/shaders/hello-gl.fragment.glsl";

struct Uniforms {
    timer: GLint,
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
    timer: GLfloat,
}

impl GResources {
    fn cleanup(&mut self) {

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

    unsafe {
        let mut shader_ok = 0;
        let shader = gl::CreateShader(shader_type);
        gl::ShaderSource(shader, 1, &source.as_ptr(), ptr::null());
        gl::CompileShader(shader);
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut shader_ok);

        if shader_ok == 0 {
            eprintln!("Failed to compile {}", filename);
            // BEGIN show_info_log.
            let mut log_length = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_length);
            let log: Vec<i8> = Vec::with_capacity(log_length as usize);
            gl::GetShaderInfoLog(shader, log_length, &mut 0, log.as_ptr() as *mut GLchar);
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

fn make_resources(
    vertex_shader_file: &str, fragment_shader_file: &str
) -> Option<GResources> {
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
    let vertex_shader = make_shader(gl::VERTEX_SHADER, vertex_shader_file);
    if vertex_shader == 0 {
        return None;
    }

    let fragment_shader = make_shader(gl::FRAGMENT_SHADER, fragment_shader_file);
    if fragment_shader == 0 {
        return None;
    }

    let program = make_program(vertex_shader, fragment_shader);
    if program == 0 {
        return None;
    }
    
    let timer_cstr = CString::new("timer").unwrap();
    let textures_0_cstr = CString::new("textures[0]").unwrap();
    let textures_1_cstr = CString::new("textures[1]").unwrap();
    let uniforms = unsafe { Uniforms {
        timer: gl::GetUniformLocation(program, timer_cstr.as_ptr()),
        textures: [
            gl::GetUniformLocation(program, textures_0_cstr.as_ptr()),
            gl::GetUniformLocation(program, textures_1_cstr.as_ptr()),
        ],
    }};

    let position_cstr = CString::new("position").unwrap();
    let attributes = unsafe { Attributes {
        position: gl::GetAttribLocation(program, position_cstr.as_ptr()),
    }};

    let timer = 0.0;

    Some(GResources {
        vertex_buffer: vertex_buffer,
        element_buffer: element_buffer,
        program: program,
        textures: textures,
        uniforms: uniforms,
        attributes: attributes,
        timer: timer,
    })
}

fn render(window: &mut glfw::Window, g_resources: &GResources) {
    unsafe {
        gl::ClearColor(0.1, 0.1, 0.1, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        gl::UseProgram(g_resources.program);
        gl::Uniform1f(g_resources.uniforms.timer, g_resources.timer);
        
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, g_resources.textures[0]);
        gl::Uniform1i(g_resources.uniforms.textures[0], 0);

        gl::ActiveTexture(gl::TEXTURE1);
        gl::BindTexture(gl::TEXTURE_2D, g_resources.textures[1]);
        gl::Uniform1i(g_resources.uniforms.textures[1], 1);

        gl::BindBuffer(gl::ARRAY_BUFFER, g_resources.vertex_buffer);
        gl::VertexAttribPointer(
            g_resources.attributes.position as GLuint,
            4,
            gl::FLOAT,
            gl::FALSE,
            (4 * mem::size_of::<GLfloat>()) as GLint,
            ptr::null()
        );
        gl::EnableVertexAttribArray(g_resources.attributes.position as GLuint);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, g_resources.element_buffer);
        gl::DrawElements(
            gl::TRIANGLE_STRIP,
            4,
            gl::UNSIGNED_SHORT,
            ptr::null()
        );

        gl::DisableVertexAttribArray(g_resources.attributes.position as GLuint);
    }
    window.swap_buffers();
}

fn update_timer(glfw: &Glfw, g_resources: &mut GResources) {
    let time = glfw.get_time();
    g_resources.timer = 1.1 * time as f32;
}

fn update(glfw: &mut Glfw, g_resources: &mut GResources) {
    update_timer(&glfw, g_resources);

    // Poll for and process events
    glfw.poll_events();
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}

fn main() {
    let mut args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        args.push(String::from(G_DEFAULT_VERTEX_SHADER));
    }

    if args.len() < 3 {
        args.push(String::from(G_DEFAULT_FRAGMENT_SHADER));
    }

    // Initialize our resources.
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(640, 480, "Hello GL!", glfw::WindowMode::Windowed)
                                   .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);

    // Load the OpenGl function pointers.
    gl::load_with(|symbol| { window.get_proc_address(symbol) as *const _ });

    let mut g_resources = make_resources(&args[1], &args[2]).expect("Failed to load resources.");

    // Loop until the user closes the window
    while !window.should_close() {
        render(&mut window, &g_resources);
        update(&mut glfw, &mut g_resources);

        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            handle_window_event(&mut window, event);
        }
    }
    
    g_resources.cleanup();

}

