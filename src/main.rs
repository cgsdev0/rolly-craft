#![allow(clippy::type_complexity)]

use futures_util::{future, pin_mut, StreamExt};
use tokio_tungstenite::connect_async;

use std::str::FromStr;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

use bevy_tokio_tasks::TokioTasksRuntime;

use tokio_tungstenite::tungstenite::http::{HeaderValue, Uri};
use valence::prelude::*;

const SPAWN_POS: BlockPos = BlockPos::new(0, 100, -16);

#[derive(Component)]
pub struct GameServerConnection {
    pub close_tx: futures_channel::mpsc::UnboundedSender<i64>,
}

impl GameServerConnection {
    fn new(runtime: &mut bevy_tokio_tasks::TokioTasksRuntime) -> Self {
        println!("creating new websocket connection!");
        let (close_tx, mut close_rx) = futures_channel::mpsc::unbounded();
        runtime.spawn_background_task(|_ctx| async move {
            println!("This task is running on a background thread");

            let uri = Uri::from_str("wss://rollycubes.com/ws/room/xEnPPK").unwrap();
            let mut request = uri.into_client_request().unwrap();
            request
                .headers_mut()
                .insert("Origin", HeaderValue::from_str("rollycubes.com").unwrap());
            request
                .headers_mut()
                .insert("Cookie", HeaderValue::from_str("_session=asdf").unwrap());

            let (ws_stream, _) = connect_async(request).await.expect("Failed to connect");
            println!("WebSocket handshake has been successfully completed");

            let (_, read) = ws_stream.split();

            let ws_to_stdout = {
                read.for_each(|message| async {
                    let data = message.unwrap().into_text().unwrap();
                    println!("{}", data);
                })
            };
            let pin_me = close_rx.next();
            pin_mut!(pin_me, ws_to_stdout);
            future::select(pin_me, ws_to_stdout).await;
            println!("rip websocket");
            close_rx.close();
        });
        Self { close_tx }
    }
    fn close(&mut self) {
        println!("closing websocket connection");
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_tokio_tasks::TokioTasksPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                init_clients,
                disconnect_clients,
                despawn_disconnected_clients,
            ),
        )
        .run();
}

#[derive(Resource)]
pub struct MessageQueue<T> {
    pub sender: std::sync::mpsc::Sender<T>,
    pub receiver: std::sync::Mutex<std::sync::mpsc::Receiver<T>>,
}

pub enum QueueMessage {
    Hello,
}

fn setup(
    mut commands: Commands,
    server: Res<Server>,
    dimensions: Res<DimensionTypeRegistry>,
    biomes: Res<BiomeRegistry>,
) {
    let (tx, rx) = std::sync::mpsc::channel::<QueueMessage>();
    let mq = MessageQueue {
        sender: tx,
        receiver: std::sync::Mutex::new(rx),
    };
    commands.insert_resource(mq);

    let mut layer = LayerBundle::new(ident!("overworld"), &dimensions, &biomes, &server);

    for z in -5..5 {
        for x in -5..5 {
            layer.chunk.insert_chunk([x, z], UnloadedChunk::new());
        }
    }

    layer.chunk.set_block(SPAWN_POS, BlockState::BEDROCK);

    commands.spawn(layer);
}

fn disconnect_clients(
    mut commands: Commands,
    mut clients: Query<(&mut GameServerConnection, Entity), Without<Client>>,
) {
    for (mut conn, entity) in &mut clients {
        conn.close();
        commands.entity(entity).remove::<GameServerConnection>();
    }
}

fn init_clients(
    mut runtime: ResMut<TokioTasksRuntime>,
    mut commands: Commands,
    mut clients: Query<
        (
            &mut EntityLayerId,
            &mut VisibleChunkLayer,
            &mut VisibleEntityLayers,
            &mut Position,
            &mut GameMode,
            Entity,
        ),
        Added<Client>,
    >,
    layers: Query<Entity, (With<ChunkLayer>, With<EntityLayer>)>,
) {
    for (
        mut layer_id,
        mut visible_chunk_layer,
        mut visible_entity_layers,
        mut pos,
        mut game_mode,
        entity,
    ) in &mut clients
    {
        let layer = layers.single();

        layer_id.0 = layer;
        visible_chunk_layer.0 = layer;
        visible_entity_layers.0.insert(layer);
        pos.set([
            SPAWN_POS.x as f64 + 0.5,
            SPAWN_POS.y as f64 + 1.0,
            SPAWN_POS.z as f64 + 0.5,
        ]);
        *game_mode = GameMode::Survival;
        commands
            .entity(entity)
            .insert(GameServerConnection::new(&mut runtime));
    }
}
