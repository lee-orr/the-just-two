use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderType},
};

pub type ToonMaterialPlugin = MaterialPlugin<ToonMaterial>;

#[derive(Debug, Clone, ShaderType)]
pub struct ToonValues {
    pub threshold: f32,
    pub shadow_multiplier: f32,
}

// impl ShaderType for ToonValues {}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "dfaf271e-ec36-4fdd-a17d-0c0c79964926"]
pub struct ToonMaterial {
    #[uniform(0)]
    pub values: ToonValues,

    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Option<Handle<Image>>,
}

impl Material for ToonMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "toon_shader.wgsl".into()
    }
}
