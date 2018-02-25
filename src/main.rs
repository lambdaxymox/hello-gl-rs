extern crate glfw;
extern crate gl;

use glfw::{Action, Context, Key};
use gl::types::GLfloat;


#[derive(Copy, Clone, Debug)]
struct Color {
    r: GLfloat,
    g: GLfloat,
    b: GLfloat,
    a: GLfloat,
}

impl Color {
    fn new() -> Color {
        Color {
            r: 0.0, g: 0.0, b: 0.0, a: 1.0
        }
    }

    fn from_colors(r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat) -> Color {
        Color {
            r: r, g: g, b: b, a: a
        }
    }
}

struct Screen {
    color: Color,
}

impl Screen {
    fn new(color: Color) -> Screen {
        Screen {
            color: color,
        }
    }

    fn update(&mut self, interval: GLfloat) {
        self.color.b += interval;
        if self.color.b > 1.0 {
            self.color.b = 0.0;
            self.color.g += interval;
        }
        if self.color.g > 1.0 {
            self.color.g = 0.0;
            self.color.r += interval;
        }
        if self.color.r > 1.0 {
            self.color.r = 0.0;
            self.color.g = 0.0;
            self.color.b = 0.0;
        }
    }
}

#[inline]
unsafe fn update_screen(screen: &mut Screen) {
    screen.update(0.1);
    gl::ClearColor(
        screen.color.r, screen.color.g, screen.color.b, screen.color.a
    );
    gl::Clear(gl::COLOR_BUFFER_BIT);
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
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(640, 480, "Hello, GL!", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);

    // Load the OpenGl function pointers.
    gl::load_with(|symbol| { window.get_proc_address(symbol) as *const _ });

    let mut screen = Screen:: new(Color::new());

    // Loop until the user closes the window
    while !window.should_close() {
        // Swap front and back buffers
        window.swap_buffers();

        unsafe {
            update_screen(&mut screen);
        }
        
        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            handle_window_event(&mut window, event);
        }
    }
}

