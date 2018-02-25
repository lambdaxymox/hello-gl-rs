extern crate glfw;
extern crate gl;

use glfw::{Action, Context, Key};
use gl::types::{
    GLsizeiptr, GLenum, GLuint, GLsizei, GLfloat, GLushort
};
use std::mem;



/*
 *  Data used to seed our vertex array and element array buffers.
 */
const G_VERTEX_BUFFER_DATA: [GLfloat ; 8] = [
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

fn make_resources() {
    let vertex_buffer = make_buffer_glfloat(
        gl::ARRAY_BUFFER,
        &G_VERTEX_BUFFER_DATA
    );
    let element_buffer = make_buffer_glushort(
        gl::ELEMENT_ARRAY_BUFFER,
        &G_ELEMENT_BUFFER_DATA
    );
    // Make textures and shaders.
}

fn make_texture() -> GLuint {
    // Make texture.
    return 0;
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

