use glow::*;

mod rtr;
use rtr::*;

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
    let window = Window::new(&event_loop);

    unsafe {
        // Create a context from a glutin window on non-wasm32 targets
        let (gl, gl_surface, gl_context, shader_version, _window, event_loop) = {
           

            let gl = glow::Context::from_loader_function_cstr(|s| gl_display.get_proc_address(s));



            (
                gl,
                gl_surface,
                gl_context,
                "#version 410",
                window,
                event_loop,
            )
        };

        let vertex_array = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(vertex_array));

        let program = gl.create_program().expect("Cannot create program");

        let (vertex_shader_source, fragment_shader_source) = (
            r#"const vec2 verts[3] = vec2[3](
                vec2(0.5f, 1.0f),
                vec2(0.0f, 0.0f),
                vec2(1.0f, 0.0f)
            );
            out vec2 vert;
            void main() {
                vert = verts[gl_VertexID];
                gl_Position = vec4(vert - 0.5, 0.0, 1.0);
            }"#,
            r#"precision mediump float;
            in vec2 vert;
            out vec4 color;
            void main() {
                color = vec4(vert, 0.5, 1.0);
            }"#,
        );

        let shader_sources = [
            (glow::VERTEX_SHADER, vertex_shader_source),
            (glow::FRAGMENT_SHADER, fragment_shader_source),
        ];

        let mut shaders = Vec::with_capacity(shader_sources.len());

        for (shader_type, shader_source) in shader_sources.iter() {
            let shader = gl
                .create_shader(*shader_type)
                .expect("Cannot create shader");
            gl.shader_source(shader, &format!("{}\n{}", shader_version, shader_source));
            gl.compile_shader(shader);
            if !gl.get_shader_compile_status(shader) {
                panic!("{}", gl.get_shader_info_log(shader));
            }
            gl.attach_shader(program, shader);
            shaders.push(shader);
        }

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }

        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }

        gl.use_program(Some(program));
        gl.clear_color(0.1, 0.2, 0.3, 1.0);

        // We handle events differently between targets

        {
            use glutin::prelude::GlSurface;
            use winit::event::{Event, WindowEvent};
            let _ = event_loop.run(move |event, elwt| {
                if let Event::WindowEvent { event, .. } = event {
                    match event {
                        WindowEvent::CloseRequested => {
                            elwt.exit();
                        }
                        WindowEvent::RedrawRequested => {
                            gl.clear(glow::COLOR_BUFFER_BIT);
                            gl.draw_arrays(glow::TRIANGLES, 0, 3);
                            gl_surface.swap_buffers(&gl_context).unwrap();
                        }
                        _ => (),
                    }
                }
            });
        }
    }
}