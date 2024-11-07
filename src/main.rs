
use anyhow::Result;
use glam::{vec3, Mat4, Vec3};
use glium::{
    glutin::surface::WindowSurface, implement_vertex, index::PrimitiveType, program, uniform, winit::{
        application::ApplicationHandler,
        event::WindowEvent,
        event_loop::{ActiveEventLoop, EventLoop},
        window::WindowId,
    }, DrawParameters, Surface
};

struct AppState {
    display: glium::backend::glutin::Display<WindowSurface>,
    window: glium::winit::window::Window,
    program: glium::Program,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u16>,
    matrix: Mat4,
    color: Vec3,
    draw_parameters: DrawParameters<'static>,
}

impl ApplicationHandler for AppState {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}
    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::RedrawRequested => {
                let mut frame = self.display.draw();
                self.color.y += 0.001;
                let matrix: [[f32; 4]; 4] = self.matrix.to_cols_array_2d();
                let color: [f32; 3] = self.color.into();
                let uniforms = uniform! {matrix: matrix, uColor: color};

                frame
                    .draw(
                        &self.vertex_buffer,
                        &self.index_buffer,
                        &self.program,
                        &uniforms,
                        &self.draw_parameters,
                    )
                    .unwrap();
                frame.finish().unwrap();
                self.window.request_redraw();
            }
            WindowEvent::Resized(_size) => {}
            _ => (),
        }
    }
}

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}
implement_vertex!(Vertex, position, color);

fn main() -> Result<()> {
    let event_loop = EventLoop::builder().build().unwrap();

    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    let vertex_buffer = glium::VertexBuffer::new(
        &display,
        &[
            Vertex {
                position: [-0.5, -0.5],
                color: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [0.0, 0.5],
                color: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5],
                color: [1.0, 0.0, 0.0],
            },
        ],
    )
    .unwrap();
    let index_buffer =
        glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList, &[0u16, 1, 2]).unwrap();

    let program: Result<glium::Program, glium::program::ProgramChooserCreationError> = program!(&display, 450 => {
        vertex: "
            #version 450

            uniform mat4 matrix;
            uniform vec3 uColor;

            in vec2 position;
            in vec3 color;

            out vec3 vColor;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0) * matrix;
                vColor = color + uColor;
            }
        ",
        fragment: "
            #version 450
            in vec3 vColor;

            out vec4 FragColor;

            void main() {
                FragColor = vec4(vColor, 1.0);
            }
        ",
        },
    );

    let program = program?;

    let matrix = Mat4::IDENTITY;
    let color = Vec3::new(1.0, 0.0, 0.0);

    let draw_parameters = DrawParameters::default();
    let mut app_state = AppState {
        display,
        window,
        vertex_buffer,
        index_buffer,
        program,
        matrix,
        color,
        draw_parameters,
    };

    event_loop.run_app(&mut app_state)?;

    Ok(())
}
