use anyhow::Result;
use glium::{
    implement_vertex,
    index::PrimitiveType,
    program, uniform,
    winit::{self, event_loop::EventLoop},
    DrawParameters, Surface,
};

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}
implement_vertex!(Vertex, position, color);

fn main() -> Result<()>{
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

    let program: Result<glium::Program, glium::program::ProgramChooserCreationError> = program!(&display, 100 => {
        vertex: "
            #version 100

            uniform lowp mat4 matrix;

            attribute lowp vec2 position;
            attribute lowp vec3 color;

            varying lowp vec3 vColor;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0) * matrix;
                vColor = color;
            }
        ",
        fragment: "
            #version 100
            varying lowp vec3 vColor;

            void main() {
                gl_FragColor = vec4(vColor, 1.0);
            }
        ",
        },
    );
    let program = program?;
    let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ]

    };

    let draw_parameters = DrawParameters::default();
    loop {
        let mut frame = display.draw();

        frame
            .draw(
                &vertex_buffer,
                &index_buffer,
                &program,
                &uniforms,
                &draw_parameters,
            )
            .unwrap();
        frame.finish().unwrap();
    }
}
