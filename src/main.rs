use bevy::prelude::*;
// // This is needed on the old MacBook Air 2011 to set the correct WGPU backend
// use bevy::render::RenderPlugin;
// use bevy::render::settings::*;
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
//mod helper;

#[derive(Component)]
enum RobotState{
    Waiting,
    Sensing,
    Computing,
    Error
}

#[derive(Component)]
struct Position {
    x: u32,
    y: u32
}

#[derive(Component)]
struct RobotMarker;

// This component markiert Wände.
#[derive(Component)]
struct Wall;

#[derive(Event)]
struct RobotMoveEvent;

// #[derive(Res    )]
// struct WorldTile{
//     map: Vec<Vec<u16>>,
// }

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
        // This event signals that the robot moved and thus the camera needs to readjust.
        .add_event::<RobotMoveEvent>()
        // This adds the observer that gets triggert after the map building is finished
        .add_observer(add_robot_component)
        // Add our systems and run the app!
        .add_systems(Startup, startup)
        .add_systems(Update, (make_robot_move,move_camera_above_robot).chain())
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(TiledMapHandle(asset_server.load("map1.tmx")));
}

// fn layer_created(
//     trigger: Trigger<TiledLayerCreated>,
//     mut q_tmtl: Query<(&Name, &mut Transform), With<TiledMapTileLayer>>,
//     mut q_tmtlfts: Query<(&TilemapSize, &Children), With<TiledMapTileLayerForTileset>>,
// ) {
//     if let Ok((name,mut t)) = q_tmtl.get_mut(trigger.event().layer) {
//         if let Ok((tms, c)) = q_tmtlfts.get_single_mut(){
//             //t.translation = Vec3::new(-16.*(tms.x as f32 /2.), -16.*(tms.y as f32 /2.), 0.);
//             info!("Moved TileMap to the middle of the screen");
//         }
//     }
// }
// This startup function adds the robot component to the tile 
// where the robot is. I can be thus accessed via the robot component.
fn add_robot_component(
    _: Trigger<TiledLayerCreated>,
    q_tile: Query<(Entity, &mut TileTextureIndex, &Name, &TilePos), With<TiledMapTile>>,
    mut commands: Commands,
    mut ev_robotmov: EventWriter<RobotMoveEvent>,
) {
    let mut wall_counter : u64 = 0;
    for (entity, 
         textureidx, 
         name,
         tp) in q_tile.iter()
    {
        if textureidx.0 == 4
        {
            commands.entity(entity).insert((
                RobotMarker,
                RobotState::Waiting,
                Position{
                    x: tp.x,
                    y: tp.y
                }));
            info!("Added the Robot component to Tile: {}", name);
        }
        else if textureidx.0 == 2
        {
            wall_counter += 1;
            commands.entity(entity).insert(Wall);
        }
    }
    // Here send the Event that the camera hovers over the robot.
    ev_robotmov.send(RobotMoveEvent);
    info!("Added finish with {} Wall tiles.", wall_counter);
}

fn move_camera_above_robot(
    mut ev_robotmove: EventReader<RobotMoveEvent>,
    mut c_query: Query<&mut Transform, With<Camera>>,
    r_query: Query<&Position, With<RobotMarker>>,
) {
    for _ in ev_robotmove.read() {
        let p = r_query.single();
        let mut c = c_query.single_mut();
        c.translation.x = p.x as f32 * 16.;
        c.translation.y = p.y as f32 * 16.;
        info!("movement recieved");
    }
}

fn make_robot_move(
    mut q_tile: Query<(&mut TileTextureIndex, &Name), With<RobotMarker>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut ev_robotmov: EventWriter<RobotMoveEvent>,
    // This we need to iterate over the child to find the new position.
    mut q_tmtlfts: Query<&Children, With<TiledMapTileLayerForTileset>>,

) {
    if keys.just_pressed(KeyCode::ArrowDown) {
        // Check if down is free.
        let children = q_tmtlfts.get_single_mut().unwrap();
            for c in children.iter() {
                info!("This id: {}", c);
            }
    }

    if keys.just_pressed(KeyCode::Space) {
        let (mut i,n ) =  q_tile.single_mut();
        if i.0 == 4 {
            i.0 = 8;
            info!("The name of the textid 44 is: {}", n);
        }
        else if i.0 == 8 {
            i.0 = 4;
        }
        ev_robotmov.send(RobotMoveEvent);
    }
}