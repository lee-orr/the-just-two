use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderType},
};

pub type ToonMaterialPlugin = MaterialPlugin<ToonMaterial>;

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "dfaf271e-ec36-4fdd-a17d-0c0c79964926"]
pub struct ToonMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Option<Handle<Image>>,

    #[texture(2)]
    #[sampler(3)]
    pub shadow_texture: Option<Handle<Image>>,
}

impl Material for ToonMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "toon_shader.wgsl".into()
    }
}
