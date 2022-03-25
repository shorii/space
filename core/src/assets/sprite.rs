use crate::types::{ComponentProps, EntitySpawner};
use bevy::asset::LoadState;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use std::any::TypeId;
use std::collections::HashMap;

#[derive(Default)]
pub struct TextureAtlasHandles {
    pub map: HashMap<TypeId, Handle<TextureAtlas>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StartupState {
    Setup,
    Loaded,
    Finished,
}

#[derive(Default)]
pub struct SpriteHandles {
    pub handles: Vec<HandleUntyped>,
}

pub fn load_textures(mut sprite_handles: ResMut<SpriteHandles>, asset_server: Res<AssetServer>) {
    sprite_handles.handles = asset_server.load_folder("textures").unwrap();
}

pub fn check_textures(
    mut state: ResMut<State<StartupState>>,
    sprite_handles: ResMut<SpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        state.set(StartupState::Loaded).unwrap();
    }
}

pub fn setup_texture_atlas(
    mut commands: Commands,
    sprite_handles: Res<SpriteHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
    mut texture_atlas_handles: ResMut<TextureAtlasHandles>,
    mut state: ResMut<State<StartupState>>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in sprite_handles.handles.iter() {
        let texture = textures.get(handle).unwrap();
        texture_atlas_builder.add_texture(handle.clone_weak().typed::<Image>(), texture);
    }
    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let handle = texture_atlases.add(texture_atlas);
    texture_atlas_handles
        .map
        .insert(TypeId::of::<TextureAtlas>(), handle);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    state.set(StartupState::Finished).unwrap();
}

pub trait WithSprite {
    fn get_sprite_sheet_bundle(
        asset_server: Res<AssetServer>,
        texture_atlas_handles: Res<TextureAtlasHandles>,
        texture_atlases: Res<Assets<TextureAtlas>>,
        props: impl ComponentProps,
    ) -> Option<SpriteSheetBundle>;
}

pub trait WithSpriteExt {
    fn spawn_with_sprite<'w, 's, 'a>(
        commands: &'a mut Commands<'w, 's>,
        asset_server: Res<AssetServer>,
        texture_atlas_handles: Res<TextureAtlasHandles>,
        texture_atlases: Res<Assets<TextureAtlas>>,
        props: impl ComponentProps,
    ) -> Option<EntityCommands<'w, 's, 'a>>;
}

impl<T> WithSpriteExt for T
where
    T: WithSprite + EntitySpawner,
{
    fn spawn_with_sprite<'w, 's, 'a>(
        commands: &'a mut Commands<'w, 's>,
        asset_server: Res<AssetServer>,
        texture_atlas_handles: Res<TextureAtlasHandles>,
        texture_atlases: Res<Assets<TextureAtlas>>,
        props: impl ComponentProps,
    ) -> Option<EntityCommands<'w, 's, 'a>> {
        let sprite_sheet_bundle = T::get_sprite_sheet_bundle(
            asset_server,
            texture_atlas_handles,
            texture_atlases,
            props.clone(),
        );
        match sprite_sheet_bundle {
            Some(ssb) => {
                let mut entity_commands = T::spawn(commands, props.clone());
                entity_commands.insert_bundle(ssb);
                Some(entity_commands)
            }
            None => None,
        }
    }
}
