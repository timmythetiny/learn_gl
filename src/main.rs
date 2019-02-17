#![allow(dead_code)]

extern crate gl;
extern crate glutin;
extern crate log;

mod rendering;

use glutin::{ContextBuilder, Event, EventsLoop, GlContext, GlWindow, WindowBuilder, WindowEvent};
use rendering::*;
use std::ptr;

fn main() {
    let mut events_loop = EventsLoop::new();
    let window_builder = WindowBuilder::new().with_title("Hello World");
    let context_builder = ContextBuilder::new().with_vsync(true);
    let window = GlWindow::new(window_builder, context_builder, &events_loop)
        .expect("Could not open a window.");

    unsafe {
        window
            .make_current()
            .expect("Could not make window current.");
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    }

    #[rustfmt::skip]
    let vertices: [f32; 20] = [
        0.5, 0.5, 1.0, 0.0, 0.0,
        0.5, -0.5, 0.0, 1.0, 0.0,
        -0.5, -0.5, 0.0, 0.0, 1.0,
        -0.5, 0.5, 0.0, 0.0, 0.0
    ];
    #[rustfmt::skip]
    let indices: [u32; 6] = [
        0, 1, 3,
        1, 2, 3
    ];
    let vert_source = include_str!("shaders/basic_vertex.glsl");
    let frag_source = include_str!("shaders/basic_fragment.glsl");
    let vert = Shader::vert(vert_source).unwrap();
    let frag = Shader::frag(frag_source).unwrap();
    let shader = ShaderProgram::new()
        .with(&vert)
        .with(&frag)
        .build()
        .unwrap();

    let vbo = Buffer::new(&vertices).build();
    let ebo = Buffer::new(&indices).kind(BufferKind::ElementArray).build();
    let positions = ArrayPointer::new()
        .shader_attribute(&shader, "position")
        .components(2)
        .stride::<f32>(5);
    let colors = ArrayPointer::new()
        .shader_attribute(&shader, "color")
        .components(3)
        .stride::<f32>(5)
        .offset::<f32>(2);
    let vao = VertexArray::new()
        .buffer(&vbo)
        .buffer(&ebo)
        .pointer(&positions)
        .pointer(&colors)
        .build();

    let mut running = true;
    while running {
        events_loop.poll_events(|event| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => running = false,
                WindowEvent::Resized(logical_size) => {
                    let dpi_factor = window.get_hidpi_factor();
                    window.resize(logical_size.to_physical(dpi_factor));
                }
                _ => {}
            },
            _ => {}
        });

        shader.bind();
        vao.bind();
        unsafe {
            gl::ClearColor(1.0, 0.5, 0.7, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }

        window.swap_buffers().expect("Could not swap backbuffer.");
    }
}
