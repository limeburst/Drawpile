use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DpMessageType {
    ServerCommand = 0,
    Disconnect = 1,
    Ping = 2,
    KeepAlive = 3,
    Thumbnail = 4,
    Internal = 31,
    Join = 32,
    Leave = 33,
    SessionOwner = 34,
    Chat = 35,
    TrustedUsers = 36,
    SoftReset = 37,
    PrivateChat = 38,
    ResetStream = 39,
    Interval = 64,
    LaserTrail = 65,
    MovePointer = 66,
    RemovedMarker = 67,
    UserAcl = 68,
    LayerAcl = 69,
    FeatureAccessLevels = 70,
    DefaultLayer = 71,
    RemovedFiltered = 72,
    Extension = 73,
    UndoDepth = 74,
    Data = 75,
    LocalChange = 76,
    FeatureLimits = 77,
    UndoPoint = 128,
    CanvasResize = 129,
    RemovedLayerCreate = 130,
    LayerAttributes = 131,
    LayerRetitle = 132,
    RemovedLayerOrder = 133,
    RemovedLayerDelete = 134,
    RemovedLayerVisibility = 135,
    PutImage = 136,
    FillRect = 137,
    RemovedToolChange = 138,
    RemovedPenMove = 139,
    PenUp = 140,
    AnnotationCreate = 141,
    AnnotationReshape = 142,
    AnnotationEdit = 143,
    AnnotationDelete = 144,
    RemovedMoveRegion = 145,
    PutTile = 146,
    CanvasBackground = 147,
    DrawDabsClassic = 148,
    DrawDabsPixel = 149,
    DrawDabsPixelSquare = 150,
    DrawDabsMypaint = 151,
    DrawDabsMypaintBlend = 152,
    MoveRect = 160,
    SetMetadataInt = 161,
    LayerTreeCreate = 162,
    LayerTreeMove = 163,
    LayerTreeDelete = 164,
    TransformRegion = 165,
    TrackCreate = 166,
    TrackRetitle = 167,
    TrackDelete = 168,
    TrackOrder = 169,
    KeyFrameSet = 170,
    KeyFrameRetitle = 171,
    KeyFrameLayerAttributes = 172,
    KeyFrameDelete = 173,
    SelectionPut = 174,
    SelectionClear = 175,
    LocalMatch = 176,
    SyncSelectionTile = 177,
    PutImageZstd = 178,
    PutTileZstd = 179,
    CanvasBackgroundZstd = 180,
    MoveRectZstd = 181,
    TransformRegionZstd = 182,
    Undo = 255,
}

impl DpMessageType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DpMessageType::ServerCommand => "servercommand",
            DpMessageType::Disconnect => "disconnect",
            DpMessageType::Ping => "ping",
            DpMessageType::KeepAlive => "keepalive",
            DpMessageType::Thumbnail => "thumbnail",
            DpMessageType::Internal => "internal",
            DpMessageType::Join => "join",
            DpMessageType::Leave => "leave",
            DpMessageType::SessionOwner => "sessionowner",
            DpMessageType::Chat => "chat",
            DpMessageType::TrustedUsers => "trusted",
            DpMessageType::SoftReset => "softreset",
            DpMessageType::PrivateChat => "privatechat",
            DpMessageType::ResetStream => "resetstream",
            DpMessageType::Interval => "interval",
            DpMessageType::LaserTrail => "lasertrail",
            DpMessageType::MovePointer => "movepointer",
            DpMessageType::RemovedMarker => "removedmarker",
            DpMessageType::UserAcl => "useracl",
            DpMessageType::LayerAcl => "layeracl",
            DpMessageType::FeatureAccessLevels => "featureaccess",
            DpMessageType::DefaultLayer => "defaultlayer",
            DpMessageType::RemovedFiltered => "removedfiltered",
            DpMessageType::Extension => "extension",
            DpMessageType::UndoDepth => "undodepth",
            DpMessageType::Data => "data",
            DpMessageType::LocalChange => "localchange",
            DpMessageType::FeatureLimits => "featurelimits",
            DpMessageType::UndoPoint => "undopoint",
            DpMessageType::CanvasResize => "resize",
            DpMessageType::RemovedLayerCreate => "removedlayercreate",
            DpMessageType::LayerAttributes => "layerattr",
            DpMessageType::LayerRetitle => "retitlelayer",
            DpMessageType::RemovedLayerOrder => "removedlayerorder",
            DpMessageType::RemovedLayerDelete => "removedlayerdelete",
            DpMessageType::RemovedLayerVisibility => "removedlayervisibility",
            DpMessageType::PutImage => "putimage",
            DpMessageType::FillRect => "fillrect",
            DpMessageType::RemovedToolChange => "removedtoolchange",
            DpMessageType::RemovedPenMove => "removedpenmove",
            DpMessageType::PenUp => "penup",
            DpMessageType::AnnotationCreate => "newannotation",
            DpMessageType::AnnotationReshape => "reshapeannotation",
            DpMessageType::AnnotationEdit => "editannotation",
            DpMessageType::AnnotationDelete => "deleteannotation",
            DpMessageType::RemovedMoveRegion => "removedmoveregion",
            DpMessageType::PutTile => "puttile",
            DpMessageType::CanvasBackground => "background",
            DpMessageType::DrawDabsClassic => "classicdabs",
            DpMessageType::DrawDabsPixel => "pixeldabs",
            DpMessageType::DrawDabsPixelSquare => "squarepixeldabs",
            DpMessageType::DrawDabsMypaint => "mypaintdabs",
            DpMessageType::DrawDabsMypaintBlend => "mypaintdabsblend",
            DpMessageType::MoveRect => "moverect",
            DpMessageType::SetMetadataInt => "setmetadataint",
            DpMessageType::LayerTreeCreate => "layertreecreate",
            DpMessageType::LayerTreeMove => "layertreemove",
            DpMessageType::LayerTreeDelete => "layertreedelete",
            DpMessageType::TransformRegion => "transformregion",
            DpMessageType::TrackCreate => "trackcreate",
            DpMessageType::TrackRetitle => "trackretitle",
            DpMessageType::TrackDelete => "trackdelete",
            DpMessageType::TrackOrder => "trackorder",
            DpMessageType::KeyFrameSet => "keyframeset",
            DpMessageType::KeyFrameRetitle => "keyframeretitle",
            DpMessageType::KeyFrameLayerAttributes => "keyframelayerattributes",
            DpMessageType::KeyFrameDelete => "keyframedelete",
            DpMessageType::SelectionPut => "selectionput",
            DpMessageType::SelectionClear => "selectionclear",
            DpMessageType::LocalMatch => "localmatch",
            DpMessageType::SyncSelectionTile => "syncselectiontile",
            DpMessageType::PutImageZstd => "putimagezstd",
            DpMessageType::PutTileZstd => "puttilezstd",
            DpMessageType::CanvasBackgroundZstd => "canvasbackgroundzstd",
            DpMessageType::MoveRectZstd => "moverectzstd",
            DpMessageType::TransformRegionZstd => "transformregionzstd",
            DpMessageType::Undo => "undo",
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct DpMessage {
    length: u16,
    message_type: DpMessageType,
    user_id: u8,
    payload: Vec<u8>,
}

// Overall, the login process is:
// 1. wait for server greeting
// 2. Upgrade to secure connection (if available)
// 3. Send client info (if supported)
// 4. Look up session (if supported)
// 5. Authenticate user (or do guest login)
// 6. wait for session list
// 7. wait for user to finish typing join password if needed
// 8. send host/join command
// 9. wait for OK
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LoginState {
    WaitForSecure,
    WaitForClientInfo,
    WaitForLookup,
    WaitForIdent,
    WaitForLogin,
    Ignore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerReplyType {
    Unknown,
    Login,            // used during the login phase
    Message,          // general chat type notification message
    Alert,            // urgent notification message
    Error,            // error occurred
    Result,           // command result
    Log,              // server log message
    SessionConf,      // session configuration update
    SizeLimitWarning, // session history size nearing limit (deprecated)
    Status,           // periodic status update
    Reset,            // session reset state
    Catchup,          // number of messages queued for upload
    ResetRequest,     // request client to perform a reset
    CaughtUp,         // previous catchup is complete
    BanImpEx,         // session ban import/export
    OutOfSpace,       // session is out of space, block local drawing
    StreamStart,      // streamed session reset start marker
    StreamProgress,   // streamed session reset progress control message
    PasswordChange,   // session password changed
    InviteCreated,    // invite code created
    Thumbnail,        // server wants us to make a thumbnail
}

impl ServerReplyType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ServerReplyType::Unknown => "unknown",
            ServerReplyType::Login => "login",
            ServerReplyType::Message => "msg",
            ServerReplyType::Alert => "alert",
            ServerReplyType::Error => "error",
            ServerReplyType::Result => "result",
            ServerReplyType::Log => "log",
            ServerReplyType::SessionConf => "sessionconf",
            ServerReplyType::SizeLimitWarning => "sizelimit",
            ServerReplyType::Status => "status",
            ServerReplyType::Reset => "reset",
            ServerReplyType::Catchup => "catchup",
            ServerReplyType::ResetRequest => "autoreset",
            ServerReplyType::CaughtUp => "caughtup",
            ServerReplyType::BanImpEx => "banimpex",
            ServerReplyType::OutOfSpace => "outofspace",
            ServerReplyType::StreamStart => "sstart",
            ServerReplyType::StreamProgress => "sprogress",
            ServerReplyType::PasswordChange => "passwordchange",
            ServerReplyType::InviteCreated => "invitecreated",
            ServerReplyType::Thumbnail => "thumbnail",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "login" => ServerReplyType::Login,
            "msg" => ServerReplyType::Message,
            "alert" => ServerReplyType::Alert,
            "error" => ServerReplyType::Error,
            "result" => ServerReplyType::Result,
            "log" => ServerReplyType::Log,
            "sessionconf" => ServerReplyType::SessionConf,
            "sizelimit" => ServerReplyType::SizeLimitWarning,
            "status" => ServerReplyType::Status,
            "reset" => ServerReplyType::Reset,
            "catchup" => ServerReplyType::Catchup,
            "autoreset" => ServerReplyType::ResetRequest,
            "caughtup" => ServerReplyType::CaughtUp,
            "banimpex" => ServerReplyType::BanImpEx,
            "outofspace" => ServerReplyType::OutOfSpace,
            "sstart" => ServerReplyType::StreamStart,
            "sprogress" => ServerReplyType::StreamProgress,
            "passwordchange" => ServerReplyType::PasswordChange,
            "invitecreated" => ServerReplyType::InviteCreated,
            "thumbnail" => ServerReplyType::Thumbnail,
            _ => ServerReplyType::Unknown,
        }
    }
}

pub struct MsgJoin {
    pub flags: u8,
    pub name: String,
}

impl MsgJoin {
    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.push(self.flags);
        buffer.extend_from_slice(&(self.name.len() as u16).to_be_bytes());
        buffer.extend_from_slice(self.name.as_bytes());
        buffer
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerCommand {
    pub cmd: String,
    pub args: Vec<Value>,
    pub kwargs: serde_json::Map<String, Value>,
}

impl ServerCommand {
    pub fn from_payload(payload: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let json_str = std::str::from_utf8(payload);

        match json_str {
            Ok(json_str) => {
                let mut map: serde_json::Map<String, Value> = serde_json::from_str(json_str)?;

                // Extract cmd and args, remove them from the map
                let cmd = match map.remove("cmd") {
                    Some(Value::String(s)) => s,
                    Some(v) => return Err(format!("cmd is not a string: {:?}", v).into()),
                    None => return Err("Missing 'cmd' field".into()),
                };

                let args = match map.remove("args") {
                    Some(Value::Array(arr)) => arr,
                    Some(Value::Null) | None => Vec::<Value>::new(),
                    Some(v) => return Err(format!("args is not an array: {:?}", v).into()),
                };

                // The rest of the map is kwargs
                let kwargs = map;

                Ok(ServerCommand { cmd, args, kwargs })
            }
            Err(e) => {
                println!("Invalid JSON: {}", e);
                print!("Payload (hex): ");
                for byte in payload {
                    print!("{:02x}", byte);
                }
                println!();
                return Err(format!("Invalid JSON: {}", e).into());
            }
        }
    }
}

#[derive(Debug)]
struct ClientState {
    state: LoginState,
    username: Option<String>,
    client_info: Option<Value>,
    lookup: Option<String>,
}

impl ClientState {
    fn new() -> Self {
        Self {
            state: LoginState::WaitForIdent,
            username: None,
            client_info: None,
            lookup: None,
        }
    }
}

impl Debug for DpMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DpMessage {{ length: {}, message_type: {:?}, user_id: {}, payload: {:?} }}",
            self.length,
            self.message_type,
            self.user_id,
            String::from_utf8(self.payload.clone())
        )
    }
}

impl DpMessage {
    fn new(message_type: DpMessageType, user_id: u8, payload: Vec<u8>) -> Self {
        Self {
            length: payload.len() as u16,
            message_type,
            user_id,
            payload,
        }
    }

    fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&self.length.to_be_bytes());
        buffer.push(self.message_type as u8);
        buffer.push(self.user_id);
        buffer.extend_from_slice(&self.payload);
        buffer
    }

    fn deserialize(buf: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        if buf.len() < 4 {
            return Err("Buffer too short to contain DpMessage header".into());
        }
        let length = u16::from_be_bytes([buf[0], buf[1]]);
        let message_type = match buf[2] {
            0 => DpMessageType::ServerCommand,
            1 => DpMessageType::Disconnect,
            2 => DpMessageType::Ping,
            3 => DpMessageType::KeepAlive,
            4 => DpMessageType::Thumbnail,
            31 => DpMessageType::Internal,
            32 => DpMessageType::Join,
            33 => DpMessageType::Leave,
            34 => DpMessageType::SessionOwner,
            35 => DpMessageType::Chat,
            36 => DpMessageType::TrustedUsers,
            37 => DpMessageType::SoftReset,
            38 => DpMessageType::PrivateChat,
            39 => DpMessageType::ResetStream,
            64 => DpMessageType::Interval,
            65 => DpMessageType::LaserTrail,
            66 => DpMessageType::MovePointer,
            67 => DpMessageType::RemovedMarker,
            68 => DpMessageType::UserAcl,
            69 => DpMessageType::LayerAcl,
            70 => DpMessageType::FeatureAccessLevels,
            71 => DpMessageType::DefaultLayer,
            72 => DpMessageType::RemovedFiltered,
            73 => DpMessageType::Extension,
            74 => DpMessageType::UndoDepth,
            75 => DpMessageType::Data,
            76 => DpMessageType::LocalChange,
            77 => DpMessageType::FeatureLimits,
            128 => DpMessageType::UndoPoint,
            129 => DpMessageType::CanvasResize,
            130 => DpMessageType::RemovedLayerCreate,
            131 => DpMessageType::LayerAttributes,
            132 => DpMessageType::LayerRetitle,
            133 => DpMessageType::RemovedLayerOrder,
            134 => DpMessageType::RemovedLayerDelete,
            135 => DpMessageType::RemovedLayerVisibility,
            136 => DpMessageType::PutImage,
            137 => DpMessageType::FillRect,
            138 => DpMessageType::RemovedToolChange,
            139 => DpMessageType::RemovedPenMove,
            140 => DpMessageType::PenUp,
            141 => DpMessageType::AnnotationCreate,
            142 => DpMessageType::AnnotationReshape,
            143 => DpMessageType::AnnotationEdit,
            144 => DpMessageType::AnnotationDelete,
            145 => DpMessageType::RemovedMoveRegion,
            146 => DpMessageType::PutTile,
            147 => DpMessageType::CanvasBackground,
            148 => DpMessageType::DrawDabsClassic,
            149 => DpMessageType::DrawDabsPixel,
            150 => DpMessageType::DrawDabsPixelSquare,
            151 => DpMessageType::DrawDabsMypaint,
            152 => DpMessageType::DrawDabsMypaintBlend,
            160 => DpMessageType::MoveRect,
            161 => DpMessageType::SetMetadataInt,
            162 => DpMessageType::LayerTreeCreate,
            163 => DpMessageType::LayerTreeMove,
            164 => DpMessageType::LayerTreeDelete,
            165 => DpMessageType::TransformRegion,
            166 => DpMessageType::TrackCreate,
            167 => DpMessageType::TrackRetitle,
            168 => DpMessageType::TrackDelete,
            169 => DpMessageType::TrackOrder,
            170 => DpMessageType::KeyFrameSet,
            171 => DpMessageType::KeyFrameRetitle,
            172 => DpMessageType::KeyFrameLayerAttributes,
            173 => DpMessageType::KeyFrameDelete,
            174 => DpMessageType::SelectionPut,
            175 => DpMessageType::SelectionClear,
            176 => DpMessageType::LocalMatch,
            177 => DpMessageType::SyncSelectionTile,
            178 => DpMessageType::PutImageZstd,
            179 => DpMessageType::PutTileZstd,
            180 => DpMessageType::CanvasBackgroundZstd,
            181 => DpMessageType::MoveRectZstd,
            182 => DpMessageType::TransformRegionZstd,
            255 => DpMessageType::Undo,
            other => {
                return Err(format!("Unknown DpMessageType byte: {}", other).into());
            }
        };
        let user_id = buf[3];
        let payload = if buf.len() > 4 {
            buf[4..].to_vec()
        } else {
            Vec::new()
        };
        Ok(Self {
            length,
            message_type,
            user_id,
            payload,
        })
    }
}

fn create_login_greeting() -> DpMessage {
    DpMessage::new(
        DpMessageType::ServerCommand,
        0,
        json!({
            "type": ServerReplyType::Login.as_str(),
            "message": "Drawpile server 2.3.0-beta.2-10-g009197990-dirty",
            "version": 4,
            "flags": ["MULTI", "AVATAR", "MBANIMPEX", "LOOKUP", "CINFO"],
            "methods": {
                "guest": {
                    "actions": ["join", "host"]
                }
            }
        })
        .to_string()
        .as_bytes()
        .to_vec(),
    )
}

fn create_error_message(code: &str, message: &str) -> DpMessage {
    DpMessage::new(
        DpMessageType::ServerCommand,
        0,
        json!({
            "type": ServerReplyType::Error.as_str(),
            "code": code,
            "message": message
        })
        .to_string()
        .as_bytes()
        .to_vec(),
    )
}

fn create_login_ok_message(
    message: &str,
    flags: Vec<&str>,
    username: &str,
    guest: bool,
) -> DpMessage {
    DpMessage::new(
        DpMessageType::ServerCommand,
        0,
        json!({
            "type": ServerReplyType::Result.as_str(),
            "state": "identOk",
            "message": message,
            "flags": flags,
            "username": username,
            "guest": guest
        })
        .to_string()
        .as_bytes()
        .to_vec(),
    )
}

fn create_session_list() -> DpMessage {
    DpMessage::new(
        DpMessageType::ServerCommand,
        0,
        json!({
            "type": ServerReplyType::Result.as_str(),
            "state": "sessions",
            "message": "Welcome",
            "sessions": []
        })
        .to_string()
        .as_bytes()
        .to_vec(),
    )
}

pub struct Session {
    pub id: String,
    pub history: SessionHistory,
}

pub struct SessionHistory {
    size_in_bytes: usize,
}

async fn handle_client_connection(
    mut socket: tokio::net::TcpStream,
    sessions: Arc<Mutex<HashMap<String, Session>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut client_state = ClientState::new();

    // Send initial greeting
    let greeting = create_login_greeting();
    socket.write_all(&greeting.serialize()).await?;
    println!("Sent greeting: {:?}", greeting);

    loop {
        let mut buf = [0; 4096];
        let n = match socket.read(&mut buf).await {
            Ok(0) => break, // Connection closed
            Ok(n) => n,
            Err(e) => {
                println!("Read error: {}", e);
                break;
            }
        };

        let message = match DpMessage::deserialize(&buf[..n]) {
            Ok(msg) => msg,
            Err(e) => {
                println!("Failed to deserialize message: {}", e);
                continue;
            }
        };

        // if message.message_type == DpMessageType::Ping {
        //     let user_id = message.user_id;

        //     let pong = DpMessage::new(
        //         DpMessageType::ServerCommand,
        //         user_id,
        //         vec![1]
        //     );
        //     socket.write_all(&pong.serialize()).await?;

        //     println!("Sent pong to user {}", user_id);
        //     continue;
        // }

        let mut cmd = None;
        match message.message_type {
            DpMessageType::ServerCommand => {
                cmd = match ServerCommand::from_payload(&message.payload) {
                    Ok(cmd) => Some(cmd),
                    Err(e) => {
                        println!("Failed to parse command: {}", e);
                        print!("Payload (hex): ");
                        for byte in &message.payload {
                            print!("{:02x}", byte);
                        }
                        println!();
                        continue;
                    }
                };

                println!("RECV < {:?}", cmd);
            }
            other => {
                println!("Unknown message type: {:?}", other);
            }
        }

        match cmd.as_ref().unwrap().cmd.as_str() {
            "cinfo" => {
                let response = json!({
                    "cinfo": { "browser": false },
                    "message": "Client info OK!",
                    "type": "result"
                });

                let response_bytes = response.to_string().into_bytes();
                let reply = DpMessage::new(
                    DpMessageType::ServerCommand,
                    message.user_id,
                    response_bytes,
                );
                socket.write_all(&reply.serialize()).await?;
            }
            "lookup" => {
                let response = json!({
                    "lookup": "host",
                    "message": "Host lookup OK!",
                    "type": "result"
                });

                let response_bytes = response.to_string().into_bytes();
                let reply = DpMessage::new(
                    DpMessageType::ServerCommand,
                    message.user_id,
                    response_bytes,
                );
                socket.write_all(&reply.serialize()).await?;
            }
            "ident" => {
                use serde_json::json;

                let response = json!({
                    "flags": ["WEB", "WEBSESSION", "WEBHOST", "HOST"],
                    "guest": true,
                    "ident": "limeburst",
                    "message": "Guest login OK!",
                    "state": "identOk",
                    "type": "result"
                });

                let response_bytes = response.to_string().into_bytes();
                let reply = DpMessage::new(
                    DpMessageType::ServerCommand,
                    message.user_id,
                    response_bytes,
                );
                socket.write_all(&reply.serialize()).await?;

                {
                    let response = json!({
                        "message": "Welcome",
                        "sessions": sessions
                            .lock()
                            .unwrap()
                            .values()
                            .map(|session| {
                                json!({
                                    "activeDrawingUserCount": 0,
                                    "alias": "",
                                    "authOnly": false,
                                    "autotitle": false,
                                    "closed": false,
                                    "founder": "limeburst",
                                    "hasPassword": false,
                                    "id": session.id,
                                    "idleOverride": false,
                                    "invites": false,
                                    "maxUserCount": 254,
                                    "nsfm": false,
                                    "persistent": false,
                                    "protocol": "dp:4.25.1",
                                    "size": 372,
                                    "startTime": "2025-08-14T13:41:15Z",
                                    "title": "",
                                    "unlisted": false,
                                    "userCount": 0
                                })
                            })
                            .collect::<Vec<Value>>(),
                        "type": "login"
                    });

                    let response_bytes = response.to_string().into_bytes();
                    let reply = DpMessage::new(
                        DpMessageType::ServerCommand,
                        message.user_id,
                        response_bytes,
                    );
                    socket.write_all(&reply.serialize()).await?;
                }
            }

            "host" => {
                // let response = serde_json::json!({
                //     "message": "New session",
                //     "sessions": [{
                //         "activeDrawingUserCount": 0,
                //         "alias": "",
                //         "authOnly": false,
                //         "autotitle": false,
                //         "closed": false,
                //         "founder": "limeburst",
                //         "hasPassword": false,
                //         "id": ulid::Ulid::new().to_string(),
                //         "idleOverride": false,
                //         "invites": false,
                //         "maxUserCount": 254,
                //         "nsfm": false,
                //         "persistent": false,
                //         "protocol": "dp:4.25.1",
                //         "size": 372,
                //         "startTime": "2025-08-14T13:41:15Z",
                //         "title": "",
                //         "unlisted": false,
                //         "userCount": 0
                //     }],
                //     "type": "login"
                // });

                // let response_bytes = response.to_string().into_bytes();
                // let reply = DpMessage::new(
                //     DpMessageType::ServerCommand,
                //     message.user_id,
                //     response_bytes,
                // );
                // socket.write_all(&reply.serialize()).await?;

                // Log the session creation
                // let log_response = serde_json::json!({
                //     "level": "Info",
                //     "message": "Session  created by limeburst",
                //     "timestamp": "2025-08-14T13:41:15Z",
                //     "topic": "Status",
                //     "type": "log"
                // });

                // let log_response_bytes = log_response.to_string().into_bytes();
                // let log_reply = DpMessage::new(
                //     DpMessageType::ServerCommand,
                //     message.user_id,
                //     log_response_bytes,
                // );
                // socket.write_all(&log_reply.serialize()).await?;

                // Send result
                let session_id = ulid::Ulid::new().to_string();
                sessions.lock().unwrap().insert(
                    session_id.clone(),
                    Session {
                        id: session_id.clone(),
                        history: SessionHistory { size_in_bytes: 0 },
                    },
                );

                let server_command_response = serde_json::json!({
                    "join": {
                        "authId": "",
                        "flags": [],
                        "id": session_id,
                        "user": 1
                    },
                    "message": "Starting new session!",
                    "state": "host",
                    "type": "result"
                });

                let server_command_bytes = server_command_response.to_string().into_bytes();
                let server_command_reply = DpMessage::new(
                    DpMessageType::ServerCommand,
                    message.user_id,
                    server_command_bytes,
                );
                socket.write_all(&server_command_reply.serialize()).await?;

                // Send join message
                let join_message = MsgJoin {
                    flags: 0,
                    name: "limeburst".to_string(),
                };

                let reply = DpMessage::new(
                    DpMessageType::Join,
                    message.user_id,
                    join_message.serialize(),
                );
                println!("SENT > {:?}", reply);
                socket.write_all(&reply.serialize()).await?;
            }
            _ => {
                println!("Unknown command: {}", cmd.as_ref().unwrap().cmd);
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:27750").await?;
    println!("Drawpile server listening on 0.0.0.0:27750");

    let sessions = Arc::new(Mutex::new(HashMap::<String, Session>::new()));

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from: {}", addr);

        let sessions = sessions.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_client_connection(socket, sessions).await {
                println!("Error handling client: {}", e);
            }
        });
    }
}
