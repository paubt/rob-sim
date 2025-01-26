use bevy::{math::vec3, prelude::*};
// // This is needed on the old MacBook Air 2011 to set the correct WGPU backend
// use bevy::render::RenderPlugin;
// use bevy::render::settings::*;
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
//mod helper;


// This component markiert den ROboter. Es kann nur einen davon geben -> darauf wird gebaut.
#[derive(Component)]
struct RoboMarker;

fn main() {
    App::new()
        // Bevy default plugins
        .add_plugins(DefaultPlugins
            // //This is needed on the old MacBook Air 2011 to set the correct WGPU backend
            // .set(RenderPlugin {
            //     render_creation: WgpuSettings {
            //     backends: Some(Backends::VULKAN),
            //     ..default()
            //     }
            //     .into(),
            //     ..default()
            // })
        )
        // Examples helper plugin (does not matter for
        // bevy_ecs_tilemap and bevy_ecs_tiled main plugins
        .add_plugins(TilemapPlugin)
        .add_plugins(TiledMapPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_observer(layer_created)
        // This adds the observer that gets triggert after the map building is finished
        .add_observer(add_robot_component)
        // Add our systems and run the app!
        .add_systems(Startup, startup)
        .add_systems(Update, change_tile)
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(TiledMapHandle(asset_server.load("map.tmx")));
}

fn layer_created(
    trigger: Trigger<TiledLayerCreated>,
    mut q_tmtl: Query<(&Name, &mut Transform), With<TiledMapTileLayer>>,
    mut q_tmtlfts: Query<(&TilemapSize, &Children), With<TiledMapTileLayerForTileset>>,
) {
    if let Ok((name,mut t)) = q_tmtl.get_mut(trigger.event().layer) {
        if let Ok((tms, c)) = q_tmtlfts.get_single_mut(){
            t.translation = Vec3::new(-16.*(tms.x as f32 /2.), -16.*(tms.y as f32 /2.), 0.);
            info!("Moved TileMap to the middle of the screen");
        }
    }
}
// This startup function adds the robot component to the tile 
// where the robot is. I can be thus accessed via the robot component.
fn add_robot_component(
    trigger: Trigger<TiledLayerCreated>,
    q_tile: Query<(Entity, &mut TileTextureIndex, &Name), With<TiledMapTile>>,
    mut commands: Commands
) {
    for (entity, 
         textureidx, 
         name) in q_tile.iter()
    {
        if textureidx.0 == 44
        {
            commands.entity(entity).insert(RoboMarker);
            info!("Added the Robot component to Tile: {}", name);
        }
    }
    info!("Added finish");
}

fn change_tile(
    mut q_tile: Query<(&mut TileTextureIndex, &Name), With<RoboMarker>>,
    keys: Res<ButtonInput<KeyCode>>
) {
    if keys.just_pressed(KeyCode::Space) {
        let (mut i,n ) =  q_tile.single_mut();
        if i.0 == 44 {
            i.0 = 57;
            info!("The name of the textid 44 is: {}", n);
        }
        else if i.0 == 57 {
            i.0 = 44;
        }
        
    }
}

