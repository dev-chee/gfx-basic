use gfx_basic::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let app = Triangle::new();

    Entry::new(app)
        .window_title("Triangle")
        .window_size([1024, 768])
        .build()?;

    Ok(())
}

struct Triangle {
    name: String,
    version: u32,
    vertex_shader: &'static str,
    fragment_shader: &'static str,
}

impl Triangle {
    fn new() -> Self {
        Self {
            name: String::from("Triangle"),
            version: 1,
            vertex_shader: include_str!("triangle.vert"),
            fragment_shader: include_str!("triangle.frag"),
        }
    }
}

impl Application for Triangle {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> u32 {
        self.version
    }

    fn shader(&self, kind: ShaderKind) -> &str {
        if kind == ShaderKind::Vertex {
            self.vertex_shader
        } else if kind == ShaderKind::Fragment {
            self.fragment_shader
        } else {
            unimplemented!()
        }
    }

    fn start(&mut self, _engine: &mut Engine) {
        //todo!()
    }

    fn event(&mut self, _engine: &mut Engine, _event: &mut Event) {
        //todo!()
    }

    fn update(&mut self, _engine: &mut Engine) {
        //todo!()
    }

    fn render(&mut self, _engine: &mut Engine) {
        //todo!()
    }

    fn late_update(&mut self, _engine: &mut Engine) {
        //todo!()
    }

    fn exit(&mut self, _engine: &mut Engine) {
        //todo!()
    }
}
