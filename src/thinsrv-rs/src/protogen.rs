// SPDX-License-Identifier: MIT
//
// Generated code - do not edit manually
// This file was generated from protocol.yaml

#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::io::{Read, Result as IoResult, Write};

#[derive(Debug)]
pub enum ProtocolError {
    IoError(std::io::Error),
    InvalidEnumValue(u8),
    InvalidMessageType(u8),
    InvalidStringData,
}

impl From<std::io::Error> for ProtocolError {
    fn from(error: std::io::Error) -> Self {
        ProtocolError::IoError(error)
    }
}

impl From<std::string::FromUtf8Error> for ProtocolError {
    fn from(_: std::string::FromUtf8Error) -> Self {
        ProtocolError::InvalidStringData
    }
}

pub type Result<T> = std::result::Result<T, ProtocolError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DisconnectReason {
    Error = 0,
    Kick = 1,
    Shutdown = 2,
    Other = 3,
}

impl DisconnectReason {
    pub fn from_u8(value: u8) -> Result<Self> {
        match value {
            0 => Ok(DisconnectReason::Error),
            1 => Ok(DisconnectReason::Kick),
            2 => Ok(DisconnectReason::Shutdown),
            3 => Ok(DisconnectReason::Other),
            _ => Err(ProtocolError::InvalidEnumValue(value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DataType {
    Userinfo = 0,
}

impl DataType {
    pub fn from_u8(value: u8) -> Result<Self> {
        match value {
            0 => Ok(DataType::Userinfo),
            _ => Err(ProtocolError::InvalidEnumValue(value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LocalchangeType {
    Layervisibility = 0,
    Backgroundtile = 1,
    Viewmode = 2,
    Activelayer = 3,
    Activeframe = 4,
    Onionskins = 5,
    Trackvisibility = 6,
    Trackonionskin = 7,
    Layersketch = 8,
    Layeralphalock = 9,
}

impl LocalchangeType {
    pub fn from_u8(value: u8) -> Result<Self> {
        match value {
            0 => Ok(LocalchangeType::Layervisibility),
            1 => Ok(LocalchangeType::Backgroundtile),
            2 => Ok(LocalchangeType::Viewmode),
            3 => Ok(LocalchangeType::Activelayer),
            4 => Ok(LocalchangeType::Activeframe),
            5 => Ok(LocalchangeType::Onionskins),
            6 => Ok(LocalchangeType::Trackvisibility),
            7 => Ok(LocalchangeType::Trackonionskin),
            8 => Ok(LocalchangeType::Layersketch),
            9 => Ok(LocalchangeType::Layeralphalock),
            _ => Err(ProtocolError::InvalidEnumValue(value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SetmetadataintField {
    Dpix = 0,
    Dpiy = 1,
    Framerate = 2,
    Framecount = 3,
}

impl SetmetadataintField {
    pub fn from_u8(value: u8) -> Result<Self> {
        match value {
            0 => Ok(SetmetadataintField::Dpix),
            1 => Ok(SetmetadataintField::Dpiy),
            2 => Ok(SetmetadataintField::Framerate),
            3 => Ok(SetmetadataintField::Framecount),
            _ => Err(ProtocolError::InvalidEnumValue(value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TransformregionMode {
    Nearest = 0,
    Bilinear = 1,
}

impl TransformregionMode {
    pub fn from_u8(value: u8) -> Result<Self> {
        match value {
            0 => Ok(TransformregionMode::Nearest),
            1 => Ok(TransformregionMode::Bilinear),
            _ => Err(ProtocolError::InvalidEnumValue(value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum KeyframesetSource {
    Layer = 0,
    Keyframe = 1,
}

impl KeyframesetSource {
    pub fn from_u8(value: u8) -> Result<Self> {
        match value {
            0 => Ok(KeyframesetSource::Layer),
            1 => Ok(KeyframesetSource::Keyframe),
            _ => Err(ProtocolError::InvalidEnumValue(value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SelectionputOp {
    Replace = 0,
    Unite = 1,
    Intersect = 2,
    Exclude = 3,
    Complement = 4,
}

impl SelectionputOp {
    pub fn from_u8(value: u8) -> Result<Self> {
        match value {
            0 => Ok(SelectionputOp::Replace),
            1 => Ok(SelectionputOp::Unite),
            2 => Ok(SelectionputOp::Intersect),
            3 => Ok(SelectionputOp::Exclude),
            4 => Ok(SelectionputOp::Complement),
            _ => Err(ProtocolError::InvalidEnumValue(value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MessageType {
    SERVER_COMMAND = 0,
    DISCONNECT = 1,
    PING = 2,
    KEEP_ALIVE = 3,
    THUMBNAIL = 4,
    JOIN = 32,
    LEAVE = 33,
    SESSION_OWNER = 34,
    CHAT = 35,
    TRUSTED_USERS = 36,
    SOFT_RESET = 37,
    PRIVATE_CHAT = 38,
    RESET_STREAM = 39,
    INTERVAL = 64,
    LASER_TRAIL = 65,
    MOVE_POINTER = 66,
    USER_ACL = 68,
    LAYER_ACL = 69,
    FEATURE_ACCESS_LEVELS = 70,
    DEFAULT_LAYER = 71,
    UNDO_DEPTH = 74,
    DATA = 75,
    LOCAL_CHANGE = 76,
    FEATURE_LIMITS = 77,
    UNDO_POINT = 128,
    CANVAS_RESIZE = 129,
    LAYER_ATTRIBUTES = 131,
    LAYER_RETITLE = 132,
    PUT_IMAGE = 136,
    FILL_RECT = 137,
    PEN_UP = 140,
    ANNOTATION_CREATE = 141,
    ANNOTATION_RESHAPE = 142,
    ANNOTATION_EDIT = 143,
    ANNOTATION_DELETE = 144,
    PUT_TILE = 146,
    CANVAS_BACKGROUND = 147,
    DRAW_DABS_CLASSIC = 148,
    DRAW_DABS_PIXEL = 149,
    DRAW_DABS_MY_PAINT = 151,
    DRAW_DABS_MY_PAINT_BLEND = 152,
    MOVE_RECT = 160,
    SET_METADATA_INT = 161,
    LAYER_TREE_CREATE = 162,
    LAYER_TREE_MOVE = 163,
    LAYER_TREE_DELETE = 164,
    TRANSFORM_REGION = 165,
    TRACK_CREATE = 166,
    TRACK_RETITLE = 167,
    TRACK_DELETE = 168,
    TRACK_ORDER = 169,
    KEY_FRAME_SET = 170,
    KEY_FRAME_RETITLE = 171,
    KEY_FRAME_LAYER_ATTRIBUTES = 172,
    KEY_FRAME_DELETE = 173,
    SELECTION_PUT = 174,
    SELECTION_CLEAR = 175,
    LOCAL_MATCH = 176,
    SYNC_SELECTION_TILE = 177,
    UNDO = 255,
}

impl MessageType {
    pub fn from_u8(value: u8) -> Result<Self> {
        match value {
            0 => Ok(MessageType::SERVER_COMMAND),
            1 => Ok(MessageType::DISCONNECT),
            2 => Ok(MessageType::PING),
            3 => Ok(MessageType::KEEP_ALIVE),
            4 => Ok(MessageType::THUMBNAIL),
            32 => Ok(MessageType::JOIN),
            33 => Ok(MessageType::LEAVE),
            34 => Ok(MessageType::SESSION_OWNER),
            35 => Ok(MessageType::CHAT),
            36 => Ok(MessageType::TRUSTED_USERS),
            37 => Ok(MessageType::SOFT_RESET),
            38 => Ok(MessageType::PRIVATE_CHAT),
            39 => Ok(MessageType::RESET_STREAM),
            64 => Ok(MessageType::INTERVAL),
            65 => Ok(MessageType::LASER_TRAIL),
            66 => Ok(MessageType::MOVE_POINTER),
            68 => Ok(MessageType::USER_ACL),
            69 => Ok(MessageType::LAYER_ACL),
            70 => Ok(MessageType::FEATURE_ACCESS_LEVELS),
            71 => Ok(MessageType::DEFAULT_LAYER),
            74 => Ok(MessageType::UNDO_DEPTH),
            75 => Ok(MessageType::DATA),
            76 => Ok(MessageType::LOCAL_CHANGE),
            77 => Ok(MessageType::FEATURE_LIMITS),
            128 => Ok(MessageType::UNDO_POINT),
            129 => Ok(MessageType::CANVAS_RESIZE),
            131 => Ok(MessageType::LAYER_ATTRIBUTES),
            132 => Ok(MessageType::LAYER_RETITLE),
            136 => Ok(MessageType::PUT_IMAGE),
            137 => Ok(MessageType::FILL_RECT),
            140 => Ok(MessageType::PEN_UP),
            141 => Ok(MessageType::ANNOTATION_CREATE),
            142 => Ok(MessageType::ANNOTATION_RESHAPE),
            143 => Ok(MessageType::ANNOTATION_EDIT),
            144 => Ok(MessageType::ANNOTATION_DELETE),
            146 => Ok(MessageType::PUT_TILE),
            147 => Ok(MessageType::CANVAS_BACKGROUND),
            148 => Ok(MessageType::DRAW_DABS_CLASSIC),
            149 => Ok(MessageType::DRAW_DABS_PIXEL),
            151 => Ok(MessageType::DRAW_DABS_MY_PAINT),
            152 => Ok(MessageType::DRAW_DABS_MY_PAINT_BLEND),
            160 => Ok(MessageType::MOVE_RECT),
            161 => Ok(MessageType::SET_METADATA_INT),
            162 => Ok(MessageType::LAYER_TREE_CREATE),
            163 => Ok(MessageType::LAYER_TREE_MOVE),
            164 => Ok(MessageType::LAYER_TREE_DELETE),
            165 => Ok(MessageType::TRANSFORM_REGION),
            166 => Ok(MessageType::TRACK_CREATE),
            167 => Ok(MessageType::TRACK_RETITLE),
            168 => Ok(MessageType::TRACK_DELETE),
            169 => Ok(MessageType::TRACK_ORDER),
            170 => Ok(MessageType::KEY_FRAME_SET),
            171 => Ok(MessageType::KEY_FRAME_RETITLE),
            172 => Ok(MessageType::KEY_FRAME_LAYER_ATTRIBUTES),
            173 => Ok(MessageType::KEY_FRAME_DELETE),
            174 => Ok(MessageType::SELECTION_PUT),
            175 => Ok(MessageType::SELECTION_CLEAR),
            176 => Ok(MessageType::LOCAL_MATCH),
            177 => Ok(MessageType::SYNC_SELECTION_TILE),
            255 => Ok(MessageType::UNDO),
            _ => Err(ProtocolError::InvalidMessageType(value)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Classicdab {
    pub x: i8,
    pub y: i8,
    pub size: u32,
    pub hardness: u8,
    pub opacity: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pixeldab {
    pub x: i8,
    pub y: i8,
    pub size: u16,
    pub opacity: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mypaintdab {
    pub x: i8,
    pub y: i8,
    pub size: u32,
    pub hardness: u8,
    pub opacity: u8,
    pub angle: u8,
    pub aspect_ratio: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mypaintblenddab {
    pub x: i8,
    pub y: i8,
    pub size: u32,
    pub hardness: u8,
    pub opacity: u8,
    pub angle: u8,
    pub aspect_ratio: u8,
}

/// Server command message
///
/// This is a general purpose message for sending commands to the server
/// and receiving replies. This is used for (among other things):
///
/// - the login handshake
/// - setting session parameters (e.g. max user count and password)
/// - sending administration commands (e.g. kick user)
#[derive(Debug, Clone, PartialEq)]
pub struct Servercommand {
    pub msg: String,
}

/// Disconnect notification
///
/// This message is used when closing the connection gracefully. The message queue
/// will automatically close the socket after sending this message.
#[derive(Debug, Clone, PartialEq)]
pub struct Disconnect {
    pub reason: DisconnectReason,
    pub message: String,
}

/// Ping message
///
/// This is used for latency measurement as well as a keepalive. Normally, the client
/// should be the one to send the ping messages.
///
/// The server should return a Ping with the is_pong flag set
#[derive(Debug, Clone, PartialEq)]
pub struct Ping {
    pub is_pong: bool,
}

/// The client may indicate support for this during login, in which
/// case the server will send this kind of message if it did not send
/// anything else for a while. This is to make sure the client doesn't
/// run into an idle timeout if its upload queue is too saturated to
/// actually manage sending out a ping.
#[derive(Debug, Clone, PartialEq)]
pub struct Keepalive {}

/// Message from the client to the server to provide a canvas
/// thumbnail. The data may be prefixed by a correlation sequence as
/// sent by the server, followed by the compressed image data in a
/// format like JPEG or WEBP. The server does not interpret this data.
#[derive(Debug, Clone, PartialEq)]
pub struct Thumbnail {
    pub data: Vec<u8>,
}

/// Inform the client of a new user
///
/// This message is sent only be the server. It associates a username
/// with a context ID.
#[derive(Debug, Clone, PartialEq)]
pub struct Join {
    pub flags: u8,
    pub name: String,
    pub avatar: Vec<u8>,
}

/// Inform the client of a user leaving
///
/// This message is sent only by the server. Upon receiving this message,
/// clients will typically remove the user from the user listing. The client
/// is also allowed to release resources associated with this context ID.
#[derive(Debug, Clone, PartialEq)]
pub struct Leave {}

/// Session ownership change
///
/// This message sets the users who have operator status. It can be
/// sent by users who are already operators or by the server (user id=0).
///
/// The list of operators implicitly contains the user who sends the
/// message, thus users cannot deop themselves.
///
/// The server sanitizes the ID list so, when distributed to other users,
/// it does not contain any duplicates or non-existing users and can be trusted
/// without checking the access control list.
#[derive(Debug, Clone, PartialEq)]
pub struct Sessionowner {
    pub users: Vec<u8>,
}

/// A chat message
///
/// Chat message sent by the server with the user ID 0 are server messages.
/// (Typically a Command message is used for server announcements, but the Chat message
/// is used for those messages that must be stored in the session history.)
#[derive(Debug, Clone, PartialEq)]
pub struct Chat {
    pub tflags: u8,
    pub oflags: u8,
    pub message: String,
}

/// List of trusted users
///
/// This message sets the list of user who have been tagged as trusted,
/// but who are not operators. The meaning of "trusted" is a mostly
/// clientside concept, but the session can be configured to allow trusted
/// users access to some operator commands. (Deputies)
///
/// This command can be sent by operators or by the server (ctx=0).
///
/// The server sanitizes the ID list so, when distributed to other users,
/// it does not contain any duplicates or non-existing users and can be trusted
/// without checking the access control list.
#[derive(Debug, Clone, PartialEq)]
pub struct Trustedusers {
    pub users: Vec<u8>,
}

/// Soft reset point marker
///
/// This message marks the point in the session history where a soft reset occurs.
/// A thick-server performs an internal soft-reset when a user joins.
///
/// All users should truncate their own session history when receiving this message,
/// since undos cannot cross the reset boundary.
#[derive(Debug, Clone, PartialEq)]
pub struct Softreset {}

/// A private chat message
///
/// Note. This message type was added in protocol 4.21.2 (v. 2.1.0). For backward compatiblity,
/// the server will not send any private messages from itself; it will only relay them from
/// other users. In version 3.0, this should be merged with the normal Chat message.
///
/// Private messages always bypass the session history.
#[derive(Debug, Clone, PartialEq)]
pub struct Privatechat {
    pub target: u8,
    pub oflags: u8,
    pub message: String,
}

/// Streamed chunk of session reset messages. The client and server
/// will negotiate support and compression algorithm.
#[derive(Debug, Clone, PartialEq)]
pub struct Resetstream {
    pub data: Vec<u8>,
}

/// Event interval record
///
/// This is used to preserve timing information in session recordings.
///
/// Note. The maximum interval (using a single message) is about 65 seconds.
/// Typically the intervals we want to store are a few seconds at most, so this should be enough.
#[derive(Debug, Clone, PartialEq)]
pub struct Interval {
    pub msecs: u16,
}

/// Start/end drawing pointer laser trail
///
/// This signals the beginning or the end of a laser pointer trail. The trail coordinates
/// are sent with MovePointer messages.
///
/// A nonzero persistence indicates the start of the trail and zero the end.
#[derive(Debug, Clone, PartialEq)]
pub struct Lasertrail {
    pub color: u32,
    pub persistence: u8,
}

/// Move user pointer
///
/// This is message is used to update the position of the user pointer when no
/// actual drawing is taking place. It is also used to draw the "laser pointer" trail.
/// Note. This is a META message, since this is used for a temporary visual effect only,
/// and thus doesn't affect the actual canvas content.
///
/// The pointer position is divided by 4, like classic brushes.
#[derive(Debug, Clone, PartialEq)]
pub struct Movepointer {
    pub x: i32,
    pub y: i32,
}

/// Set user specific locks
///
/// This is an opaque meta command that contains a list of users to be locked.
/// It can only be sent by session operators.
#[derive(Debug, Clone, PartialEq)]
pub struct Useracl {
    pub users: Vec<u8>,
}

/// Change layer access control list, setting permission flags, access level
/// and exclusive access on a layer.
///
/// The first two bits of the flags field indicate the access tier level, 0
/// standing for operators only, 1 for operators and trusted users, 2 for
/// operators, trusted and registered users and 3 for everyone having
/// access.
///
/// The sixth bit of the flags field locks or unlocks the layer properties.
///
/// The seventh bit of the flags field locks or unlocks the layer from being
/// moved.
///
/// The eigth bit of the flags field locks or unlocks the layer.
///
/// As a special case, setting the ACLs of ID 0 sets or clears the canvas
/// lock. The tier and exclusive user list is not used in this case.
#[derive(Debug, Clone, PartialEq)]
pub struct Layeracl {
    pub id: u32,
    pub flags: u8,
    pub exclusive: Vec<u8>,
}

/// Change feature access tiers
///
/// Tier 0 is operator, 1 is trusted, 2 is authenticated, 3 and above
/// is guest. A value of 255 means to leave that tier unchanged. Any
/// unknown features will be ignored by the client.
#[derive(Debug, Clone, PartialEq)]
pub struct Featureaccesslevels {
    pub feature_tiers: Vec<u8>,
}

/// Set the default layer
///
/// The default layer is the one new users default to when logging in.
/// If no default layer is set, the newest layer will be selected by default.
#[derive(Debug, Clone, PartialEq)]
pub struct Defaultlayer {
    pub id: u32,
}

/// Set maximum undo depth
#[derive(Debug, Clone, PartialEq)]
pub struct Undodepth {
    pub depth: u8,
}

/// Send and receive structured information. Intended for stuff like
/// sending and receiving user troubleshooting information, sharing
/// brushes etc. Should probably be a server meta message so that it
/// can be directed at the appropriate user, but that's something for
/// Drawpile 3.0.
#[derive(Debug, Clone, PartialEq)]
pub struct Data {
    pub msg_type: DataType,
    pub recipient: u8,
    pub body: Vec<u8>,
}

/// A local-only modification, such as toggling layer visibility or
/// setting a local canvas background. Shouldn't be sent over the
/// network, but will be recorded.
#[derive(Debug, Clone, PartialEq)]
pub struct Localchange {
    pub msg_type: LocalchangeType,
    pub body: Vec<u8>,
}

/// Change feature limits.
#[derive(Debug, Clone, PartialEq)]
pub struct Featurelimits {
    pub limits: Vec<i32>,
}

/// Undo demarcation point
///
/// The client sends an UndoPoint message to signal the start of an undoable sequence.
#[derive(Debug, Clone, PartialEq)]
pub struct Undopoint {}

/// Adjust canvas size
///
/// This is the first command that must be sent to initialize the session.
///
/// This affects the size of all existing and future layers.
///
/// The new canvas size is relative to the old one. The four adjustement
/// parameters extend or retract their respective borders.
/// Initial canvas resize should be (0, w, h, 0).
#[derive(Debug, Clone, PartialEq)]
pub struct Canvasresize {
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub left: i32,
}

/// Change layer attributes
///
/// If the target layer is locked, this command requires session operator privileges.
///
/// Specifying a sublayer requires session operator privileges. Currently, it is used
/// only when sublayers are needed at canvas initialization.
///
/// Note: the `fixed` flag is unused since version 2.2. It's functionality is replaced
/// by the custom timeline feature.
#[derive(Debug, Clone, PartialEq)]
pub struct Layerattributes {
    pub id: u32,
    pub sublayer: u8,
    pub flags: u8,
    pub opacity: u8,
    pub blend: u8,
}

/// Change a layer's title
#[derive(Debug, Clone, PartialEq)]
pub struct Layerretitle {
    pub id: u32,
    pub title: String,
}

/// Draw a bitmap onto a layer or selection.
///
/// This is used for pasting images, flood-filling, merging annotations
/// and other tasks where image processing is done client-side.
///
/// All layer blending modes are supported.
///
/// The image data is DEFLATEd 32bit premultiplied ARGB data. The image
/// is prefixed with a 32 bit unsigned integer (big endian) which
/// contains the expected length of the uncompressed data.
///
/// Note that since the message length is fairly limited, a large image
/// may have to be divided into multiple PutImage commands.
///
/// The layer id may refer to a selection.
#[derive(Debug, Clone, PartialEq)]
pub struct Putimage {
    pub layer: u32,
    pub mode: u8,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub image: Vec<u8>,
}

/// Fill a rectangle with solid color
///
/// The layer id may refer to a selection.
#[derive(Debug, Clone, PartialEq)]
pub struct Fillrect {
    pub layer: u32,
    pub mode: u8,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub color: u32,
}

/// Signals the end of a stroke. If one exists, this causes the
/// sublayer of this user on the given layer to be merged into its
/// parent. Strokes in indirect paint modes will create such sublayers.
/// A pen up command for layer 0 causes the sublayers on all layers to
/// be merged. If the message context id is 0, the sublayers of all
/// users are merged, either on the given layer or on all layers if
/// that's 0 too.
///
/// The layer id may refer to a selection.
#[derive(Debug, Clone, PartialEq)]
pub struct Penup {
    pub layer: u32,
}

/// Create a new annotation
///
/// Annotations are floating text layers. They are drawn over the image layers and
/// have no defined stacking order.
///
/// The new annotation created with this command is initally empy with a transparent background
#[derive(Debug, Clone, PartialEq)]
pub struct Annotationcreate {
    pub id: u16,
    pub x: i32,
    pub y: i32,
    pub w: u16,
    pub h: u16,
}

/// Change the position and size of an annotation
#[derive(Debug, Clone, PartialEq)]
pub struct Annotationreshape {
    pub id: u16,
    pub x: i32,
    pub y: i32,
    pub w: u16,
    pub h: u16,
}

/// Change annotation content
///
/// Accepted contents is the subset of HTML understood by QTextDocument
///
/// If an annotation is flagged as protected, it cannot be modified by users
/// other than the one who created it, or session operators.
#[derive(Debug, Clone, PartialEq)]
pub struct Annotationedit {
    pub id: u16,
    pub bg: u32,
    pub flags: u8,
    pub border: u8,
    pub text: String,
}

/// Delete an annotation
///
/// Note: Unlike in layer delete command, there is no "merge" option here.
/// Merging an annotation is done by rendering the annotation item to
/// an image and drawing the image with the PutImage command. This ensures
/// identical rendering on all clients.
#[derive(Debug, Clone, PartialEq)]
pub struct Annotationdelete {
    pub id: u16,
}

/// Set the content of a tile.
///
/// Unlike PutImage, this replaces an entire tile directly without any
/// blending. This command is typically used during canvas
/// initialization to set the initial content.
///
/// PutTile can target sublayers as well. This is used when generating
/// a reset image with incomplete indirect strokes. Sending a PenUp
/// command will merge the sublayer.
#[derive(Debug, Clone, PartialEq)]
pub struct Puttile {
    pub user: u8,
    pub layer: u32,
    pub sublayer: u8,
    pub col: u16,
    pub row: u16,
    pub repeat: u16,
    pub image: Vec<u8>,
}

/// Set the canvas background tile
///
/// If the payload is exactly 4 bytes long, it should be interpreted as a solid background color.
/// Otherwise, it is the DEFLATED tile bitmap
#[derive(Debug, Clone, PartialEq)]
pub struct Canvasbackground {
    pub image: Vec<u8>,
}

/// Draw classic brush dabs
///
/// A simple delta compression scheme is used. The coordinates of each
/// dab are relative to the previous dab. The coordinate system has
/// 1/4 pixel resolution. Divide by 4.0 before use. The size field is
/// the brush diameter multiplied by 256.
///
/// The layer id may refer to a selection.
#[derive(Debug, Clone, PartialEq)]
pub struct Drawdabsclassic {
    pub flags: u8,
    pub layer: u32,
    pub x: i32,
    pub y: i32,
    pub color: u32,
    pub mode: u8,
    pub dabs: Vec<Classicdab>,
}

/// Draw round pixel brush dabs
///
/// The same kind of delta compression is used as in classicdabs,
/// but the fields all have integer precision.
///
/// The layer id may refer to a selection.
#[derive(Debug, Clone, PartialEq)]
pub struct Drawdabspixel {
    pub flags: u8,
    pub layer: u32,
    pub x: i32,
    pub y: i32,
    pub color: u32,
    pub mode: u8,
    pub dabs: Vec<Pixeldab>,
}

/// Draw MyPaint brush dabs in "normal and eraser" or "pigment and
/// eraser" mode, the regular modes of MyPaint brushes as used in
/// MyPaint itself.
///
/// The lowest flag bit indicates that this message uses pigment and
/// eraser instead of normal and eraser mode. The top 5 bits indicate
/// the selection ID to use for masking.
///
/// If the highest bit (0x80) of the mode field is not set, it is
/// treated as the number of posterization colors. If it is set, the
/// first two bits decide the paint and blend mode of this stroke: 0x0
/// means direct with Normal and Eraser mode, 0x1 means soft indirect
/// with Normal mode, 0x2 means soft indirect with Recolor mode and 0x3
/// means soft indirect with Erase mode. This is for backward
/// compatibility with the 2.2 protocol, 2.3 uses the mypaintdabsblend
/// message instead.
///
/// The layer id may refer to a selection.
#[derive(Debug, Clone, PartialEq)]
pub struct Drawdabsmypaint {
    pub flags: u8,
    pub layer: u32,
    pub x: i32,
    pub y: i32,
    pub color: u32,
    pub lock_alpha: u8,
    pub colorize: u8,
    pub posterize: u8,
    pub mode: u8,
    pub dabs: Vec<Mypaintdab>,
}

/// Draw MyPaint brush dabs with single blend mode. Used for cases
/// where the regular MyPaint blending with "normal and eraser" mode is
/// unsuitable.
///
/// The layer id may refer to a selection.
#[derive(Debug, Clone, PartialEq)]
pub struct Drawdabsmypaintblend {
    pub flags: u8,
    pub layer: u32,
    pub x: i32,
    pub y: i32,
    pub color: u32,
    pub mode: u8,
    pub dabs: Vec<Mypaintblenddab>,
}

/// Move a rectangular area on a layer or a selection.
///
/// A mask image can be given to mask out part of the region
/// to support non-rectangular selections.
///
/// Source and target rects may be (partially) outside the canvas.
///
/// The source and layer id may refer to a selection.
#[derive(Debug, Clone, PartialEq)]
pub struct Moverect {
    pub layer: u32,
    pub source: u32,
    pub sx: i32,
    pub sy: i32,
    pub tx: i32,
    pub ty: i32,
    pub w: i32,
    pub h: i32,
    pub blend: u8,
    pub opacity: u8,
    pub mask: Vec<u8>,
}

/// Set a document metadata field (integer type)
///
/// These typically don't have an immediate visual effect,
/// but these fields are part of the document, like the pixel content
/// or the annotations.
#[derive(Debug, Clone, PartialEq)]
pub struct Setmetadataint {
    pub field: SetmetadataintField,
    pub value: i32,
}

/// Create a new layer
///
/// A session starts with zero layers, so a layer creation command is typically
/// the second command to be sent, right after setting the canvas size.
///
/// The layer ID must be prefixed with the context ID of the user creating it.
/// This allows the client to choose the layer ID without worrying about
/// clashes. In multiuser mode the ACL filter validates the prefix for all new layers.
///
/// If the `source` field is nonzero, a copy of the source layer is made.
/// Otherwise, either a blank new bitmap or a group layer is created.
/// When copying a group, the group's layers are assigned new IDs sequentally,
/// starting from the group ID, using the group IDs user prefix.
///
/// If the `target` field is nonzero, the newly created layer will be
/// insert above that layer or group, or into that group. If zero,
/// the layer will be added to the top of the root group.
///
/// The following flags can be used with layer creation:
/// - GROUP: a group layer is created (ignored if `source` is set)
/// - INTO: the new layer will be added to the top to the `target` group.
/// The target must be nonzero.
///
/// If layer controls are locked, this command requires session operator privileges.
#[derive(Debug, Clone, PartialEq)]
pub struct Layertreecreate {
    pub id: u32,
    pub source: u32,
    pub target: u32,
    pub fill: u32,
    pub flags: u8,
    pub title: String,
}

/// Reorder a layer
///
/// Moves the given layer into the given parent group or into the root
/// if given 0. It will be placed either below the given sibling or at
/// the bottom if given 0. An invalid move, such as because the parent
/// is missing or the sibling isn't part of the parent group, will not
/// be executed at all, no fallback is attempted.
///
/// The user is allowed to move a layer if they can edit both it and
/// the parent. Moving into the parent group is allowed if the user has
/// would be allowed to create a layer there (which is always, since
/// that's currently conflated with being allowed to edit layers.)
#[derive(Debug, Clone, PartialEq)]
pub struct Layertreemove {
    pub layer: u32,
    pub parent: u32,
    pub sibling: u32,
}

/// Delete a layer
///
/// If the merge to attribute is nonzero, the contents of the layer is merged
/// to the layer with the given ID. If the id and merge id both refer to
/// the same layer group, that group is collapsed into a layer.
///
/// If the current layer or layer controls in general are locked, this command
/// requires session operator privileges.
#[derive(Debug, Clone, PartialEq)]
pub struct Layertreedelete {
    pub id: u32,
    pub merge_to: u32,
}

/// Transform an area, optionally moving it between two layers.
///
/// This is used to implement selection moving. It is equivalent
/// to doing two PutImages: the first to mask away the original
/// selection and the other to paste the selection to a new location.
///
/// This command packages that into a single action that is more
/// bandwidth efficient and can be used even when PutImages in general
/// are locked, since it's not introducing any new pixels onto the canvas.
///
/// Internally, the paint engine performs the following steps:
/// 1. Copy selected pixels to a buffer
/// 2. Erase selected pixels from the source layer
/// 3. Composite transformed buffer onto the target layer
///
/// The pixel selection is determined by the mask bitmap. The mask
/// is DEFLATEd 8 bit alpha.
///
/// For axis aligned rectangle selections, no bitmap is necessary.
///
/// The source and layer id may refer to a selection.
#[derive(Debug, Clone, PartialEq)]
pub struct Transformregion {
    pub layer: u32,
    pub source: u32,
    pub bx: i32,
    pub by: i32,
    pub bw: i32,
    pub bh: i32,
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub x3: i32,
    pub y3: i32,
    pub x4: i32,
    pub y4: i32,
    pub mode: TransformregionMode,
    pub blend: u8,
    pub opacity: u8,
    pub mask: Vec<u8>,
}

/// Create a timeline track.
///
/// The track id must be prefixed by the user's context id, like layer
/// and annotation ids. Operators are exempt from this restriction.
#[derive(Debug, Clone, PartialEq)]
pub struct Trackcreate {
    pub id: u16,
    pub insert_id: u16,
    pub source_id: u16,
    pub title: String,
}

/// Rename a timeline track.
#[derive(Debug, Clone, PartialEq)]
pub struct Trackretitle {
    pub id: u16,
    pub title: String,
}

/// Delete a timeline track.
#[derive(Debug, Clone, PartialEq)]
pub struct Trackdelete {
    pub id: u16,
}

/// Reorder timeline tracks.
///
/// Works like the LayerOrder command, just for tracks: duplicates are
/// ignored and missing tracks are appended to the end.
#[derive(Debug, Clone, PartialEq)]
pub struct Trackorder {
    pub tracks: Vec<u16>,
}

/// Create or modify a key frame.
///
/// If there's no key frame at the given frame index, it will be
/// created, otherwise it will be clobbered. The layer must exist and
/// the frame index must be within the document's frame count.
///
/// If the source is `Layer`, a new key frame will be created with
/// `source_id` as the layer id, `source_index` will be ignored. If the
/// source is `KeyFrame`, the key frame is copied from the key frame at
/// track with id `source_id` and frame index `source_index`.
#[derive(Debug, Clone, PartialEq)]
pub struct Keyframeset {
    pub track_id: u16,
    pub frame_index: u16,
    pub source_id: u32,
    pub source_index: u16,
    pub source: KeyframesetSource,
}

/// Rename a key frame.
#[derive(Debug, Clone, PartialEq)]
pub struct Keyframeretitle {
    pub track_id: u16,
    pub frame_index: u16,
    pub title: String,
}

/// Set (clobber) flags for layers inside of a key frame.
///
/// This takes a list of (layer id, flags) pairs. If a layer appears
/// multiple times, only the first occurrence will apply. If a layer
/// doesn't exist or the flags value is 0, it will be ignored. Unknown
/// flags will be retained for forward-compatibility.
///
/// Used flags are:
///
/// * `0x1` hidden: hides a layer in the key frame. Children can
/// override being hidden by setting the revealed flag on them.
///
/// * `0x2` revealed: overrides the hidden state given by a parent
/// group, making it visible again. Putting both hidden and revealed
/// on the same layer cancels each other out and will act like
/// neither is set.
#[derive(Debug, Clone, PartialEq)]
pub struct Keyframelayerattributes {
    pub track_id: u16,
    pub frame_index: u16,
    pub layer_flags: Vec<u32>,
}

/// Delete a key frame, possibly moving it somewhere else.
#[derive(Debug, Clone, PartialEq)]
pub struct Keyframedelete {
    pub track_id: u16,
    pub frame_index: u16,
    pub move_track_id: u16,
    pub move_frame_index: u16,
}

/// Modify selection, either by a rectangle or a pixel mask.
///
/// The selection_id specifies which of the user's selections is affected.
/// An id of 0 is invalid.
///
/// The mask is delta-encoded, zstd-compressed 8 bit alpha. If absent, this
/// fills the entire rectangle instead.
///
/// This message is never sent over the network, it's matched by LocalMatch
/// messages instead and selections are synchronized to the remote using
/// SyncSelectionTile messages.
#[derive(Debug, Clone, PartialEq)]
pub struct Selectionput {
    pub selection_id: u8,
    pub op: SelectionputOp,
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
    pub mask: Vec<u8>,
}

/// Remove the selection specified by the selection_id, or all selections if
/// it's 0.
///
/// This message is never sent over the network, it's matched by LocalMatch
/// messages instead and selections are synchronized to the remote using
/// SyncSelectionTile messages.
#[derive(Debug, Clone, PartialEq)]
pub struct Selectionclear {
    pub selection_id: u8,
}

/// A command that is sent over the network just to match it with another
/// message in the local fork. It has no effect in itself. This is currently
/// used for selections, which only have an effect on the local user and
/// other users don't need to bother with processing them.
///
/// The type describes the message type this is matched with and the
/// contents of data depend on that type being matched. It usually contains
/// the same stuff as the matched message, minus any variable-length
/// buffers, where only the size is sent along, since that's good enough for
/// getting a match in practice.
///
/// Strictly speaking, this is not compatible with the 2.2 protocol and
/// clients before version 2.2.2 don't understand it. However, since it
/// doesn't have any effect for other users, it doesn't cause desync, so we
/// accept it anyway. In sessions on the builtin server, a PutImage message
/// with an invalid blend mode is used instead.
#[derive(Debug, Clone, PartialEq)]
pub struct Localmatch {
    pub msg_type: u8,
    pub data: Vec<u8>,
}

/// Synchronizes a tile from a local selection into a remote one. The
/// selection id must be 128 or higher.
///
/// The mask is zstd-compressed, delta-encoded 8 bit alpha. A zero-length
/// mask will make the tile blank. A mask with a single zero byte will make
/// the tile fully opaque.  When the column and row are both 0xffff and the
/// mask has a length of zero, the selection is cleared instead.
///
/// This command isn't rolled back by undos.
#[derive(Debug, Clone, PartialEq)]
pub struct Syncselectiontile {
    pub user: u8,
    pub selection_id: u8,
    pub col: u16,
    pub row: u16,
    pub mask: Vec<u8>,
}

/// Undo or redo actions
#[derive(Debug, Clone, PartialEq)]
pub struct Undo {
    pub override_user: u8,
    pub redo: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    /// Server command message
    ///
    /// This is a general purpose message for sending commands to the server
    /// and receiving replies. This is used for (among other things):
    ///
    /// - the login handshake
    /// - setting session parameters (e.g. max user count and password)
    /// - sending administration commands (e.g. kick user)
    Servercommand(Servercommand),
    /// Disconnect notification
    ///
    /// This message is used when closing the connection gracefully. The message queue
    /// will automatically close the socket after sending this message.
    Disconnect(Disconnect),
    /// Ping message
    ///
    /// This is used for latency measurement as well as a keepalive. Normally, the client
    /// should be the one to send the ping messages.
    ///
    /// The server should return a Ping with the is_pong flag set
    Ping(Ping),
    /// The client may indicate support for this during login, in which
    /// case the server will send this kind of message if it did not send
    /// anything else for a while. This is to make sure the client doesn't
    /// run into an idle timeout if its upload queue is too saturated to
    /// actually manage sending out a ping.
    Keepalive(Keepalive),
    /// Message from the client to the server to provide a canvas
    /// thumbnail. The data may be prefixed by a correlation sequence as
    /// sent by the server, followed by the compressed image data in a
    /// format like JPEG or WEBP. The server does not interpret this data.
    Thumbnail(Thumbnail),
    /// Inform the client of a new user
    ///
    /// This message is sent only be the server. It associates a username
    /// with a context ID.
    Join(Join),
    /// Inform the client of a user leaving
    ///
    /// This message is sent only by the server. Upon receiving this message,
    /// clients will typically remove the user from the user listing. The client
    /// is also allowed to release resources associated with this context ID.
    Leave(Leave),
    /// Session ownership change
    ///
    /// This message sets the users who have operator status. It can be
    /// sent by users who are already operators or by the server (user id=0).
    ///
    /// The list of operators implicitly contains the user who sends the
    /// message, thus users cannot deop themselves.
    ///
    /// The server sanitizes the ID list so, when distributed to other users,
    /// it does not contain any duplicates or non-existing users and can be trusted
    /// without checking the access control list.
    Sessionowner(Sessionowner),
    /// A chat message
    ///
    /// Chat message sent by the server with the user ID 0 are server messages.
    /// (Typically a Command message is used for server announcements, but the Chat message
    /// is used for those messages that must be stored in the session history.)
    Chat(Chat),
    /// List of trusted users
    ///
    /// This message sets the list of user who have been tagged as trusted,
    /// but who are not operators. The meaning of "trusted" is a mostly
    /// clientside concept, but the session can be configured to allow trusted
    /// users access to some operator commands. (Deputies)
    ///
    /// This command can be sent by operators or by the server (ctx=0).
    ///
    /// The server sanitizes the ID list so, when distributed to other users,
    /// it does not contain any duplicates or non-existing users and can be trusted
    /// without checking the access control list.
    Trustedusers(Trustedusers),
    /// Soft reset point marker
    ///
    /// This message marks the point in the session history where a soft reset occurs.
    /// A thick-server performs an internal soft-reset when a user joins.
    ///
    /// All users should truncate their own session history when receiving this message,
    /// since undos cannot cross the reset boundary.
    Softreset(Softreset),
    /// A private chat message
    ///
    /// Note. This message type was added in protocol 4.21.2 (v. 2.1.0). For backward compatiblity,
    /// the server will not send any private messages from itself; it will only relay them from
    /// other users. In version 3.0, this should be merged with the normal Chat message.
    ///
    /// Private messages always bypass the session history.
    Privatechat(Privatechat),
    /// Streamed chunk of session reset messages. The client and server
    /// will negotiate support and compression algorithm.
    Resetstream(Resetstream),
    /// Event interval record
    ///
    /// This is used to preserve timing information in session recordings.
    ///
    /// Note. The maximum interval (using a single message) is about 65 seconds.
    /// Typically the intervals we want to store are a few seconds at most, so this should be enough.
    Interval(Interval),
    /// Start/end drawing pointer laser trail
    ///
    /// This signals the beginning or the end of a laser pointer trail. The trail coordinates
    /// are sent with MovePointer messages.
    ///
    /// A nonzero persistence indicates the start of the trail and zero the end.
    Lasertrail(Lasertrail),
    /// Move user pointer
    ///
    /// This is message is used to update the position of the user pointer when no
    /// actual drawing is taking place. It is also used to draw the "laser pointer" trail.
    /// Note. This is a META message, since this is used for a temporary visual effect only,
    /// and thus doesn't affect the actual canvas content.
    ///
    /// The pointer position is divided by 4, like classic brushes.
    Movepointer(Movepointer),
    /// Set user specific locks
    ///
    /// This is an opaque meta command that contains a list of users to be locked.
    /// It can only be sent by session operators.
    Useracl(Useracl),
    /// Change layer access control list, setting permission flags, access level
    /// and exclusive access on a layer.
    ///
    /// The first two bits of the flags field indicate the access tier level, 0
    /// standing for operators only, 1 for operators and trusted users, 2 for
    /// operators, trusted and registered users and 3 for everyone having
    /// access.
    ///
    /// The sixth bit of the flags field locks or unlocks the layer properties.
    ///
    /// The seventh bit of the flags field locks or unlocks the layer from being
    /// moved.
    ///
    /// The eigth bit of the flags field locks or unlocks the layer.
    ///
    /// As a special case, setting the ACLs of ID 0 sets or clears the canvas
    /// lock. The tier and exclusive user list is not used in this case.
    Layeracl(Layeracl),
    /// Change feature access tiers
    ///
    /// Tier 0 is operator, 1 is trusted, 2 is authenticated, 3 and above
    /// is guest. A value of 255 means to leave that tier unchanged. Any
    /// unknown features will be ignored by the client.
    Featureaccesslevels(Featureaccesslevels),
    /// Set the default layer
    ///
    /// The default layer is the one new users default to when logging in.
    /// If no default layer is set, the newest layer will be selected by default.
    Defaultlayer(Defaultlayer),
    /// Set maximum undo depth
    Undodepth(Undodepth),
    /// Send and receive structured information. Intended for stuff like
    /// sending and receiving user troubleshooting information, sharing
    /// brushes etc. Should probably be a server meta message so that it
    /// can be directed at the appropriate user, but that's something for
    /// Drawpile 3.0.
    Data(Data),
    /// A local-only modification, such as toggling layer visibility or
    /// setting a local canvas background. Shouldn't be sent over the
    /// network, but will be recorded.
    Localchange(Localchange),
    /// Change feature limits.
    Featurelimits(Featurelimits),
    /// Undo demarcation point
    ///
    /// The client sends an UndoPoint message to signal the start of an undoable sequence.
    Undopoint(Undopoint),
    /// Adjust canvas size
    ///
    /// This is the first command that must be sent to initialize the session.
    ///
    /// This affects the size of all existing and future layers.
    ///
    /// The new canvas size is relative to the old one. The four adjustement
    /// parameters extend or retract their respective borders.
    /// Initial canvas resize should be (0, w, h, 0).
    Canvasresize(Canvasresize),
    /// Change layer attributes
    ///
    /// If the target layer is locked, this command requires session operator privileges.
    ///
    /// Specifying a sublayer requires session operator privileges. Currently, it is used
    /// only when sublayers are needed at canvas initialization.
    ///
    /// Note: the `fixed` flag is unused since version 2.2. It's functionality is replaced
    /// by the custom timeline feature.
    Layerattributes(Layerattributes),
    /// Change a layer's title
    Layerretitle(Layerretitle),
    /// Draw a bitmap onto a layer or selection.
    ///
    /// This is used for pasting images, flood-filling, merging annotations
    /// and other tasks where image processing is done client-side.
    ///
    /// All layer blending modes are supported.
    ///
    /// The image data is DEFLATEd 32bit premultiplied ARGB data. The image
    /// is prefixed with a 32 bit unsigned integer (big endian) which
    /// contains the expected length of the uncompressed data.
    ///
    /// Note that since the message length is fairly limited, a large image
    /// may have to be divided into multiple PutImage commands.
    ///
    /// The layer id may refer to a selection.
    Putimage(Putimage),
    /// Fill a rectangle with solid color
    ///
    /// The layer id may refer to a selection.
    Fillrect(Fillrect),
    /// Signals the end of a stroke. If one exists, this causes the
    /// sublayer of this user on the given layer to be merged into its
    /// parent. Strokes in indirect paint modes will create such sublayers.
    /// A pen up command for layer 0 causes the sublayers on all layers to
    /// be merged. If the message context id is 0, the sublayers of all
    /// users are merged, either on the given layer or on all layers if
    /// that's 0 too.
    ///
    /// The layer id may refer to a selection.
    Penup(Penup),
    /// Create a new annotation
    ///
    /// Annotations are floating text layers. They are drawn over the image layers and
    /// have no defined stacking order.
    ///
    /// The new annotation created with this command is initally empy with a transparent background
    Annotationcreate(Annotationcreate),
    /// Change the position and size of an annotation
    Annotationreshape(Annotationreshape),
    /// Change annotation content
    ///
    /// Accepted contents is the subset of HTML understood by QTextDocument
    ///
    /// If an annotation is flagged as protected, it cannot be modified by users
    /// other than the one who created it, or session operators.
    Annotationedit(Annotationedit),
    /// Delete an annotation
    ///
    /// Note: Unlike in layer delete command, there is no "merge" option here.
    /// Merging an annotation is done by rendering the annotation item to
    /// an image and drawing the image with the PutImage command. This ensures
    /// identical rendering on all clients.
    Annotationdelete(Annotationdelete),
    /// Set the content of a tile.
    ///
    /// Unlike PutImage, this replaces an entire tile directly without any
    /// blending. This command is typically used during canvas
    /// initialization to set the initial content.
    ///
    /// PutTile can target sublayers as well. This is used when generating
    /// a reset image with incomplete indirect strokes. Sending a PenUp
    /// command will merge the sublayer.
    Puttile(Puttile),
    /// Set the canvas background tile
    ///
    /// If the payload is exactly 4 bytes long, it should be interpreted as a solid background color.
    /// Otherwise, it is the DEFLATED tile bitmap
    Canvasbackground(Canvasbackground),
    /// Draw classic brush dabs
    ///
    /// A simple delta compression scheme is used. The coordinates of each
    /// dab are relative to the previous dab. The coordinate system has
    /// 1/4 pixel resolution. Divide by 4.0 before use. The size field is
    /// the brush diameter multiplied by 256.
    ///
    /// The layer id may refer to a selection.
    Drawdabsclassic(Drawdabsclassic),
    /// Draw round pixel brush dabs
    ///
    /// The same kind of delta compression is used as in classicdabs,
    /// but the fields all have integer precision.
    ///
    /// The layer id may refer to a selection.
    Drawdabspixel(Drawdabspixel),
    /// Draw MyPaint brush dabs in "normal and eraser" or "pigment and
    /// eraser" mode, the regular modes of MyPaint brushes as used in
    /// MyPaint itself.
    ///
    /// The lowest flag bit indicates that this message uses pigment and
    /// eraser instead of normal and eraser mode. The top 5 bits indicate
    /// the selection ID to use for masking.
    ///
    /// If the highest bit (0x80) of the mode field is not set, it is
    /// treated as the number of posterization colors. If it is set, the
    /// first two bits decide the paint and blend mode of this stroke: 0x0
    /// means direct with Normal and Eraser mode, 0x1 means soft indirect
    /// with Normal mode, 0x2 means soft indirect with Recolor mode and 0x3
    /// means soft indirect with Erase mode. This is for backward
    /// compatibility with the 2.2 protocol, 2.3 uses the mypaintdabsblend
    /// message instead.
    ///
    /// The layer id may refer to a selection.
    Drawdabsmypaint(Drawdabsmypaint),
    /// Draw MyPaint brush dabs with single blend mode. Used for cases
    /// where the regular MyPaint blending with "normal and eraser" mode is
    /// unsuitable.
    ///
    /// The layer id may refer to a selection.
    Drawdabsmypaintblend(Drawdabsmypaintblend),
    /// Move a rectangular area on a layer or a selection.
    ///
    /// A mask image can be given to mask out part of the region
    /// to support non-rectangular selections.
    ///
    /// Source and target rects may be (partially) outside the canvas.
    ///
    /// The source and layer id may refer to a selection.
    Moverect(Moverect),
    /// Set a document metadata field (integer type)
    ///
    /// These typically don't have an immediate visual effect,
    /// but these fields are part of the document, like the pixel content
    /// or the annotations.
    Setmetadataint(Setmetadataint),
    /// Create a new layer
    ///
    /// A session starts with zero layers, so a layer creation command is typically
    /// the second command to be sent, right after setting the canvas size.
    ///
    /// The layer ID must be prefixed with the context ID of the user creating it.
    /// This allows the client to choose the layer ID without worrying about
    /// clashes. In multiuser mode the ACL filter validates the prefix for all new layers.
    ///
    /// If the `source` field is nonzero, a copy of the source layer is made.
    /// Otherwise, either a blank new bitmap or a group layer is created.
    /// When copying a group, the group's layers are assigned new IDs sequentally,
    /// starting from the group ID, using the group IDs user prefix.
    ///
    /// If the `target` field is nonzero, the newly created layer will be
    /// insert above that layer or group, or into that group. If zero,
    /// the layer will be added to the top of the root group.
    ///
    /// The following flags can be used with layer creation:
    /// - GROUP: a group layer is created (ignored if `source` is set)
    /// - INTO: the new layer will be added to the top to the `target` group.
    /// The target must be nonzero.
    ///
    /// If layer controls are locked, this command requires session operator privileges.
    Layertreecreate(Layertreecreate),
    /// Reorder a layer
    ///
    /// Moves the given layer into the given parent group or into the root
    /// if given 0. It will be placed either below the given sibling or at
    /// the bottom if given 0. An invalid move, such as because the parent
    /// is missing or the sibling isn't part of the parent group, will not
    /// be executed at all, no fallback is attempted.
    ///
    /// The user is allowed to move a layer if they can edit both it and
    /// the parent. Moving into the parent group is allowed if the user has
    /// would be allowed to create a layer there (which is always, since
    /// that's currently conflated with being allowed to edit layers.)
    Layertreemove(Layertreemove),
    /// Delete a layer
    ///
    /// If the merge to attribute is nonzero, the contents of the layer is merged
    /// to the layer with the given ID. If the id and merge id both refer to
    /// the same layer group, that group is collapsed into a layer.
    ///
    /// If the current layer or layer controls in general are locked, this command
    /// requires session operator privileges.
    Layertreedelete(Layertreedelete),
    /// Transform an area, optionally moving it between two layers.
    ///
    /// This is used to implement selection moving. It is equivalent
    /// to doing two PutImages: the first to mask away the original
    /// selection and the other to paste the selection to a new location.
    ///
    /// This command packages that into a single action that is more
    /// bandwidth efficient and can be used even when PutImages in general
    /// are locked, since it's not introducing any new pixels onto the canvas.
    ///
    /// Internally, the paint engine performs the following steps:
    /// 1. Copy selected pixels to a buffer
    /// 2. Erase selected pixels from the source layer
    /// 3. Composite transformed buffer onto the target layer
    ///
    /// The pixel selection is determined by the mask bitmap. The mask
    /// is DEFLATEd 8 bit alpha.
    ///
    /// For axis aligned rectangle selections, no bitmap is necessary.
    ///
    /// The source and layer id may refer to a selection.
    Transformregion(Transformregion),
    /// Create a timeline track.
    ///
    /// The track id must be prefixed by the user's context id, like layer
    /// and annotation ids. Operators are exempt from this restriction.
    Trackcreate(Trackcreate),
    /// Rename a timeline track.
    Trackretitle(Trackretitle),
    /// Delete a timeline track.
    Trackdelete(Trackdelete),
    /// Reorder timeline tracks.
    ///
    /// Works like the LayerOrder command, just for tracks: duplicates are
    /// ignored and missing tracks are appended to the end.
    Trackorder(Trackorder),
    /// Create or modify a key frame.
    ///
    /// If there's no key frame at the given frame index, it will be
    /// created, otherwise it will be clobbered. The layer must exist and
    /// the frame index must be within the document's frame count.
    ///
    /// If the source is `Layer`, a new key frame will be created with
    /// `source_id` as the layer id, `source_index` will be ignored. If the
    /// source is `KeyFrame`, the key frame is copied from the key frame at
    /// track with id `source_id` and frame index `source_index`.
    Keyframeset(Keyframeset),
    /// Rename a key frame.
    Keyframeretitle(Keyframeretitle),
    /// Set (clobber) flags for layers inside of a key frame.
    ///
    /// This takes a list of (layer id, flags) pairs. If a layer appears
    /// multiple times, only the first occurrence will apply. If a layer
    /// doesn't exist or the flags value is 0, it will be ignored. Unknown
    /// flags will be retained for forward-compatibility.
    ///
    /// Used flags are:
    ///
    /// * `0x1` hidden: hides a layer in the key frame. Children can
    /// override being hidden by setting the revealed flag on them.
    ///
    /// * `0x2` revealed: overrides the hidden state given by a parent
    /// group, making it visible again. Putting both hidden and revealed
    /// on the same layer cancels each other out and will act like
    /// neither is set.
    Keyframelayerattributes(Keyframelayerattributes),
    /// Delete a key frame, possibly moving it somewhere else.
    Keyframedelete(Keyframedelete),
    /// Modify selection, either by a rectangle or a pixel mask.
    ///
    /// The selection_id specifies which of the user's selections is affected.
    /// An id of 0 is invalid.
    ///
    /// The mask is delta-encoded, zstd-compressed 8 bit alpha. If absent, this
    /// fills the entire rectangle instead.
    ///
    /// This message is never sent over the network, it's matched by LocalMatch
    /// messages instead and selections are synchronized to the remote using
    /// SyncSelectionTile messages.
    Selectionput(Selectionput),
    /// Remove the selection specified by the selection_id, or all selections if
    /// it's 0.
    ///
    /// This message is never sent over the network, it's matched by LocalMatch
    /// messages instead and selections are synchronized to the remote using
    /// SyncSelectionTile messages.
    Selectionclear(Selectionclear),
    /// A command that is sent over the network just to match it with another
    /// message in the local fork. It has no effect in itself. This is currently
    /// used for selections, which only have an effect on the local user and
    /// other users don't need to bother with processing them.
    ///
    /// The type describes the message type this is matched with and the
    /// contents of data depend on that type being matched. It usually contains
    /// the same stuff as the matched message, minus any variable-length
    /// buffers, where only the size is sent along, since that's good enough for
    /// getting a match in practice.
    ///
    /// Strictly speaking, this is not compatible with the 2.2 protocol and
    /// clients before version 2.2.2 don't understand it. However, since it
    /// doesn't have any effect for other users, it doesn't cause desync, so we
    /// accept it anyway. In sessions on the builtin server, a PutImage message
    /// with an invalid blend mode is used instead.
    Localmatch(Localmatch),
    /// Synchronizes a tile from a local selection into a remote one. The
    /// selection id must be 128 or higher.
    ///
    /// The mask is zstd-compressed, delta-encoded 8 bit alpha. A zero-length
    /// mask will make the tile blank. A mask with a single zero byte will make
    /// the tile fully opaque.  When the column and row are both 0xffff and the
    /// mask has a length of zero, the selection is cleared instead.
    ///
    /// This command isn't rolled back by undos.
    Syncselectiontile(Syncselectiontile),
    /// Undo or redo actions
    Undo(Undo),
}

pub trait BinaryWriter {
    fn write_u8(&mut self, value: u8) -> IoResult<()>;
    fn write_u16(&mut self, value: u16) -> IoResult<()>;
    fn write_u24(&mut self, value: u32) -> IoResult<()>;
    fn write_u32(&mut self, value: u32) -> IoResult<()>;
    fn write_i8(&mut self, value: i8) -> IoResult<()>;
    fn write_i16(&mut self, value: i16) -> IoResult<()>;
    fn write_i24(&mut self, value: i32) -> IoResult<()>;
    fn write_i32(&mut self, value: i32) -> IoResult<()>;
    fn write_all(&mut self, buf: &[u8]) -> IoResult<()>;
}

pub trait BinaryReader {
    fn read_u8(&mut self) -> IoResult<u8>;
    fn read_u16(&mut self) -> IoResult<u16>;
    fn read_u24(&mut self) -> IoResult<u32>;
    fn read_u32(&mut self) -> IoResult<u32>;
    fn read_i8(&mut self) -> IoResult<i8>;
    fn read_i16(&mut self) -> IoResult<i16>;
    fn read_i24(&mut self) -> IoResult<i32>;
    fn read_i32(&mut self) -> IoResult<i32>;
    fn read_exact(&mut self, buf: &mut [u8]) -> IoResult<()>;
}

impl<W: Write> BinaryWriter for W {
    fn write_u8(&mut self, value: u8) -> IoResult<()> {
        self.write_all(&[value])
    }

    fn write_u16(&mut self, value: u16) -> IoResult<()> {
        self.write_all(&value.to_be_bytes())
    }

    fn write_u24(&mut self, value: u32) -> IoResult<()> {
        let bytes = value.to_be_bytes();
        self.write_all(&bytes[1..])
    }

    fn write_u32(&mut self, value: u32) -> IoResult<()> {
        self.write_all(&value.to_be_bytes())
    }

    fn write_i8(&mut self, value: i8) -> IoResult<()> {
        self.write_all(&[value as u8])
    }

    fn write_i16(&mut self, value: i16) -> IoResult<()> {
        self.write_all(&value.to_be_bytes())
    }

    fn write_i24(&mut self, value: i32) -> IoResult<()> {
        let bytes = value.to_be_bytes();
        self.write_all(&bytes[1..])
    }

    fn write_i32(&mut self, value: i32) -> IoResult<()> {
        self.write_all(&value.to_be_bytes())
    }

    fn write_all(&mut self, buf: &[u8]) -> IoResult<()> {
        Write::write_all(self, buf)
    }
}

impl<R: Read> BinaryReader for R {
    fn read_u8(&mut self) -> IoResult<u8> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    fn read_u16(&mut self) -> IoResult<u16> {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    fn read_u24(&mut self) -> IoResult<u32> {
        let mut buf = [0u8; 3];
        self.read_exact(&mut buf)?;
        Ok(u32::from_be_bytes([0, buf[0], buf[1], buf[2]]))
    }

    fn read_u32(&mut self) -> IoResult<u32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }

    fn read_i8(&mut self) -> IoResult<i8> {
        Ok(self.read_u8()? as i8)
    }

    fn read_i16(&mut self) -> IoResult<i16> {
        Ok(self.read_u16()? as i16)
    }

    fn read_i24(&mut self) -> IoResult<i32> {
        let value = self.read_u24()? as i32;
        // Sign extend 24-bit to 32-bit
        if value & 0x800000 != 0 {
            Ok(value | 0xFF000000u32 as i32)
        } else {
            Ok(value)
        }
    }

    fn read_i32(&mut self) -> IoResult<i32> {
        Ok(self.read_u32()? as i32)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> IoResult<()> {
        Read::read_exact(self, buf)
    }
}

pub trait Serializable {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()>;
    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self>
    where
        Self: Sized;
    fn serialized_size(&self) -> usize;
}

impl Serializable for Servercommand {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        let bytes = self.msg.as_bytes();
        writer.write_u16(bytes.len() as u16)?;
        writer.write_all(bytes)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let len = reader.read_u16()? as usize;
        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf)?;
        let msg = String::from_utf8(buf)?;
        Ok(Servercommand { msg })
    }

    fn serialized_size(&self) -> usize {
        2 + self.msg.len()
    }
}

impl Serializable for Disconnect {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(self.reason as u8)?;
        let bytes = self.message.as_bytes();
        writer.write_u16(bytes.len() as u16)?;
        writer.write_all(bytes)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let reason = DisconnectReason::from_u8(reader.read_u8()?)?;
        let len = reader.read_u16()? as usize;
        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf)?;
        let message = String::from_utf8(buf)?;
        Ok(Disconnect { reason, message })
    }

    fn serialized_size(&self) -> usize {
        1 + 2 + self.message.len()
    }
}

impl Serializable for Ping {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(if self.is_pong { 1 } else { 0 })?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let is_pong = reader.read_u8()? != 0;
        Ok(Ping { is_pong })
    }

    fn serialized_size(&self) -> usize {
        1
    }
}

impl Serializable for Keepalive {
    fn serialize<W: BinaryWriter>(&self, _writer: &mut W) -> Result<()> {
        Ok(())
    }

    fn deserialize<R: BinaryReader>(_reader: &mut R) -> Result<Self> {
        Ok(Keepalive {})
    }

    fn serialized_size(&self) -> usize {
        0
    }
}

impl Serializable for Thumbnail {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u16(self.data.len() as u16)?;
        writer.write_all(&self.data)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let len = reader.read_u16()? as usize;
        let mut data = vec![0u8; len];
        reader.read_exact(&mut data)?;
        Ok(Thumbnail { data })
    }

    fn serialized_size(&self) -> usize {
        2 + self.data.len()
    }
}

impl Serializable for Join {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(self.flags)?;
        let bytes = self.name.as_bytes();
        writer.write_u16(bytes.len() as u16)?;
        writer.write_all(bytes)?;
        writer.write_u16(self.avatar.len() as u16)?;
        writer.write_all(&self.avatar)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let flags = reader.read_u8()?;
        let len = reader.read_u16()? as usize;
        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf)?;
        let name = String::from_utf8(buf)?;
        let len = reader.read_u16()? as usize;
        let mut avatar = vec![0u8; len];
        reader.read_exact(&mut avatar)?;
        Ok(Join {
            flags,
            name,
            avatar,
        })
    }

    fn serialized_size(&self) -> usize {
        1 + 2 + self.name.len() + 2 + self.avatar.len()
    }
}

impl Serializable for Leave {
    fn serialize<W: BinaryWriter>(&self, _writer: &mut W) -> Result<()> {
        Ok(())
    }

    fn deserialize<R: BinaryReader>(_reader: &mut R) -> Result<Self> {
        Ok(Leave {})
    }

    fn serialized_size(&self) -> usize {
        0
    }
}

impl Serializable for Sessionowner {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u16(self.users.len() as u16)?;
        for item in &self.users {
            writer.write_u8(*item)?;
        }
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let len = reader.read_u16()? as usize;
        let mut users = Vec::with_capacity(len);
        for _ in 0..len {
            users.push(reader.read_u8()?);
        }
        Ok(Sessionowner { users })
    }

    fn serialized_size(&self) -> usize {
        2 + self.users.len() * 1
    }
}

impl Serializable for Chat {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(self.tflags)?;
        writer.write_u8(self.oflags)?;
        let bytes = self.message.as_bytes();
        writer.write_u16(bytes.len() as u16)?;
        writer.write_all(bytes)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let tflags = reader.read_u8()?;
        let oflags = reader.read_u8()?;
        let len = reader.read_u16()? as usize;
        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf)?;
        let message = String::from_utf8(buf)?;
        Ok(Chat {
            tflags,
            oflags,
            message,
        })
    }

    fn serialized_size(&self) -> usize {
        1 + 1 + 2 + self.message.len()
    }
}

impl Serializable for Trustedusers {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u16(self.users.len() as u16)?;
        for item in &self.users {
            writer.write_u8(*item)?;
        }
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let len = reader.read_u16()? as usize;
        let mut users = Vec::with_capacity(len);
        for _ in 0..len {
            users.push(reader.read_u8()?);
        }
        Ok(Trustedusers { users })
    }

    fn serialized_size(&self) -> usize {
        2 + self.users.len() * 1
    }
}

impl Serializable for Softreset {
    fn serialize<W: BinaryWriter>(&self, _writer: &mut W) -> Result<()> {
        Ok(())
    }

    fn deserialize<R: BinaryReader>(_reader: &mut R) -> Result<Self> {
        Ok(Softreset {})
    }

    fn serialized_size(&self) -> usize {
        0
    }
}

impl Serializable for Privatechat {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(self.target)?;
        writer.write_u8(self.oflags)?;
        let bytes = self.message.as_bytes();
        writer.write_u16(bytes.len() as u16)?;
        writer.write_all(bytes)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let target = reader.read_u8()?;
        let oflags = reader.read_u8()?;
        let len = reader.read_u16()? as usize;
        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf)?;
        let message = String::from_utf8(buf)?;
        Ok(Privatechat {
            target,
            oflags,
            message,
        })
    }

    fn serialized_size(&self) -> usize {
        1 + 1 + 2 + self.message.len()
    }
}

impl Serializable for Resetstream {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u16(self.data.len() as u16)?;
        writer.write_all(&self.data)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let len = reader.read_u16()? as usize;
        let mut data = vec![0u8; len];
        reader.read_exact(&mut data)?;
        Ok(Resetstream { data })
    }

    fn serialized_size(&self) -> usize {
        2 + self.data.len()
    }
}

impl Serializable for Interval {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u16(self.msecs)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let msecs = reader.read_u16()?;
        Ok(Interval { msecs })
    }

    fn serialized_size(&self) -> usize {
        2
    }
}

impl Serializable for Lasertrail {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u32(self.color)?;
        writer.write_u8(self.persistence)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let color = reader.read_u32()?;
        let persistence = reader.read_u8()?;
        Ok(Lasertrail { color, persistence })
    }

    fn serialized_size(&self) -> usize {
        4 + 1
    }
}

impl Serializable for Movepointer {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_i32(self.x)?;
        writer.write_i32(self.y)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let x = reader.read_i32()?;
        let y = reader.read_i32()?;
        Ok(Movepointer { x, y })
    }

    fn serialized_size(&self) -> usize {
        4 + 4
    }
}

impl Serializable for Useracl {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u16(self.users.len() as u16)?;
        for item in &self.users {
            writer.write_u8(*item)?;
        }
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let len = reader.read_u16()? as usize;
        let mut users = Vec::with_capacity(len);
        for _ in 0..len {
            users.push(reader.read_u8()?);
        }
        Ok(Useracl { users })
    }

    fn serialized_size(&self) -> usize {
        2 + self.users.len() * 1
    }
}

impl Serializable for Layeracl {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u24(self.id)?;
        writer.write_u8(self.flags)?;
        writer.write_u16(self.exclusive.len() as u16)?;
        for item in &self.exclusive {
            writer.write_u8(*item)?;
        }
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let id = reader.read_u24()?;
        let flags = reader.read_u8()?;
        let len = reader.read_u16()? as usize;
        let mut exclusive = Vec::with_capacity(len);
        for _ in 0..len {
            exclusive.push(reader.read_u8()?);
        }
        Ok(Layeracl {
            id,
            flags,
            exclusive,
        })
    }

    fn serialized_size(&self) -> usize {
        3 + 1 + 2 + self.exclusive.len() * 1
    }
}

impl Serializable for Featureaccesslevels {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u16(self.feature_tiers.len() as u16)?;
        for item in &self.feature_tiers {
            writer.write_u8(*item)?;
        }
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let len = reader.read_u16()? as usize;
        let mut feature_tiers = Vec::with_capacity(len);
        for _ in 0..len {
            feature_tiers.push(reader.read_u8()?);
        }
        Ok(Featureaccesslevels { feature_tiers })
    }

    fn serialized_size(&self) -> usize {
        2 + self.feature_tiers.len() * 1
    }
}

impl Serializable for Defaultlayer {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u24(self.id)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let id = reader.read_u24()?;
        Ok(Defaultlayer { id })
    }

    fn serialized_size(&self) -> usize {
        3
    }
}

impl Serializable for Undodepth {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(self.depth)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let depth = reader.read_u8()?;
        Ok(Undodepth { depth })
    }

    fn serialized_size(&self) -> usize {
        1
    }
}

impl Serializable for Data {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(self.msg_type as u8)?;
        writer.write_u8(self.recipient)?;
        writer.write_u16(self.body.len() as u16)?;
        writer.write_all(&self.body)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let msg_type = DataType::from_u8(reader.read_u8()?)?;
        let recipient = reader.read_u8()?;
        let len = reader.read_u16()? as usize;
        let mut body = vec![0u8; len];
        reader.read_exact(&mut body)?;
        Ok(Data {
            msg_type,
            recipient,
            body,
        })
    }

    fn serialized_size(&self) -> usize {
        1 + 1 + 2 + self.body.len()
    }
}

impl Serializable for Localchange {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(self.msg_type as u8)?;
        writer.write_u16(self.body.len() as u16)?;
        writer.write_all(&self.body)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let msg_type = LocalchangeType::from_u8(reader.read_u8()?)?;
        let len = reader.read_u16()? as usize;
        let mut body = vec![0u8; len];
        reader.read_exact(&mut body)?;
        Ok(Localchange { msg_type, body })
    }

    fn serialized_size(&self) -> usize {
        1 + 2 + self.body.len()
    }
}

impl Serializable for Featurelimits {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u16(self.limits.len() as u16)?;
        for item in &self.limits {
            writer.write_i32(*item)?;
        }
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let len = reader.read_u16()? as usize;
        let mut limits = Vec::with_capacity(len);
        for _ in 0..len {
            limits.push(reader.read_i32()?);
        }
        Ok(Featurelimits { limits })
    }

    fn serialized_size(&self) -> usize {
        2 + self.limits.len() * 4
    }
}

impl Serializable for Undopoint {
    fn serialize<W: BinaryWriter>(&self, _writer: &mut W) -> Result<()> {
        Ok(())
    }

    fn deserialize<R: BinaryReader>(_reader: &mut R) -> Result<Self> {
        Ok(Undopoint {})
    }

    fn serialized_size(&self) -> usize {
        0
    }
}

impl Serializable for Canvasresize {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_i32(self.top)?;
        writer.write_i32(self.right)?;
        writer.write_i32(self.bottom)?;
        writer.write_i32(self.left)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let top = reader.read_i32()?;
        let right = reader.read_i32()?;
        let bottom = reader.read_i32()?;
        let left = reader.read_i32()?;
        Ok(Canvasresize {
            top,
            right,
            bottom,
            left,
        })
    }

    fn serialized_size(&self) -> usize {
        4 + 4 + 4 + 4
    }
}

impl Serializable for Layerattributes {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u24(self.id)?;
        writer.write_u8(self.sublayer)?;
        writer.write_u8(self.flags)?;
        writer.write_u8(self.opacity)?;
        writer.write_u8(self.blend)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let id = reader.read_u24()?;
        let sublayer = reader.read_u8()?;
        let flags = reader.read_u8()?;
        let opacity = reader.read_u8()?;
        let blend = reader.read_u8()?;
        Ok(Layerattributes {
            id,
            sublayer,
            flags,
            opacity,
            blend,
        })
    }

    fn serialized_size(&self) -> usize {
        3 + 1 + 1 + 1 + 1
    }
}

impl Serializable for Layerretitle {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u24(self.id)?;
        let bytes = self.title.as_bytes();
        writer.write_u16(bytes.len() as u16)?;
        writer.write_all(bytes)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let id = reader.read_u24()?;
        let len = reader.read_u16()? as usize;
        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf)?;
        let title = String::from_utf8(buf)?;
        Ok(Layerretitle { id, title })
    }

    fn serialized_size(&self) -> usize {
        3 + 2 + self.title.len()
    }
}

impl Serializable for Putimage {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u24(self.layer)?;
        writer.write_u8(self.mode)?;
        writer.write_u32(self.x)?;
        writer.write_u32(self.y)?;
        writer.write_u32(self.w)?;
        writer.write_u32(self.h)?;
        writer.write_u16(self.image.len() as u16)?;
        writer.write_all(&self.image)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let layer = reader.read_u24()?;
        let mode = reader.read_u8()?;
        let x = reader.read_u32()?;
        let y = reader.read_u32()?;
        let w = reader.read_u32()?;
        let h = reader.read_u32()?;
        let len = reader.read_u16()? as usize;
        let mut image = vec![0u8; len];
        reader.read_exact(&mut image)?;
        Ok(Putimage {
            layer,
            mode,
            x,
            y,
            w,
            h,
            image,
        })
    }

    fn serialized_size(&self) -> usize {
        3 + 1 + 4 + 4 + 4 + 4 + 2 + self.image.len()
    }
}

impl Serializable for Fillrect {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u24(self.layer)?;
        writer.write_u8(self.mode)?;
        writer.write_u32(self.x)?;
        writer.write_u32(self.y)?;
        writer.write_u32(self.w)?;
        writer.write_u32(self.h)?;
        writer.write_u32(self.color)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let layer = reader.read_u24()?;
        let mode = reader.read_u8()?;
        let x = reader.read_u32()?;
        let y = reader.read_u32()?;
        let w = reader.read_u32()?;
        let h = reader.read_u32()?;
        let color = reader.read_u32()?;
        Ok(Fillrect {
            layer,
            mode,
            x,
            y,
            w,
            h,
            color,
        })
    }

    fn serialized_size(&self) -> usize {
        3 + 1 + 4 + 4 + 4 + 4 + 4
    }
}

impl Serializable for Penup {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u24(self.layer)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let layer = reader.read_u24()?;
        Ok(Penup { layer })
    }

    fn serialized_size(&self) -> usize {
        3
    }
}

impl Serializable for Annotationcreate {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u16(self.id)?;
        writer.write_i32(self.x)?;
        writer.write_i32(self.y)?;
        writer.write_u16(self.w)?;
        writer.write_u16(self.h)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let id = reader.read_u16()?;
        let x = reader.read_i32()?;
        let y = reader.read_i32()?;
        let w = reader.read_u16()?;
        let h = reader.read_u16()?;
        Ok(Annotationcreate { id, x, y, w, h })
    }

    fn serialized_size(&self) -> usize {
        2 + 4 + 4 + 2 + 2
    }
}

impl Serializable for Annotationreshape {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u16(self.id)?;
        writer.write_i32(self.x)?;
        writer.write_i32(self.y)?;
        writer.write_u16(self.w)?;
        writer.write_u16(self.h)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let id = reader.read_u16()?;
        let x = reader.read_i32()?;
        let y = reader.read_i32()?;
        let w = reader.read_u16()?;
        let h = reader.read_u16()?;
        Ok(Annotationreshape { id, x, y, w, h })
    }

    fn serialized_size(&self) -> usize {
        2 + 4 + 4 + 2 + 2
    }
}

impl Serializable for Annotationedit {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u16(self.id)?;
        writer.write_u32(self.bg)?;
        writer.write_u8(self.flags)?;
        writer.write_u8(self.border)?;
        let bytes = self.text.as_bytes();
        writer.write_u16(bytes.len() as u16)?;
        writer.write_all(bytes)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let id = reader.read_u16()?;
        let bg = reader.read_u32()?;
        let flags = reader.read_u8()?;
        let border = reader.read_u8()?;
        let len = reader.read_u16()? as usize;
        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf)?;
        let text = String::from_utf8(buf)?;
        Ok(Annotationedit {
            id,
            bg,
            flags,
            border,
            text,
        })
    }

    fn serialized_size(&self) -> usize {
        2 + 4 + 1 + 1 + 2 + self.text.len()
    }
}

impl Serializable for Annotationdelete {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u16(self.id)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let id = reader.read_u16()?;
        Ok(Annotationdelete { id })
    }

    fn serialized_size(&self) -> usize {
        2
    }
}

impl Serializable for Puttile {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(self.user)?;
        writer.write_u24(self.layer)?;
        writer.write_u8(self.sublayer)?;
        writer.write_u16(self.col)?;
        writer.write_u16(self.row)?;
        writer.write_u16(self.repeat)?;
        writer.write_u16(self.image.len() as u16)?;
        writer.write_all(&self.image)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let user = reader.read_u8()?;
        let layer = reader.read_u24()?;
        let sublayer = reader.read_u8()?;
        let col = reader.read_u16()?;
        let row = reader.read_u16()?;
        let repeat = reader.read_u16()?;
        let len = reader.read_u16()? as usize;
        let mut image = vec![0u8; len];
        reader.read_exact(&mut image)?;
        Ok(Puttile {
            user,
            layer,
            sublayer,
            col,
            row,
            repeat,
            image,
        })
    }

    fn serialized_size(&self) -> usize {
        1 + 3 + 1 + 2 + 2 + 2 + 2 + self.image.len()
    }
}

impl Serializable for Canvasbackground {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u16(self.image.len() as u16)?;
        writer.write_all(&self.image)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let len = reader.read_u16()? as usize;
        let mut image = vec![0u8; len];
        reader.read_exact(&mut image)?;
        Ok(Canvasbackground { image })
    }

    fn serialized_size(&self) -> usize {
        2 + self.image.len()
    }
}

impl Serializable for Drawdabsclassic {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(self.flags)?;
        writer.write_u24(self.layer)?;
        writer.write_i32(self.x)?;
        writer.write_i32(self.y)?;
        writer.write_u32(self.color)?;
        writer.write_u8(self.mode)?;
        writer.write_u16(self.dabs.len() as u16)?;
        for item in &self.dabs {
            item.serialize(writer)?;
        }
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let flags = reader.read_u8()?;
        let layer = reader.read_u24()?;
        let x = reader.read_i32()?;
        let y = reader.read_i32()?;
        let color = reader.read_u32()?;
        let mode = reader.read_u8()?;
        let len = reader.read_u16()? as usize;
        let mut dabs = Vec::with_capacity(len);
        for _ in 0..len {
            dabs.push(Classicdab::deserialize(reader)?);
        }
        Ok(Drawdabsclassic {
            flags,
            layer,
            x,
            y,
            color,
            mode,
            dabs,
        })
    }

    fn serialized_size(&self) -> usize {
        1 + 3
            + 4
            + 4
            + 4
            + 1
            + 2
            + self
                .dabs
                .iter()
                .map(|item| item.serialized_size())
                .sum::<usize>()
    }
}

impl Serializable for Drawdabspixel {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(self.flags)?;
        writer.write_u24(self.layer)?;
        writer.write_i32(self.x)?;
        writer.write_i32(self.y)?;
        writer.write_u32(self.color)?;
        writer.write_u8(self.mode)?;
        writer.write_u16(self.dabs.len() as u16)?;
        for item in &self.dabs {
            item.serialize(writer)?;
        }
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let flags = reader.read_u8()?;
        let layer = reader.read_u24()?;
        let x = reader.read_i32()?;
        let y = reader.read_i32()?;
        let color = reader.read_u32()?;
        let mode = reader.read_u8()?;
        let len = reader.read_u16()? as usize;
        let mut dabs = Vec::with_capacity(len);
        for _ in 0..len {
            dabs.push(Pixeldab::deserialize(reader)?);
        }
        Ok(Drawdabspixel {
            flags,
            layer,
            x,
            y,
            color,
            mode,
            dabs,
        })
    }

    fn serialized_size(&self) -> usize {
        1 + 3
            + 4
            + 4
            + 4
            + 1
            + 2
            + self
                .dabs
                .iter()
                .map(|item| item.serialized_size())
                .sum::<usize>()
    }
}

impl Serializable for Drawdabsmypaint {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(self.flags)?;
        writer.write_u24(self.layer)?;
        writer.write_i32(self.x)?;
        writer.write_i32(self.y)?;
        writer.write_u32(self.color)?;
        writer.write_u8(self.lock_alpha)?;
        writer.write_u8(self.colorize)?;
        writer.write_u8(self.posterize)?;
        writer.write_u8(self.mode)?;
        writer.write_u16(self.dabs.len() as u16)?;
        for item in &self.dabs {
            item.serialize(writer)?;
        }
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let flags = reader.read_u8()?;
        let layer = reader.read_u24()?;
        let x = reader.read_i32()?;
        let y = reader.read_i32()?;
        let color = reader.read_u32()?;
        let lock_alpha = reader.read_u8()?;
        let colorize = reader.read_u8()?;
        let posterize = reader.read_u8()?;
        let mode = reader.read_u8()?;
        let len = reader.read_u16()? as usize;
        let mut dabs = Vec::with_capacity(len);
        for _ in 0..len {
            dabs.push(Mypaintdab::deserialize(reader)?);
        }
        Ok(Drawdabsmypaint {
            flags,
            layer,
            x,
            y,
            color,
            lock_alpha,
            colorize,
            posterize,
            mode,
            dabs,
        })
    }

    fn serialized_size(&self) -> usize {
        1 + 3
            + 4
            + 4
            + 4
            + 1
            + 1
            + 1
            + 1
            + 2
            + self
                .dabs
                .iter()
                .map(|item| item.serialized_size())
                .sum::<usize>()
    }
}

impl Serializable for Drawdabsmypaintblend {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(self.flags)?;
        writer.write_u24(self.layer)?;
        writer.write_i32(self.x)?;
        writer.write_i32(self.y)?;
        writer.write_u32(self.color)?;
        writer.write_u8(self.mode)?;
        writer.write_u16(self.dabs.len() as u16)?;
        for item in &self.dabs {
            item.serialize(writer)?;
        }
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let flags = reader.read_u8()?;
        let layer = reader.read_u24()?;
        let x = reader.read_i32()?;
        let y = reader.read_i32()?;
        let color = reader.read_u32()?;
        let mode = reader.read_u8()?;
        let len = reader.read_u16()? as usize;
        let mut dabs = Vec::with_capacity(len);
        for _ in 0..len {
            dabs.push(Mypaintblenddab::deserialize(reader)?);
        }
        Ok(Drawdabsmypaintblend {
            flags,
            layer,
            x,
            y,
            color,
            mode,
            dabs,
        })
    }

    fn serialized_size(&self) -> usize {
        1 + 3
            + 4
            + 4
            + 4
            + 1
            + 2
            + self
                .dabs
                .iter()
                .map(|item| item.serialized_size())
                .sum::<usize>()
    }
}

impl Serializable for Moverect {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u24(self.layer)?;
        writer.write_u24(self.source)?;
        writer.write_i32(self.sx)?;
        writer.write_i32(self.sy)?;
        writer.write_i32(self.tx)?;
        writer.write_i32(self.ty)?;
        writer.write_i32(self.w)?;
        writer.write_i32(self.h)?;
        writer.write_u8(self.blend)?;
        writer.write_u8(self.opacity)?;
        writer.write_u16(self.mask.len() as u16)?;
        writer.write_all(&self.mask)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let layer = reader.read_u24()?;
        let source = reader.read_u24()?;
        let sx = reader.read_i32()?;
        let sy = reader.read_i32()?;
        let tx = reader.read_i32()?;
        let ty = reader.read_i32()?;
        let w = reader.read_i32()?;
        let h = reader.read_i32()?;
        let blend = reader.read_u8()?;
        let opacity = reader.read_u8()?;
        let len = reader.read_u16()? as usize;
        let mut mask = vec![0u8; len];
        reader.read_exact(&mut mask)?;
        Ok(Moverect {
            layer,
            source,
            sx,
            sy,
            tx,
            ty,
            w,
            h,
            blend,
            opacity,
            mask,
        })
    }

    fn serialized_size(&self) -> usize {
        3 + 3 + 4 + 4 + 4 + 4 + 4 + 4 + 1 + 1 + 2 + self.mask.len()
    }
}

impl Serializable for Setmetadataint {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(self.field as u8)?;
        writer.write_i32(self.value)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let field = SetmetadataintField::from_u8(reader.read_u8()?)?;
        let value = reader.read_i32()?;
        Ok(Setmetadataint { field, value })
    }

    fn serialized_size(&self) -> usize {
        1 + 4
    }
}

impl Serializable for Layertreecreate {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u24(self.id)?;
        writer.write_u24(self.source)?;
        writer.write_u24(self.target)?;
        writer.write_u32(self.fill)?;
        writer.write_u8(self.flags)?;
        let bytes = self.title.as_bytes();
        writer.write_u16(bytes.len() as u16)?;
        writer.write_all(bytes)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let id = reader.read_u24()?;
        let source = reader.read_u24()?;
        let target = reader.read_u24()?;
        let fill = reader.read_u32()?;
        let flags = reader.read_u8()?;
        let len = reader.read_u16()? as usize;
        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf)?;
        let title = String::from_utf8(buf)?;
        Ok(Layertreecreate {
            id,
            source,
            target,
            fill,
            flags,
            title,
        })
    }

    fn serialized_size(&self) -> usize {
        3 + 3 + 3 + 4 + 1 + 2 + self.title.len()
    }
}

impl Serializable for Layertreemove {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u24(self.layer)?;
        writer.write_u24(self.parent)?;
        writer.write_u24(self.sibling)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let layer = reader.read_u24()?;
        let parent = reader.read_u24()?;
        let sibling = reader.read_u24()?;
        Ok(Layertreemove {
            layer,
            parent,
            sibling,
        })
    }

    fn serialized_size(&self) -> usize {
        3 + 3 + 3
    }
}

impl Serializable for Layertreedelete {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u24(self.id)?;
        writer.write_u24(self.merge_to)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let id = reader.read_u24()?;
        let merge_to = reader.read_u24()?;
        Ok(Layertreedelete { id, merge_to })
    }

    fn serialized_size(&self) -> usize {
        3 + 3
    }
}

impl Serializable for Transformregion {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u24(self.layer)?;
        writer.write_u24(self.source)?;
        writer.write_i32(self.bx)?;
        writer.write_i32(self.by)?;
        writer.write_i32(self.bw)?;
        writer.write_i32(self.bh)?;
        writer.write_i32(self.x1)?;
        writer.write_i32(self.y1)?;
        writer.write_i32(self.x2)?;
        writer.write_i32(self.y2)?;
        writer.write_i32(self.x3)?;
        writer.write_i32(self.y3)?;
        writer.write_i32(self.x4)?;
        writer.write_i32(self.y4)?;
        writer.write_u8(self.mode as u8)?;
        writer.write_u8(self.blend)?;
        writer.write_u8(self.opacity)?;
        writer.write_u16(self.mask.len() as u16)?;
        writer.write_all(&self.mask)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let layer = reader.read_u24()?;
        let source = reader.read_u24()?;
        let bx = reader.read_i32()?;
        let by = reader.read_i32()?;
        let bw = reader.read_i32()?;
        let bh = reader.read_i32()?;
        let x1 = reader.read_i32()?;
        let y1 = reader.read_i32()?;
        let x2 = reader.read_i32()?;
        let y2 = reader.read_i32()?;
        let x3 = reader.read_i32()?;
        let y3 = reader.read_i32()?;
        let x4 = reader.read_i32()?;
        let y4 = reader.read_i32()?;
        let mode = TransformregionMode::from_u8(reader.read_u8()?)?;
        let blend = reader.read_u8()?;
        let opacity = reader.read_u8()?;
        let len = reader.read_u16()? as usize;
        let mut mask = vec![0u8; len];
        reader.read_exact(&mut mask)?;
        Ok(Transformregion {
            layer,
            source,
            bx,
            by,
            bw,
            bh,
            x1,
            y1,
            x2,
            y2,
            x3,
            y3,
            x4,
            y4,
            mode,
            blend,
            opacity,
            mask,
        })
    }

    fn serialized_size(&self) -> usize {
        3 + 3 + 4 + 4 + 4 + 4 + 4 + 4 + 4 + 4 + 4 + 4 + 4 + 4 + 1 + 1 + 1 + 2 + self.mask.len()
    }
}

impl Serializable for Trackcreate {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u16(self.id)?;
        writer.write_u16(self.insert_id)?;
        writer.write_u16(self.source_id)?;
        let bytes = self.title.as_bytes();
        writer.write_u16(bytes.len() as u16)?;
        writer.write_all(bytes)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let id = reader.read_u16()?;
        let insert_id = reader.read_u16()?;
        let source_id = reader.read_u16()?;
        let len = reader.read_u16()? as usize;
        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf)?;
        let title = String::from_utf8(buf)?;
        Ok(Trackcreate {
            id,
            insert_id,
            source_id,
            title,
        })
    }

    fn serialized_size(&self) -> usize {
        2 + 2 + 2 + 2 + self.title.len()
    }
}

impl Serializable for Trackretitle {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u16(self.id)?;
        let bytes = self.title.as_bytes();
        writer.write_u16(bytes.len() as u16)?;
        writer.write_all(bytes)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let id = reader.read_u16()?;
        let len = reader.read_u16()? as usize;
        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf)?;
        let title = String::from_utf8(buf)?;
        Ok(Trackretitle { id, title })
    }

    fn serialized_size(&self) -> usize {
        2 + 2 + self.title.len()
    }
}

impl Serializable for Trackdelete {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u16(self.id)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let id = reader.read_u16()?;
        Ok(Trackdelete { id })
    }

    fn serialized_size(&self) -> usize {
        2
    }
}

impl Serializable for Trackorder {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u16(self.tracks.len() as u16)?;
        for item in &self.tracks {
            writer.write_u16(*item)?;
        }
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let len = reader.read_u16()? as usize;
        let mut tracks = Vec::with_capacity(len);
        for _ in 0..len {
            tracks.push(reader.read_u16()?);
        }
        Ok(Trackorder { tracks })
    }

    fn serialized_size(&self) -> usize {
        2 + self.tracks.len() * 2
    }
}

impl Serializable for Keyframeset {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u16(self.track_id)?;
        writer.write_u16(self.frame_index)?;
        writer.write_u24(self.source_id)?;
        writer.write_u16(self.source_index)?;
        writer.write_u8(self.source as u8)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let track_id = reader.read_u16()?;
        let frame_index = reader.read_u16()?;
        let source_id = reader.read_u24()?;
        let source_index = reader.read_u16()?;
        let source = KeyframesetSource::from_u8(reader.read_u8()?)?;
        Ok(Keyframeset {
            track_id,
            frame_index,
            source_id,
            source_index,
            source,
        })
    }

    fn serialized_size(&self) -> usize {
        2 + 2 + 3 + 2 + 1
    }
}

impl Serializable for Keyframeretitle {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u16(self.track_id)?;
        writer.write_u16(self.frame_index)?;
        let bytes = self.title.as_bytes();
        writer.write_u16(bytes.len() as u16)?;
        writer.write_all(bytes)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let track_id = reader.read_u16()?;
        let frame_index = reader.read_u16()?;
        let len = reader.read_u16()? as usize;
        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf)?;
        let title = String::from_utf8(buf)?;
        Ok(Keyframeretitle {
            track_id,
            frame_index,
            title,
        })
    }

    fn serialized_size(&self) -> usize {
        2 + 2 + 2 + self.title.len()
    }
}

impl Serializable for Keyframelayerattributes {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u16(self.track_id)?;
        writer.write_u16(self.frame_index)?;
        writer.write_u16(self.layer_flags.len() as u16)?;
        for item in &self.layer_flags {
            writer.write_u32(*item)?;
        }
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let track_id = reader.read_u16()?;
        let frame_index = reader.read_u16()?;
        let len = reader.read_u16()? as usize;
        let mut layer_flags = Vec::with_capacity(len);
        for _ in 0..len {
            layer_flags.push(reader.read_u32()?);
        }
        Ok(Keyframelayerattributes {
            track_id,
            frame_index,
            layer_flags,
        })
    }

    fn serialized_size(&self) -> usize {
        2 + 2 + 2 + self.layer_flags.len() * 4
    }
}

impl Serializable for Keyframedelete {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u16(self.track_id)?;
        writer.write_u16(self.frame_index)?;
        writer.write_u16(self.move_track_id)?;
        writer.write_u16(self.move_frame_index)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let track_id = reader.read_u16()?;
        let frame_index = reader.read_u16()?;
        let move_track_id = reader.read_u16()?;
        let move_frame_index = reader.read_u16()?;
        Ok(Keyframedelete {
            track_id,
            frame_index,
            move_track_id,
            move_frame_index,
        })
    }

    fn serialized_size(&self) -> usize {
        2 + 2 + 2 + 2
    }
}

impl Serializable for Selectionput {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(self.selection_id)?;
        writer.write_u8(self.op as u8)?;
        writer.write_i32(self.x)?;
        writer.write_i32(self.y)?;
        writer.write_u32(self.w)?;
        writer.write_u32(self.h)?;
        writer.write_u16(self.mask.len() as u16)?;
        writer.write_all(&self.mask)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let selection_id = reader.read_u8()?;
        let op = SelectionputOp::from_u8(reader.read_u8()?)?;
        let x = reader.read_i32()?;
        let y = reader.read_i32()?;
        let w = reader.read_u32()?;
        let h = reader.read_u32()?;
        let len = reader.read_u16()? as usize;
        let mut mask = vec![0u8; len];
        reader.read_exact(&mut mask)?;
        Ok(Selectionput {
            selection_id,
            op,
            x,
            y,
            w,
            h,
            mask,
        })
    }

    fn serialized_size(&self) -> usize {
        1 + 1 + 4 + 4 + 4 + 4 + 2 + self.mask.len()
    }
}

impl Serializable for Selectionclear {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(self.selection_id)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let selection_id = reader.read_u8()?;
        Ok(Selectionclear { selection_id })
    }

    fn serialized_size(&self) -> usize {
        1
    }
}

impl Serializable for Localmatch {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(self.msg_type)?;
        writer.write_u16(self.data.len() as u16)?;
        writer.write_all(&self.data)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let msg_type = reader.read_u8()?;
        let len = reader.read_u16()? as usize;
        let mut data = vec![0u8; len];
        reader.read_exact(&mut data)?;
        Ok(Localmatch { msg_type, data })
    }

    fn serialized_size(&self) -> usize {
        1 + 2 + self.data.len()
    }
}

impl Serializable for Syncselectiontile {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(self.user)?;
        writer.write_u8(self.selection_id)?;
        writer.write_u16(self.col)?;
        writer.write_u16(self.row)?;
        writer.write_u16(self.mask.len() as u16)?;
        writer.write_all(&self.mask)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let user = reader.read_u8()?;
        let selection_id = reader.read_u8()?;
        let col = reader.read_u16()?;
        let row = reader.read_u16()?;
        let len = reader.read_u16()? as usize;
        let mut mask = vec![0u8; len];
        reader.read_exact(&mut mask)?;
        Ok(Syncselectiontile {
            user,
            selection_id,
            col,
            row,
            mask,
        })
    }

    fn serialized_size(&self) -> usize {
        1 + 1 + 2 + 2 + 2 + self.mask.len()
    }
}

impl Serializable for Undo {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(self.override_user)?;
        writer.write_u8(if self.redo { 1 } else { 0 })?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let override_user = reader.read_u8()?;
        let redo = reader.read_u8()? != 0;
        Ok(Undo {
            override_user,
            redo,
        })
    }

    fn serialized_size(&self) -> usize {
        1 + 1
    }
}

impl Serializable for Classicdab {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_i8(self.x)?;
        writer.write_i8(self.y)?;
        writer.write_u24(self.size)?;
        writer.write_u8(self.hardness)?;
        writer.write_u8(self.opacity)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let x = reader.read_i8()?;
        let y = reader.read_i8()?;
        let size = reader.read_u24()?;
        let hardness = reader.read_u8()?;
        let opacity = reader.read_u8()?;
        Ok(Classicdab {
            x,
            y,
            size,
            hardness,
            opacity,
        })
    }

    fn serialized_size(&self) -> usize {
        1 + 1 + 3 + 1 + 1
    }
}

impl Serializable for Pixeldab {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_i8(self.x)?;
        writer.write_i8(self.y)?;
        writer.write_u16(self.size)?;
        writer.write_u8(self.opacity)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let x = reader.read_i8()?;
        let y = reader.read_i8()?;
        let size = reader.read_u16()?;
        let opacity = reader.read_u8()?;
        Ok(Pixeldab {
            x,
            y,
            size,
            opacity,
        })
    }

    fn serialized_size(&self) -> usize {
        1 + 1 + 2 + 1
    }
}

impl Serializable for Mypaintdab {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_i8(self.x)?;
        writer.write_i8(self.y)?;
        writer.write_u24(self.size)?;
        writer.write_u8(self.hardness)?;
        writer.write_u8(self.opacity)?;
        writer.write_u8(self.angle)?;
        writer.write_u8(self.aspect_ratio)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let x = reader.read_i8()?;
        let y = reader.read_i8()?;
        let size = reader.read_u24()?;
        let hardness = reader.read_u8()?;
        let opacity = reader.read_u8()?;
        let angle = reader.read_u8()?;
        let aspect_ratio = reader.read_u8()?;
        Ok(Mypaintdab {
            x,
            y,
            size,
            hardness,
            opacity,
            angle,
            aspect_ratio,
        })
    }

    fn serialized_size(&self) -> usize {
        1 + 1 + 3 + 1 + 1 + 1 + 1
    }
}

impl Serializable for Mypaintblenddab {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        writer.write_i8(self.x)?;
        writer.write_i8(self.y)?;
        writer.write_u24(self.size)?;
        writer.write_u8(self.hardness)?;
        writer.write_u8(self.opacity)?;
        writer.write_u8(self.angle)?;
        writer.write_u8(self.aspect_ratio)?;
        Ok(())
    }

    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let x = reader.read_i8()?;
        let y = reader.read_i8()?;
        let size = reader.read_u24()?;
        let hardness = reader.read_u8()?;
        let opacity = reader.read_u8()?;
        let angle = reader.read_u8()?;
        let aspect_ratio = reader.read_u8()?;
        Ok(Mypaintblenddab {
            x,
            y,
            size,
            hardness,
            opacity,
            angle,
            aspect_ratio,
        })
    }

    fn serialized_size(&self) -> usize {
        1 + 1 + 3 + 1 + 1 + 1 + 1
    }
}

impl Message {
    pub fn message_type(&self) -> MessageType {
        match self {
            Message::Servercommand(_) => MessageType::SERVER_COMMAND,
            Message::Disconnect(_) => MessageType::DISCONNECT,
            Message::Ping(_) => MessageType::PING,
            Message::Keepalive(_) => MessageType::KEEP_ALIVE,
            Message::Thumbnail(_) => MessageType::THUMBNAIL,
            Message::Join(_) => MessageType::JOIN,
            Message::Leave(_) => MessageType::LEAVE,
            Message::Sessionowner(_) => MessageType::SESSION_OWNER,
            Message::Chat(_) => MessageType::CHAT,
            Message::Trustedusers(_) => MessageType::TRUSTED_USERS,
            Message::Softreset(_) => MessageType::SOFT_RESET,
            Message::Privatechat(_) => MessageType::PRIVATE_CHAT,
            Message::Resetstream(_) => MessageType::RESET_STREAM,
            Message::Interval(_) => MessageType::INTERVAL,
            Message::Lasertrail(_) => MessageType::LASER_TRAIL,
            Message::Movepointer(_) => MessageType::MOVE_POINTER,
            Message::Useracl(_) => MessageType::USER_ACL,
            Message::Layeracl(_) => MessageType::LAYER_ACL,
            Message::Featureaccesslevels(_) => MessageType::FEATURE_ACCESS_LEVELS,
            Message::Defaultlayer(_) => MessageType::DEFAULT_LAYER,
            Message::Undodepth(_) => MessageType::UNDO_DEPTH,
            Message::Data(_) => MessageType::DATA,
            Message::Localchange(_) => MessageType::LOCAL_CHANGE,
            Message::Featurelimits(_) => MessageType::FEATURE_LIMITS,
            Message::Undopoint(_) => MessageType::UNDO_POINT,
            Message::Canvasresize(_) => MessageType::CANVAS_RESIZE,
            Message::Layerattributes(_) => MessageType::LAYER_ATTRIBUTES,
            Message::Layerretitle(_) => MessageType::LAYER_RETITLE,
            Message::Putimage(_) => MessageType::PUT_IMAGE,
            Message::Fillrect(_) => MessageType::FILL_RECT,
            Message::Penup(_) => MessageType::PEN_UP,
            Message::Annotationcreate(_) => MessageType::ANNOTATION_CREATE,
            Message::Annotationreshape(_) => MessageType::ANNOTATION_RESHAPE,
            Message::Annotationedit(_) => MessageType::ANNOTATION_EDIT,
            Message::Annotationdelete(_) => MessageType::ANNOTATION_DELETE,
            Message::Puttile(_) => MessageType::PUT_TILE,
            Message::Canvasbackground(_) => MessageType::CANVAS_BACKGROUND,
            Message::Drawdabsclassic(_) => MessageType::DRAW_DABS_CLASSIC,
            Message::Drawdabspixel(_) => MessageType::DRAW_DABS_PIXEL,
            Message::Drawdabsmypaint(_) => MessageType::DRAW_DABS_MY_PAINT,
            Message::Drawdabsmypaintblend(_) => MessageType::DRAW_DABS_MY_PAINT_BLEND,
            Message::Moverect(_) => MessageType::MOVE_RECT,
            Message::Setmetadataint(_) => MessageType::SET_METADATA_INT,
            Message::Layertreecreate(_) => MessageType::LAYER_TREE_CREATE,
            Message::Layertreemove(_) => MessageType::LAYER_TREE_MOVE,
            Message::Layertreedelete(_) => MessageType::LAYER_TREE_DELETE,
            Message::Transformregion(_) => MessageType::TRANSFORM_REGION,
            Message::Trackcreate(_) => MessageType::TRACK_CREATE,
            Message::Trackretitle(_) => MessageType::TRACK_RETITLE,
            Message::Trackdelete(_) => MessageType::TRACK_DELETE,
            Message::Trackorder(_) => MessageType::TRACK_ORDER,
            Message::Keyframeset(_) => MessageType::KEY_FRAME_SET,
            Message::Keyframeretitle(_) => MessageType::KEY_FRAME_RETITLE,
            Message::Keyframelayerattributes(_) => MessageType::KEY_FRAME_LAYER_ATTRIBUTES,
            Message::Keyframedelete(_) => MessageType::KEY_FRAME_DELETE,
            Message::Selectionput(_) => MessageType::SELECTION_PUT,
            Message::Selectionclear(_) => MessageType::SELECTION_CLEAR,
            Message::Localmatch(_) => MessageType::LOCAL_MATCH,
            Message::Syncselectiontile(_) => MessageType::SYNC_SELECTION_TILE,
            Message::Undo(_) => MessageType::UNDO,
        }
    }

    pub fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        // Write message type first
        writer.write_u8(self.message_type() as u8)?;

        // Write message payload
        match self {
            Message::Servercommand(msg) => msg.serialize(writer),
            Message::Disconnect(msg) => msg.serialize(writer),
            Message::Ping(msg) => msg.serialize(writer),
            Message::Keepalive(msg) => msg.serialize(writer),
            Message::Thumbnail(msg) => msg.serialize(writer),
            Message::Join(msg) => msg.serialize(writer),
            Message::Leave(msg) => msg.serialize(writer),
            Message::Sessionowner(msg) => msg.serialize(writer),
            Message::Chat(msg) => msg.serialize(writer),
            Message::Trustedusers(msg) => msg.serialize(writer),
            Message::Softreset(msg) => msg.serialize(writer),
            Message::Privatechat(msg) => msg.serialize(writer),
            Message::Resetstream(msg) => msg.serialize(writer),
            Message::Interval(msg) => msg.serialize(writer),
            Message::Lasertrail(msg) => msg.serialize(writer),
            Message::Movepointer(msg) => msg.serialize(writer),
            Message::Useracl(msg) => msg.serialize(writer),
            Message::Layeracl(msg) => msg.serialize(writer),
            Message::Featureaccesslevels(msg) => msg.serialize(writer),
            Message::Defaultlayer(msg) => msg.serialize(writer),
            Message::Undodepth(msg) => msg.serialize(writer),
            Message::Data(msg) => msg.serialize(writer),
            Message::Localchange(msg) => msg.serialize(writer),
            Message::Featurelimits(msg) => msg.serialize(writer),
            Message::Undopoint(msg) => msg.serialize(writer),
            Message::Canvasresize(msg) => msg.serialize(writer),
            Message::Layerattributes(msg) => msg.serialize(writer),
            Message::Layerretitle(msg) => msg.serialize(writer),
            Message::Putimage(msg) => msg.serialize(writer),
            Message::Fillrect(msg) => msg.serialize(writer),
            Message::Penup(msg) => msg.serialize(writer),
            Message::Annotationcreate(msg) => msg.serialize(writer),
            Message::Annotationreshape(msg) => msg.serialize(writer),
            Message::Annotationedit(msg) => msg.serialize(writer),
            Message::Annotationdelete(msg) => msg.serialize(writer),
            Message::Puttile(msg) => msg.serialize(writer),
            Message::Canvasbackground(msg) => msg.serialize(writer),
            Message::Drawdabsclassic(msg) => msg.serialize(writer),
            Message::Drawdabspixel(msg) => msg.serialize(writer),
            Message::Drawdabsmypaint(msg) => msg.serialize(writer),
            Message::Drawdabsmypaintblend(msg) => msg.serialize(writer),
            Message::Moverect(msg) => msg.serialize(writer),
            Message::Setmetadataint(msg) => msg.serialize(writer),
            Message::Layertreecreate(msg) => msg.serialize(writer),
            Message::Layertreemove(msg) => msg.serialize(writer),
            Message::Layertreedelete(msg) => msg.serialize(writer),
            Message::Transformregion(msg) => msg.serialize(writer),
            Message::Trackcreate(msg) => msg.serialize(writer),
            Message::Trackretitle(msg) => msg.serialize(writer),
            Message::Trackdelete(msg) => msg.serialize(writer),
            Message::Trackorder(msg) => msg.serialize(writer),
            Message::Keyframeset(msg) => msg.serialize(writer),
            Message::Keyframeretitle(msg) => msg.serialize(writer),
            Message::Keyframelayerattributes(msg) => msg.serialize(writer),
            Message::Keyframedelete(msg) => msg.serialize(writer),
            Message::Selectionput(msg) => msg.serialize(writer),
            Message::Selectionclear(msg) => msg.serialize(writer),
            Message::Localmatch(msg) => msg.serialize(writer),
            Message::Syncselectiontile(msg) => msg.serialize(writer),
            Message::Undo(msg) => msg.serialize(writer),
        }
    }

    pub fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let msg_type = MessageType::from_u8(reader.read_u8()?)?;

        match msg_type {
            MessageType::SERVER_COMMAND => {
                Ok(Message::Servercommand(Servercommand::deserialize(reader)?))
            }
            MessageType::DISCONNECT => Ok(Message::Disconnect(Disconnect::deserialize(reader)?)),
            MessageType::PING => Ok(Message::Ping(Ping::deserialize(reader)?)),
            MessageType::KEEP_ALIVE => Ok(Message::Keepalive(Keepalive::deserialize(reader)?)),
            MessageType::THUMBNAIL => Ok(Message::Thumbnail(Thumbnail::deserialize(reader)?)),
            MessageType::JOIN => Ok(Message::Join(Join::deserialize(reader)?)),
            MessageType::LEAVE => Ok(Message::Leave(Leave::deserialize(reader)?)),
            MessageType::SESSION_OWNER => {
                Ok(Message::Sessionowner(Sessionowner::deserialize(reader)?))
            }
            MessageType::CHAT => Ok(Message::Chat(Chat::deserialize(reader)?)),
            MessageType::TRUSTED_USERS => {
                Ok(Message::Trustedusers(Trustedusers::deserialize(reader)?))
            }
            MessageType::SOFT_RESET => Ok(Message::Softreset(Softreset::deserialize(reader)?)),
            MessageType::PRIVATE_CHAT => {
                Ok(Message::Privatechat(Privatechat::deserialize(reader)?))
            }
            MessageType::RESET_STREAM => {
                Ok(Message::Resetstream(Resetstream::deserialize(reader)?))
            }
            MessageType::INTERVAL => Ok(Message::Interval(Interval::deserialize(reader)?)),
            MessageType::LASER_TRAIL => Ok(Message::Lasertrail(Lasertrail::deserialize(reader)?)),
            MessageType::MOVE_POINTER => {
                Ok(Message::Movepointer(Movepointer::deserialize(reader)?))
            }
            MessageType::USER_ACL => Ok(Message::Useracl(Useracl::deserialize(reader)?)),
            MessageType::LAYER_ACL => Ok(Message::Layeracl(Layeracl::deserialize(reader)?)),
            MessageType::FEATURE_ACCESS_LEVELS => Ok(Message::Featureaccesslevels(
                Featureaccesslevels::deserialize(reader)?,
            )),
            MessageType::DEFAULT_LAYER => {
                Ok(Message::Defaultlayer(Defaultlayer::deserialize(reader)?))
            }
            MessageType::UNDO_DEPTH => Ok(Message::Undodepth(Undodepth::deserialize(reader)?)),
            MessageType::DATA => Ok(Message::Data(Data::deserialize(reader)?)),
            MessageType::LOCAL_CHANGE => {
                Ok(Message::Localchange(Localchange::deserialize(reader)?))
            }
            MessageType::FEATURE_LIMITS => {
                Ok(Message::Featurelimits(Featurelimits::deserialize(reader)?))
            }
            MessageType::UNDO_POINT => Ok(Message::Undopoint(Undopoint::deserialize(reader)?)),
            MessageType::CANVAS_RESIZE => {
                Ok(Message::Canvasresize(Canvasresize::deserialize(reader)?))
            }
            MessageType::LAYER_ATTRIBUTES => Ok(Message::Layerattributes(
                Layerattributes::deserialize(reader)?,
            )),
            MessageType::LAYER_RETITLE => {
                Ok(Message::Layerretitle(Layerretitle::deserialize(reader)?))
            }
            MessageType::PUT_IMAGE => Ok(Message::Putimage(Putimage::deserialize(reader)?)),
            MessageType::FILL_RECT => Ok(Message::Fillrect(Fillrect::deserialize(reader)?)),
            MessageType::PEN_UP => Ok(Message::Penup(Penup::deserialize(reader)?)),
            MessageType::ANNOTATION_CREATE => Ok(Message::Annotationcreate(
                Annotationcreate::deserialize(reader)?,
            )),
            MessageType::ANNOTATION_RESHAPE => Ok(Message::Annotationreshape(
                Annotationreshape::deserialize(reader)?,
            )),
            MessageType::ANNOTATION_EDIT => Ok(Message::Annotationedit(
                Annotationedit::deserialize(reader)?,
            )),
            MessageType::ANNOTATION_DELETE => Ok(Message::Annotationdelete(
                Annotationdelete::deserialize(reader)?,
            )),
            MessageType::PUT_TILE => Ok(Message::Puttile(Puttile::deserialize(reader)?)),
            MessageType::CANVAS_BACKGROUND => Ok(Message::Canvasbackground(
                Canvasbackground::deserialize(reader)?,
            )),
            MessageType::DRAW_DABS_CLASSIC => Ok(Message::Drawdabsclassic(
                Drawdabsclassic::deserialize(reader)?,
            )),
            MessageType::DRAW_DABS_PIXEL => {
                Ok(Message::Drawdabspixel(Drawdabspixel::deserialize(reader)?))
            }
            MessageType::DRAW_DABS_MY_PAINT => Ok(Message::Drawdabsmypaint(
                Drawdabsmypaint::deserialize(reader)?,
            )),
            MessageType::DRAW_DABS_MY_PAINT_BLEND => Ok(Message::Drawdabsmypaintblend(
                Drawdabsmypaintblend::deserialize(reader)?,
            )),
            MessageType::MOVE_RECT => Ok(Message::Moverect(Moverect::deserialize(reader)?)),
            MessageType::SET_METADATA_INT => Ok(Message::Setmetadataint(
                Setmetadataint::deserialize(reader)?,
            )),
            MessageType::LAYER_TREE_CREATE => Ok(Message::Layertreecreate(
                Layertreecreate::deserialize(reader)?,
            )),
            MessageType::LAYER_TREE_MOVE => {
                Ok(Message::Layertreemove(Layertreemove::deserialize(reader)?))
            }
            MessageType::LAYER_TREE_DELETE => Ok(Message::Layertreedelete(
                Layertreedelete::deserialize(reader)?,
            )),
            MessageType::TRANSFORM_REGION => Ok(Message::Transformregion(
                Transformregion::deserialize(reader)?,
            )),
            MessageType::TRACK_CREATE => {
                Ok(Message::Trackcreate(Trackcreate::deserialize(reader)?))
            }
            MessageType::TRACK_RETITLE => {
                Ok(Message::Trackretitle(Trackretitle::deserialize(reader)?))
            }
            MessageType::TRACK_DELETE => {
                Ok(Message::Trackdelete(Trackdelete::deserialize(reader)?))
            }
            MessageType::TRACK_ORDER => Ok(Message::Trackorder(Trackorder::deserialize(reader)?)),
            MessageType::KEY_FRAME_SET => {
                Ok(Message::Keyframeset(Keyframeset::deserialize(reader)?))
            }
            MessageType::KEY_FRAME_RETITLE => Ok(Message::Keyframeretitle(
                Keyframeretitle::deserialize(reader)?,
            )),
            MessageType::KEY_FRAME_LAYER_ATTRIBUTES => Ok(Message::Keyframelayerattributes(
                Keyframelayerattributes::deserialize(reader)?,
            )),
            MessageType::KEY_FRAME_DELETE => Ok(Message::Keyframedelete(
                Keyframedelete::deserialize(reader)?,
            )),
            MessageType::SELECTION_PUT => {
                Ok(Message::Selectionput(Selectionput::deserialize(reader)?))
            }
            MessageType::SELECTION_CLEAR => Ok(Message::Selectionclear(
                Selectionclear::deserialize(reader)?,
            )),
            MessageType::LOCAL_MATCH => Ok(Message::Localmatch(Localmatch::deserialize(reader)?)),
            MessageType::SYNC_SELECTION_TILE => Ok(Message::Syncselectiontile(
                Syncselectiontile::deserialize(reader)?,
            )),
            MessageType::UNDO => Ok(Message::Undo(Undo::deserialize(reader)?)),
        }
    }

    pub fn serialized_size(&self) -> usize {
        1 + match self {
            Message::Servercommand(msg) => msg.serialized_size(),
            Message::Disconnect(msg) => msg.serialized_size(),
            Message::Ping(msg) => msg.serialized_size(),
            Message::Keepalive(msg) => msg.serialized_size(),
            Message::Thumbnail(msg) => msg.serialized_size(),
            Message::Join(msg) => msg.serialized_size(),
            Message::Leave(msg) => msg.serialized_size(),
            Message::Sessionowner(msg) => msg.serialized_size(),
            Message::Chat(msg) => msg.serialized_size(),
            Message::Trustedusers(msg) => msg.serialized_size(),
            Message::Softreset(msg) => msg.serialized_size(),
            Message::Privatechat(msg) => msg.serialized_size(),
            Message::Resetstream(msg) => msg.serialized_size(),
            Message::Interval(msg) => msg.serialized_size(),
            Message::Lasertrail(msg) => msg.serialized_size(),
            Message::Movepointer(msg) => msg.serialized_size(),
            Message::Useracl(msg) => msg.serialized_size(),
            Message::Layeracl(msg) => msg.serialized_size(),
            Message::Featureaccesslevels(msg) => msg.serialized_size(),
            Message::Defaultlayer(msg) => msg.serialized_size(),
            Message::Undodepth(msg) => msg.serialized_size(),
            Message::Data(msg) => msg.serialized_size(),
            Message::Localchange(msg) => msg.serialized_size(),
            Message::Featurelimits(msg) => msg.serialized_size(),
            Message::Undopoint(msg) => msg.serialized_size(),
            Message::Canvasresize(msg) => msg.serialized_size(),
            Message::Layerattributes(msg) => msg.serialized_size(),
            Message::Layerretitle(msg) => msg.serialized_size(),
            Message::Putimage(msg) => msg.serialized_size(),
            Message::Fillrect(msg) => msg.serialized_size(),
            Message::Penup(msg) => msg.serialized_size(),
            Message::Annotationcreate(msg) => msg.serialized_size(),
            Message::Annotationreshape(msg) => msg.serialized_size(),
            Message::Annotationedit(msg) => msg.serialized_size(),
            Message::Annotationdelete(msg) => msg.serialized_size(),
            Message::Puttile(msg) => msg.serialized_size(),
            Message::Canvasbackground(msg) => msg.serialized_size(),
            Message::Drawdabsclassic(msg) => msg.serialized_size(),
            Message::Drawdabspixel(msg) => msg.serialized_size(),
            Message::Drawdabsmypaint(msg) => msg.serialized_size(),
            Message::Drawdabsmypaintblend(msg) => msg.serialized_size(),
            Message::Moverect(msg) => msg.serialized_size(),
            Message::Setmetadataint(msg) => msg.serialized_size(),
            Message::Layertreecreate(msg) => msg.serialized_size(),
            Message::Layertreemove(msg) => msg.serialized_size(),
            Message::Layertreedelete(msg) => msg.serialized_size(),
            Message::Transformregion(msg) => msg.serialized_size(),
            Message::Trackcreate(msg) => msg.serialized_size(),
            Message::Trackretitle(msg) => msg.serialized_size(),
            Message::Trackdelete(msg) => msg.serialized_size(),
            Message::Trackorder(msg) => msg.serialized_size(),
            Message::Keyframeset(msg) => msg.serialized_size(),
            Message::Keyframeretitle(msg) => msg.serialized_size(),
            Message::Keyframelayerattributes(msg) => msg.serialized_size(),
            Message::Keyframedelete(msg) => msg.serialized_size(),
            Message::Selectionput(msg) => msg.serialized_size(),
            Message::Selectionclear(msg) => msg.serialized_size(),
            Message::Localmatch(msg) => msg.serialized_size(),
            Message::Syncselectiontile(msg) => msg.serialized_size(),
            Message::Undo(msg) => msg.serialized_size(),
        }
    }
}
