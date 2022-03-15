use bevy::ecs::system::{lifetimeless::SRes, SystemParamItem};
use bevy::pbr::{MaterialPipeline, SpecializedMaterial};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::{
    color::Color,
    render_asset::{PrepareAssetError, RenderAsset},
    render_resource::{
        BindGroup, BindGroupLayout, RenderPipelineDescriptor, Buffer, BufferUsages, BufferInitDescriptor,
        BindGroupDescriptor, Face, BindGroupEntry, BindingType, BufferBindingType, BufferSize,
        std140::{AsStd140, Std140},
    },
    renderer::RenderDevice
};
use bevy::render::render_resource::{BindGroupLayoutDescriptor, BindGroupLayoutEntry, ShaderStages};
use crate::board::{
    board_element::{BoardElement, ElementEvent, InvalidBoardError},
    player::{BoardPlayer, CoinCarrier}
};

pub fn empty_function(land_event: ElementEvent) -> Result<(), InvalidBoardError> {
    // IT DOES NOTHING!!!!
    Ok(())
}

pub fn pass_function(pass_event: ElementEvent) -> Result<(), InvalidBoardError> {
    if let Ok(mut player) = pass_event.player_query
            .get_component_mut::<BoardPlayer>(pass_event.active_player) {
        if player.spaces_to_move != 0 {
            player.spaces_to_move -= 1;
            if let Ok(element) = pass_event.board_element_query
                    .get_component_mut::<BoardElement>(player.current_space) {
                if element.next_spaces.len() == 1 {
                    player.current_space = *element.next_spaces.get(0)
                        .expect("board has no next space!");
                } else {
                    return Err(InvalidBoardError::MultipleNextSpaces);
                }
            }
        }
    }
    Ok(())
}

// use circle shader
#[derive(TypeUuid, Clone, Debug)]
#[uuid = "2e57b508-ec30-4636-82de-78d7419ec424"]
pub struct CircleMaterial {
    pub inner_color: Color,
    pub outer_color: Color
}

#[derive(Clone, Default, AsStd140)]
pub struct CircleMaterialUniformData {
    inner_color: Vec4,
    outer_color: Vec4
}

#[derive(Debug, Clone)]
pub struct GpuCircleMaterial {
    pub buffer: Buffer,
    pub bind_group: BindGroup,
    pub alpha_mode: AlphaMode,
    pub cull_mode: Option<Face>,
}

impl RenderAsset for CircleMaterial {
    type ExtractedAsset = CircleMaterial;
    type PreparedAsset = GpuCircleMaterial;
    type Param = (
        SRes<RenderDevice>,
        SRes<MaterialPipeline<CircleMaterial>>
    );

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(material: Self::ExtractedAsset, (render_device, pipeline): &mut SystemParamItem<Self::Param>)
        -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let std140 = CircleMaterialUniformData {
            inner_color: material.inner_color.as_linear_rgba_f32().into(),
            outer_color: material.outer_color.as_linear_rgba_f32().into()
        }.as_std140();


        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("circle_material_uniform_buffer"),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            contents: std140.as_bytes(),
        });

        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                },
            ],
            label: Some("circle_material_bind_group"),
            layout: &pipeline.material_layout,
        });

        Ok(GpuCircleMaterial {
            buffer,
             bind_group,
            alpha_mode: AlphaMode::Mask(0.5),
            cull_mode: Some(Face::Back)
        })
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct StandardMaterialKey {
    cull_mode: Option<Face>,
}

impl SpecializedMaterial for CircleMaterial {
    type Key = StandardMaterialKey;

    fn key(material: &<Self as RenderAsset>::PreparedAsset) -> Self::Key {
        StandardMaterialKey {
            cull_mode: material.cull_mode
        }
    }

    fn specialize(key: Self::Key, descriptor: &mut RenderPipelineDescriptor) {
        descriptor.primitive.cull_mode = key.cull_mode;
        if let Some(label) = &mut descriptor.label {
            *label = format!("circle_{}", *label).into();
        }
    }

    fn bind_group(material: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &material.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("circle_material_layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(
                            CircleMaterialUniformData::std140_size_static() as u64,
                        ),
                    },
                    count: None
                }
            ]
        })
    }

    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.get_handle("shader/space_shader.wgsl"))
    }
}

// blue space function
fn blue_space(land_event: ElementEvent) -> Result<(), InvalidBoardError> {
    if let Ok(mut coin_carrier) = land_event.player_query.get_component_mut::<CoinCarrier>(land_event.active_player) {
        // TODO increase by 5 if its the last 5 turns
        coin_carrier.coins += 3;
    }
    Ok(())
}

#[derive(Bundle)]
pub struct BlueSpaceBundle {
    #[bundle]
    material_bundle: MaterialMeshBundle<CircleMaterial>
}

// red space function
fn red_space(land_event: ElementEvent) -> Result<(), InvalidBoardError> {
    if let Ok(mut coin_carrier) = land_event.player_query.get_component_mut::<CoinCarrier>(land_event.active_player) {
        // TODO decrease by 5 if its the last 5 turns
        if coin_carrier.coins < 3 {
            coin_carrier.coins = 0;
        } else {
            coin_carrier.coins -= 3;
        }
    }
    Ok(())
}