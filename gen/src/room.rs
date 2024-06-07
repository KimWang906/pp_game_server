// @generated
// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomInfoRequest {
    #[prost(uint64, tag="1")]
    pub id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRoomRequest {
    #[prost(string, tag="1")]
    pub room_name: ::prost::alloc::string::String,
    #[prost(enumeration="MaxRoomSize", tag="2")]
    pub max_size: i32,
    #[prost(string, tag="3")]
    pub owner_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRoomRequest {
    #[prost(uint64, tag="1")]
    pub id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomUserList {
    #[prost(message, repeated, tag="1")]
    pub users: ::prost::alloc::vec::Vec<RoomUser>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserList {
    #[prost(message, repeated, tag="1")]
    pub users: ::prost::alloc::vec::Vec<RoomUser>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomList {
    #[prost(message, repeated, tag="1")]
    pub rooms: ::prost::alloc::vec::Vec<Room>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomUser {
    #[prost(string, tag="1")]
    pub student_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub username: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Room {
    #[prost(enumeration="RoomStatus", optional, tag="1")]
    pub status: ::core::option::Option<i32>,
    #[prost(int64, tag="2")]
    pub id: i64,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="MaxRoomSize", tag="4")]
    pub max_size: i32,
    #[prost(message, optional, tag="5")]
    pub owner: ::core::option::Option<RoomUser>,
    #[prost(message, optional, tag="6")]
    pub members: ::core::option::Option<RoomUserList>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RoomStatus {
    Open = 0,
    Closed = 1,
    Full = 2,
}
impl RoomStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RoomStatus::Open => "OPEN",
            RoomStatus::Closed => "CLOSED",
            RoomStatus::Full => "FULL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPEN" => Some(Self::Open),
            "CLOSED" => Some(Self::Closed),
            "FULL" => Some(Self::Full),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MaxRoomSize {
    DoublePlayers = 0,
    QuadruplePlayers = 1,
}
impl MaxRoomSize {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MaxRoomSize::DoublePlayers => "DOUBLE_PLAYERS",
            MaxRoomSize::QuadruplePlayers => "QUADRUPLE_PLAYERS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DOUBLE_PLAYERS" => Some(Self::DoublePlayers),
            "QUADRUPLE_PLAYERS" => Some(Self::QuadruplePlayers),
            _ => None,
        }
    }
}
/// Encoded file descriptor set for the `room` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xb4, 0x18, 0x0a, 0x0a, 0x72, 0x6f, 0x6f, 0x6d, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x04, 0x72, 0x6f, 0x6f, 0x6d, 0x1a, 0x1b, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x22, 0x21, 0x0a, 0x0f, 0x52, 0x6f, 0x6f, 0x6d, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x04, 0x52, 0x02, 0x69, 0x64, 0x22, 0x7d, 0x0a, 0x11, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x52,
    0x6f, 0x6f, 0x6d, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1b, 0x0a, 0x09, 0x72, 0x6f,
    0x6f, 0x6d, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x72,
    0x6f, 0x6f, 0x6d, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x2c, 0x0a, 0x08, 0x6d, 0x61, 0x78, 0x5f, 0x73,
    0x69, 0x7a, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x11, 0x2e, 0x72, 0x6f, 0x6f, 0x6d,
    0x2e, 0x4d, 0x61, 0x78, 0x52, 0x6f, 0x6f, 0x6d, 0x53, 0x69, 0x7a, 0x65, 0x52, 0x07, 0x6d, 0x61,
    0x78, 0x53, 0x69, 0x7a, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x5f, 0x6e,
    0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x6f, 0x77, 0x6e, 0x65, 0x72,
    0x4e, 0x61, 0x6d, 0x65, 0x22, 0x23, 0x0a, 0x11, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x52, 0x6f,
    0x6f, 0x6d, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x02, 0x69, 0x64, 0x22, 0x34, 0x0a, 0x0c, 0x52, 0x6f, 0x6f,
    0x6d, 0x55, 0x73, 0x65, 0x72, 0x4c, 0x69, 0x73, 0x74, 0x12, 0x24, 0x0a, 0x05, 0x75, 0x73, 0x65,
    0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x72, 0x6f, 0x6f, 0x6d, 0x2e,
    0x52, 0x6f, 0x6f, 0x6d, 0x55, 0x73, 0x65, 0x72, 0x52, 0x05, 0x75, 0x73, 0x65, 0x72, 0x73, 0x22,
    0x30, 0x0a, 0x08, 0x55, 0x73, 0x65, 0x72, 0x4c, 0x69, 0x73, 0x74, 0x12, 0x24, 0x0a, 0x05, 0x75,
    0x73, 0x65, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x72, 0x6f, 0x6f,
    0x6d, 0x2e, 0x52, 0x6f, 0x6f, 0x6d, 0x55, 0x73, 0x65, 0x72, 0x52, 0x05, 0x75, 0x73, 0x65, 0x72,
    0x73, 0x22, 0x2c, 0x0a, 0x08, 0x52, 0x6f, 0x6f, 0x6d, 0x4c, 0x69, 0x73, 0x74, 0x12, 0x20, 0x0a,
    0x05, 0x72, 0x6f, 0x6f, 0x6d, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x72,
    0x6f, 0x6f, 0x6d, 0x2e, 0x52, 0x6f, 0x6f, 0x6d, 0x52, 0x05, 0x72, 0x6f, 0x6f, 0x6d, 0x73, 0x22,
    0x45, 0x0a, 0x08, 0x52, 0x6f, 0x6f, 0x6d, 0x55, 0x73, 0x65, 0x72, 0x12, 0x1d, 0x0a, 0x0a, 0x73,
    0x74, 0x75, 0x64, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x09, 0x73, 0x74, 0x75, 0x64, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x1a, 0x0a, 0x08, 0x75, 0x73,
    0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x75, 0x73,
    0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x22, 0xe6, 0x01, 0x0a, 0x04, 0x52, 0x6f, 0x6f, 0x6d, 0x12,
    0x2d, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x10, 0x2e, 0x72, 0x6f, 0x6f, 0x6d, 0x2e, 0x52, 0x6f, 0x6f, 0x6d, 0x53, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x48, 0x00, 0x52, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x88, 0x01, 0x01, 0x12, 0x0e,
    0x0a, 0x02, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x52, 0x02, 0x69, 0x64, 0x12, 0x12,
    0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61,
    0x6d, 0x65, 0x12, 0x2c, 0x0a, 0x08, 0x6d, 0x61, 0x78, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x0e, 0x32, 0x11, 0x2e, 0x72, 0x6f, 0x6f, 0x6d, 0x2e, 0x4d, 0x61, 0x78, 0x52,
    0x6f, 0x6f, 0x6d, 0x53, 0x69, 0x7a, 0x65, 0x52, 0x07, 0x6d, 0x61, 0x78, 0x53, 0x69, 0x7a, 0x65,
    0x12, 0x24, 0x0a, 0x05, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x0e, 0x2e, 0x72, 0x6f, 0x6f, 0x6d, 0x2e, 0x52, 0x6f, 0x6f, 0x6d, 0x55, 0x73, 0x65, 0x72, 0x52,
    0x05, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x12, 0x2c, 0x0a, 0x07, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72,
    0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x72, 0x6f, 0x6f, 0x6d, 0x2e, 0x52,
    0x6f, 0x6f, 0x6d, 0x55, 0x73, 0x65, 0x72, 0x4c, 0x69, 0x73, 0x74, 0x52, 0x07, 0x6d, 0x65, 0x6d,
    0x62, 0x65, 0x72, 0x73, 0x42, 0x09, 0x0a, 0x07, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2a,
    0x2c, 0x0a, 0x0a, 0x52, 0x6f, 0x6f, 0x6d, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x08, 0x0a,
    0x04, 0x4f, 0x50, 0x45, 0x4e, 0x10, 0x00, 0x12, 0x0a, 0x0a, 0x06, 0x43, 0x4c, 0x4f, 0x53, 0x45,
    0x44, 0x10, 0x01, 0x12, 0x08, 0x0a, 0x04, 0x46, 0x55, 0x4c, 0x4c, 0x10, 0x02, 0x2a, 0x38, 0x0a,
    0x0b, 0x4d, 0x61, 0x78, 0x52, 0x6f, 0x6f, 0x6d, 0x53, 0x69, 0x7a, 0x65, 0x12, 0x12, 0x0a, 0x0e,
    0x44, 0x4f, 0x55, 0x42, 0x4c, 0x45, 0x5f, 0x50, 0x4c, 0x41, 0x59, 0x45, 0x52, 0x53, 0x10, 0x00,
    0x12, 0x15, 0x0a, 0x11, 0x51, 0x55, 0x41, 0x44, 0x52, 0x55, 0x50, 0x4c, 0x45, 0x5f, 0x50, 0x4c,
    0x41, 0x59, 0x45, 0x52, 0x53, 0x10, 0x01, 0x32, 0xf4, 0x01, 0x0a, 0x0b, 0x52, 0x6f, 0x6f, 0x6d,
    0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x12, 0x31, 0x0a, 0x09, 0x4c, 0x69, 0x73, 0x74, 0x52,
    0x6f, 0x6f, 0x6d, 0x73, 0x12, 0x16, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x45, 0x6d, 0x70, 0x74, 0x79, 0x1a, 0x0a, 0x2e, 0x72,
    0x6f, 0x6f, 0x6d, 0x2e, 0x52, 0x6f, 0x6f, 0x6d, 0x30, 0x01, 0x12, 0x3d, 0x0a, 0x0a, 0x43, 0x72,
    0x65, 0x61, 0x74, 0x65, 0x52, 0x6f, 0x6f, 0x6d, 0x12, 0x17, 0x2e, 0x72, 0x6f, 0x6f, 0x6d, 0x2e,
    0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x52, 0x6f, 0x6f, 0x6d, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x1a, 0x16, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x62, 0x75, 0x66, 0x2e, 0x45, 0x6d, 0x70, 0x74, 0x79, 0x12, 0x3d, 0x0a, 0x0a, 0x44, 0x65, 0x6c,
    0x65, 0x74, 0x65, 0x52, 0x6f, 0x6f, 0x6d, 0x12, 0x17, 0x2e, 0x72, 0x6f, 0x6f, 0x6d, 0x2e, 0x44,
    0x65, 0x6c, 0x65, 0x74, 0x65, 0x52, 0x6f, 0x6f, 0x6d, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x1a, 0x16, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62,
    0x75, 0x66, 0x2e, 0x45, 0x6d, 0x70, 0x74, 0x79, 0x12, 0x34, 0x0a, 0x0d, 0x57, 0x61, 0x74, 0x63,
    0x68, 0x52, 0x6f, 0x6f, 0x6d, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x15, 0x2e, 0x72, 0x6f, 0x6f, 0x6d,
    0x2e, 0x52, 0x6f, 0x6f, 0x6d, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x1a, 0x0a, 0x2e, 0x72, 0x6f, 0x6f, 0x6d, 0x2e, 0x52, 0x6f, 0x6f, 0x6d, 0x30, 0x01, 0x32, 0xb2,
    0x01, 0x0a, 0x0f, 0x52, 0x6f, 0x6f, 0x6d, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x61, 0x6e, 0x61, 0x67,
    0x65, 0x72, 0x12, 0x32, 0x0a, 0x08, 0x4a, 0x6f, 0x69, 0x6e, 0x52, 0x6f, 0x6f, 0x6d, 0x12, 0x0e,
    0x2e, 0x72, 0x6f, 0x6f, 0x6d, 0x2e, 0x52, 0x6f, 0x6f, 0x6d, 0x55, 0x73, 0x65, 0x72, 0x1a, 0x16,
    0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66,
    0x2e, 0x45, 0x6d, 0x70, 0x74, 0x79, 0x12, 0x36, 0x0a, 0x08, 0x47, 0x65, 0x74, 0x55, 0x73, 0x65,
    0x72, 0x73, 0x12, 0x16, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x62, 0x75, 0x66, 0x2e, 0x45, 0x6d, 0x70, 0x74, 0x79, 0x1a, 0x12, 0x2e, 0x72, 0x6f, 0x6f,
    0x6d, 0x2e, 0x52, 0x6f, 0x6f, 0x6d, 0x55, 0x73, 0x65, 0x72, 0x4c, 0x69, 0x73, 0x74, 0x12, 0x33,
    0x0a, 0x09, 0x4c, 0x65, 0x61, 0x76, 0x65, 0x52, 0x6f, 0x6f, 0x6d, 0x12, 0x0e, 0x2e, 0x72, 0x6f,
    0x6f, 0x6d, 0x2e, 0x52, 0x6f, 0x6f, 0x6d, 0x55, 0x73, 0x65, 0x72, 0x1a, 0x16, 0x2e, 0x67, 0x6f,
    0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x45, 0x6d,
    0x70, 0x74, 0x79, 0x4a, 0xd9, 0x0e, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x44, 0x01, 0x0a, 0x08,
    0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03,
    0x02, 0x00, 0x25, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x04, 0x00, 0x0d, 0x0a, 0x0a, 0x0a,
    0x02, 0x06, 0x00, 0x12, 0x04, 0x06, 0x00, 0x0b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01,
    0x12, 0x03, 0x06, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x03, 0x07,
    0x04, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x07, 0x12, 0x27, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x07, 0x32, 0x38, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x07, 0x39, 0x3d, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x08, 0x04, 0x46, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x08, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03,
    0x08, 0x13, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x08, 0x2f,
    0x44, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x02, 0x12, 0x03, 0x09, 0x04, 0x46, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x09, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x09, 0x13, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x09, 0x2f, 0x44, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x03,
    0x12, 0x03, 0x0a, 0x04, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x0a, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x0a, 0x16,
    0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x0a, 0x30, 0x36, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0a, 0x37, 0x3b, 0x0a, 0x0a, 0x0a,
    0x02, 0x06, 0x01, 0x12, 0x04, 0x0d, 0x00, 0x11, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x01, 0x01,
    0x12, 0x03, 0x0d, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0e,
    0x04, 0x3b, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0e, 0x11, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0e, 0x24, 0x39, 0x0a, 0x0b, 0x0a, 0x04,
    0x06, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x04, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x01, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x0f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x01, 0x02, 0x01, 0x02,
    0x12, 0x03, 0x0f, 0x11, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x0f, 0x31, 0x3d, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x01, 0x02, 0x02, 0x12, 0x03, 0x10, 0x04, 0x3c,
    0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x10, 0x08, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x01, 0x02, 0x02, 0x02, 0x12, 0x03, 0x10, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x10, 0x25, 0x3a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x13, 0x00, 0x15, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x13,
    0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x14, 0x04, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x14, 0x04, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x14, 0x0b, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x14, 0x10, 0x11, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12,
    0x04, 0x17, 0x00, 0x1b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x17, 0x08,
    0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x18, 0x04, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x18, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18, 0x0b, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x18, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01,
    0x12, 0x03, 0x19, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03,
    0x19, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x19, 0x10,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x19, 0x1b, 0x1c, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x1a, 0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x1a, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x1a, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x1a, 0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x1d, 0x00,
    0x1f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x1d, 0x08, 0x19, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x1e, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1e, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x1e, 0x0b, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x1e, 0x10, 0x11, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x21, 0x00, 0x25,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x21, 0x05, 0x0f, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x22, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x22, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00,
    0x02, 0x12, 0x03, 0x22, 0x0b, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x23, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x23, 0x04,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x23, 0x0d, 0x0e, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x24, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x24, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x02, 0x02, 0x12, 0x03, 0x24, 0x0b, 0x0c, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x01, 0x12, 0x04,
    0x27, 0x00, 0x2a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x01, 0x01, 0x12, 0x03, 0x27, 0x05, 0x10,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00, 0x12, 0x03, 0x28, 0x04, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x28, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x28, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02,
    0x01, 0x12, 0x03, 0x29, 0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x29, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x02, 0x12, 0x03, 0x29,
    0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x2c, 0x00, 0x2e, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x2c, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x00, 0x12, 0x03, 0x2d, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x2d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x2d, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2d, 0x16,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2d, 0x1e, 0x1f, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x30, 0x00, 0x32, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x04, 0x01, 0x12, 0x03, 0x30, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12,
    0x03, 0x31, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x31,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03, 0x31, 0x0d, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x31, 0x16, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x31, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x05, 0x12, 0x04, 0x34, 0x00, 0x36, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12,
    0x03, 0x34, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x35, 0x04,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x35, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12, 0x03, 0x35, 0x0d, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x35, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x35, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12,
    0x04, 0x38, 0x00, 0x3b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x38, 0x08,
    0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x39, 0x04, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x39, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x39, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x39, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01,
    0x12, 0x03, 0x3a, 0x04, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x3a, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3a, 0x0b,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3a, 0x16, 0x17, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x3d, 0x00, 0x44, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x07, 0x01, 0x12, 0x03, 0x3d, 0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12,
    0x03, 0x3e, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3e,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x06, 0x12, 0x03, 0x3e, 0x0d, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3e, 0x18, 0x1e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3e, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x3f, 0x04, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x3f, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x3f, 0x0a, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x3f, 0x0f, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x02, 0x12, 0x03, 0x40, 0x04, 0x14,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x05, 0x12, 0x03, 0x40, 0x04, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12, 0x03, 0x40, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x03, 0x40, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07,
    0x02, 0x03, 0x12, 0x03, 0x41, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x06,
    0x12, 0x03, 0x41, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x41, 0x10, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x03, 0x12, 0x03, 0x41, 0x1b,
    0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x04, 0x12, 0x03, 0x42, 0x04, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x06, 0x12, 0x03, 0x42, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x04, 0x01, 0x12, 0x03, 0x42, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x42, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x05,
    0x12, 0x03, 0x43, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x05, 0x06, 0x12, 0x03,
    0x43, 0x04, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x05, 0x01, 0x12, 0x03, 0x43, 0x11,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x05, 0x03, 0x12, 0x03, 0x43, 0x1b, 0x1c, 0x62,
    0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("room.serde.rs");
include!("room.tonic.rs");
// @@protoc_insertion_point(module)