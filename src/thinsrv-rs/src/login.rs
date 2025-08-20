use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::protogen::{Message, ServerCommand};
use crate::servercmd::ServerCommand as ServerCommandParser;
use crate::{ServerReplyType, Session, SessionHistory};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoginState {
    WaitForSecure,
    WaitForClientInfo,
    WaitForLookup,
    WaitForIdent,
    WaitForLogin,
    Ignore,
}

#[derive(Debug)]
pub struct ClientState {
    pub state: LoginState,
    pub username: Option<String>,
    pub client_info: Option<Value>,
    pub lookup: Option<String>,
}

impl ClientState {
    pub fn new() -> Self {
        Self {
            state: LoginState::WaitForIdent,
            username: None,
            client_info: None,
            lookup: None,
        }
    }
}

pub fn create_login_greeting() -> Message {
    let server_cmd = ServerCommand {
        msg: json!({
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
        .to_string(),
    };

    Message::ServerCommand {
        user_id: 0,
        payload: server_cmd,
    }
}

pub async fn handle_login_command(
    command: &str,
    user_id: u8,
    socket: &mut TcpStream,
    sessions: Arc<Mutex<HashMap<String, Session>>>,
    _server_command: &ServerCommandParser,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        "cinfo" => {
            let response = json!({
                "cinfo": { "browser": false },
                "message": "Client info OK!",
                "type": "result"
            });

            let response_bytes = response.to_string().into_bytes();
            let server_cmd = ServerCommand {
                msg: String::from_utf8(response_bytes)?,
            };
            let reply = Message::ServerCommand {
                user_id,
                payload: server_cmd,
            };

            let mut buffer = Vec::new();
            reply.serialize(&mut buffer)?;
            socket.write_all(&buffer).await?;
            println!("> {:?}", reply);
        }
        "lookup" => {
            let response = json!({
                "lookup": "host",
                "message": "Host lookup OK!",
                "type": "result"
            });

            let response_bytes = response.to_string().into_bytes();
            let server_cmd = ServerCommand {
                msg: String::from_utf8(response_bytes)?,
            };
            let reply = Message::ServerCommand {
                user_id,
                payload: server_cmd,
            };

            let mut buffer = Vec::new();
            reply.serialize(&mut buffer)?;
            socket.write_all(&buffer).await?;
            println!("> {:?}", reply);
        }
        "ident" => {
            let response = json!({
                "flags": ["WEB", "WEBSESSION", "WEBHOST", "HOST"],
                "guest": true,
                "ident": "limeburst",
                "message": "Guest login OK!",
                "state": "identOk",
                "type": "result"
            });

            let response_bytes = response.to_string().into_bytes();
            let server_cmd = ServerCommand {
                msg: String::from_utf8(response_bytes)?,
            };
            let reply = Message::ServerCommand {
                user_id,
                payload: server_cmd,
            };

            let mut buffer = Vec::new();
            reply.serialize(&mut buffer)?;
            socket.write_all(&buffer).await?;

            // Send session list
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
                let server_cmd = ServerCommand {
                    msg: String::from_utf8(response_bytes)?,
                };
                let reply = Message::ServerCommand {
                    user_id,
                    payload: server_cmd,
                };

                let mut buffer = Vec::new();
                reply.serialize(&mut buffer)?;
                socket.write_all(&buffer).await?;
                println!("> {:?}", reply);
            }
        }
        "host" => {
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
            let server_cmd = ServerCommand {
                msg: String::from_utf8(server_command_bytes)?,
            };
            let server_command_reply = Message::ServerCommand {
                user_id,
                payload: server_cmd,
            };

            let mut buffer = Vec::new();
            server_command_reply.serialize(&mut buffer)?;
            socket.write_all(&buffer).await?;
            println!("> {:?}", server_command_reply);

            // Send join message
            let join_payload = crate::protogen::Join {
                flags: 0,
                name: "limeburst".to_string(),
                avatar: Vec::new(),
            };

            let reply = Message::Join {
                user_id,
                payload: join_payload,
            };
            let mut buffer = Vec::new();
            reply.serialize(&mut buffer)?;
            socket.write_all(&buffer).await?;
            println!("> {:?}", reply);
        }
        "join" => {
            let response = json!({
                "message": "Joining session!",
                "state": "join",
                "type": "result"
            });

            let response_bytes = response.to_string().into_bytes();
            let server_cmd = ServerCommand {
                msg: String::from_utf8(response_bytes)?,
            };
            let reply = Message::ServerCommand {
                user_id,
                payload: server_cmd,
            };

            let mut buffer = Vec::new();
            reply.serialize(&mut buffer)?;
            socket.write_all(&buffer).await?;
            println!("> {:?}", reply);
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }

    Ok(())
}
