#![allow(clippy::type_complexity)]
pub mod generated;

use bevy_tokio_tasks::TokioTasksRuntime;
use futures_util::{future, pin_mut, SinkExt, StreamExt};
use std::str::FromStr;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::{HeaderValue, Uri};
use valence::math::IVec3;
use valence::message::ChatMessageEvent;
use valence::prelude::*;

const SPAWN_POS: BlockPos = BlockPos::new(0, 100, 0);
#[derive(Component)]
pub struct GameServerConnection {
    pub send_tx: futures_channel::mpsc::UnboundedSender<String>,
    pub close_tx: futures_channel::mpsc::UnboundedSender<i64>,
    pub entity_id: Entity,
}

impl GameServerConnection {
    fn new(
        runtime: &mut bevy_tokio_tasks::TokioTasksRuntime,
        mq: &mut MessageQueue<QueueMessage>,
        entity_id: Entity,
        username: String,
    ) -> Self {
        println!("creating new websocket connection!");
        let (send_tx, send_rx) = futures_channel::mpsc::unbounded::<String>();
        let (close_tx, mut close_rx) = futures_channel::mpsc::unbounded::<i64>();
        let sender = mq.sender.clone();
        let other_sender = send_tx.clone();
        runtime.spawn_background_task(move |_ctx| async move {
            println!("This task is running on a background thread");

            let uri = Uri::from_str("wss://beta.rollycubes.com/ws/room/9UkkHT").unwrap();
            let mut request = uri.into_client_request().unwrap();
            request.headers_mut().insert(
                "Origin",
                HeaderValue::from_str("beta.rollycubes.com").unwrap(),
            );
            request
                .headers_mut()
                // TODO: dont hardcode session
                .insert(
                    "Cookie",
                    HeaderValue::from_str(&format!("_session=minecrafter:{}", username)).unwrap(),
                );

            let (ws_stream, _) = connect_async(request).await.expect("Failed to connect");
            println!("WebSocket handshake has been successfully completed");

            let (write, read) = ws_stream.split();
            let stdin_to_ws = send_rx
                .map(|m| Ok(tokio_tungstenite::tungstenite::protocol::Message::Text(m)))
                .forward(write);
            other_sender
                .unbounded_send(
                    serde_json::to_string(&generated::UpdateNameMsg {
                        name: username,
                        ..Default::default()
                    })
                    .unwrap(),
                )
                .unwrap();
            let ws_to_stdout = {
                read.for_each(|message| async {
                    let data = message.unwrap().into_text().unwrap();
                    let msg: generated::ServerMsg = serde_json::from_str(data.as_str()).unwrap();
                    println!("{msg:?}");
                    sender.send(QueueMessage { msg, entity_id }).unwrap();
                })
            };
            let n = close_rx.next();
            pin_mut!(stdin_to_ws, ws_to_stdout, n);
            future::select(n, future::select(stdin_to_ws, ws_to_stdout)).await;
            println!("rip websocket");
        });
        Self {
            send_tx,
            entity_id,
            close_tx,
        }
    }
    fn close(&mut self) {
        println!("closing websocket connection");
    }
    pub fn send<T: serde::Serialize>(&mut self, thing: T) {
        let text = serde_json::to_string(&thing).unwrap();
        self.send_tx.unbounded_send(text).unwrap();
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
                handle_websocket_events,
                chat_event_handler,
                despawn_disconnected_clients,
            ),
        )
        .run();
}

fn chat_event_handler(
    mut clients: Query<(&Username, &Properties, &UniqueId, &mut GameServerConnection)>,
    mut messages: EventReader<ChatMessageEvent>,
) {
    for ChatMessageEvent {
        client, message, ..
    } in messages.iter()
    {
        let Ok((_, _, _, mut connection)) = clients.get_mut(*client) else {
            continue;
        };

        connection.send(generated::ChatMsg {
            msg: message.to_string(),
            ..Default::default()
        });
    }
}
#[derive(Resource)]
pub struct MessageQueue<T> {
    pub sender: std::sync::mpsc::Sender<T>,
    pub receiver: std::sync::Mutex<std::sync::mpsc::Receiver<T>>,
}

pub struct QueueMessage {
    msg: generated::ServerMsg,
    entity_id: Entity,
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

    for z in -5..6 {
        for x in -5..6 {
            for y in -5..6 {
                if i32::abs(x) < 5 && i32::abs(y) < 5 && i32::abs(z) < 5 {
                    continue;
                }
                layer
                    .chunk
                    .set_block(SPAWN_POS + IVec3 { x, y, z }, BlockState::GLASS);
            }
        }
    }

    commands.spawn(layer);
}

fn handle_websocket_events(
    mut mq: ResMut<MessageQueue<QueueMessage>>,
    mut query: Query<&mut Client>,
) {
    let receiver = mq.receiver.get_mut().unwrap();
    while let Ok(msg) = receiver.try_recv() {
        match msg.msg {
            generated::ServerMsg::ErrorMsg(_) => {}
            generated::ServerMsg::RoomListMsg(_) => {}
            generated::ServerMsg::RedirectMsg(_) => {}
            generated::ServerMsg::RefetchPlayerMsg(_) => {}
            generated::ServerMsg::WelcomeMsg(_) => {}
            generated::ServerMsg::RestartMsg(_) => {}
            generated::ServerMsg::SpectatorsMsg(_) => {}
            generated::ServerMsg::WinMsg(_) => {}
            generated::ServerMsg::RollMsg(_) => {}
            generated::ServerMsg::RollAgainMsg(_) => {}
            generated::ServerMsg::JoinMsg(_) => {}
            generated::ServerMsg::DisconnectMsg(_) => {}
            generated::ServerMsg::ReconnectMsg(_) => {}
            generated::ServerMsg::KickMsg(_) => {}
            generated::ServerMsg::ChatMsg(m) => {
                let mut client = query.get_mut(msg.entity_id).unwrap();
                client.send_chat_message(m.msg);
            }
            generated::ServerMsg::UpdateTurnMsg(_) => {}
            generated::ServerMsg::UpdateNameMsg(_) => {}
            generated::ServerMsg::UpdateMsg(_) => {}
        }
    }
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
    mut mq: ResMut<MessageQueue<QueueMessage>>,
    mut commands: Commands,
    mut clients: Query<
        (
            &mut EntityLayerId,
            &mut VisibleChunkLayer,
            &mut VisibleEntityLayers,
            &mut Position,
            &mut GameMode,
            &Username,
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
        username,
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
        *game_mode = GameMode::Adventure;
        commands.entity(entity).insert(GameServerConnection::new(
            &mut runtime,
            &mut mq,
            entity,
            username.to_string(),
        ));
    }
}
