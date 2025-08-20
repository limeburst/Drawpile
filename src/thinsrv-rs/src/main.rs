use std::collections::HashMap;
use std::fmt::Debug;
use std::io::Cursor;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub mod login;
pub mod protogen;
pub mod servercmd;

use login::{ClientState, create_login_greeting, handle_login_command};
use protogen::Message;
use servercmd::ServerCommand as ServerCommandParser;

use crate::protogen::{MessageType, Ping};

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
    let _client_state = ClientState::new();

    // Send initial greeting
    let greeting = create_login_greeting();
    let mut buffer = Vec::new();
    greeting.serialize(&mut buffer)?;

    socket.write_all(&buffer).await?;
    println!("> {:?}", greeting);

    loop {
        // Read DpMessage.length, and then read the rest of the message
        let mut length_buf = [0; 2];
        socket.read_exact(&mut length_buf).await?;
        let length = u16::from_be_bytes(length_buf);

        let mut message_type_buf = [0; 1];
        socket.read_exact(&mut message_type_buf).await?;
        let _message_type = message_type_buf[0];

        let mut user_id_buf = [0; 1];
        socket.read_exact(&mut user_id_buf).await?;
        let user_id = user_id_buf[0];

        let mut message_buf = vec![0; length as usize];
        socket.read_exact(&mut message_buf).await?;

        // Create a cursor that includes the full message for deserialization
        let mut full_message = Vec::with_capacity(2 + 1 + 1 + message_buf.len());
        full_message.extend_from_slice(&length_buf);
        full_message.extend_from_slice(&message_type_buf);
        full_message.extend_from_slice(&user_id_buf);
        full_message.extend_from_slice(&message_buf);

        let message = Message::deserialize(&mut Cursor::new(full_message))?;
        println!("< {:?}", message);

        if message.message_type() == MessageType::PING {
            let pong = Ping { is_pong: true };
            let reply = Message::Ping {
                user_id,
                payload: pong,
            };

            let mut buffer = Vec::new();
            reply.serialize(&mut buffer)?;
            socket.write_all(&buffer).await?;
            println!("> {:?}", reply);
            continue;
        }

        // From here, message is guaranteed to be a ServerCommand
        if let Message::ServerCommand {
            user_id: _msg_user_id,
            payload,
        } = &message
        {
            let server_command = ServerCommandParser::from_payload(payload.msg.as_bytes())?;

            handle_login_command(
                &server_command.cmd,
                user_id,
                &mut socket,
                sessions.clone(),
                &server_command,
            )
            .await?;
        }
    }
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
