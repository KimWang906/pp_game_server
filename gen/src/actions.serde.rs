// @generated
impl serde::Serialize for AddPlayerRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.player.is_some() {
            len += 1;
        }
        if self.room_id != 0 {
            len += 1;
        }
        if !self.password.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("actions.AddPlayerRequest", len)?;
        if let Some(v) = self.player.as_ref() {
            struct_ser.serialize_field("player", v)?;
        }
        if self.room_id != 0 {
            struct_ser.serialize_field("roomId", &self.room_id)?;
        }
        if !self.password.is_empty() {
            struct_ser.serialize_field("password", &self.password)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddPlayerRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "player",
            "room_id",
            "roomId",
            "password",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Player,
            RoomId,
            Password,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "player" => Ok(GeneratedField::Player),
                            "roomId" | "room_id" => Ok(GeneratedField::RoomId),
                            "password" => Ok(GeneratedField::Password),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddPlayerRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct actions.AddPlayerRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddPlayerRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut player__ = None;
                let mut room_id__ = None;
                let mut password__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Player => {
                            if player__.is_some() {
                                return Err(serde::de::Error::duplicate_field("player"));
                            }
                            player__ = map_.next_value()?;
                        }
                        GeneratedField::RoomId => {
                            if room_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomId"));
                            }
                            room_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Password => {
                            if password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            password__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AddPlayerRequest {
                    player: player__,
                    room_id: room_id__.unwrap_or_default(),
                    password: password__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("actions.AddPlayerRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddPlayerResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.room_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("actions.AddPlayerResponse", len)?;
        if self.room_id != 0 {
            struct_ser.serialize_field("roomId", &self.room_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddPlayerResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "room_id",
            "roomId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RoomId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "roomId" | "room_id" => Ok(GeneratedField::RoomId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddPlayerResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct actions.AddPlayerResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddPlayerResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut room_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RoomId => {
                            if room_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomId"));
                            }
                            room_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(AddPlayerResponse {
                    room_id: room_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("actions.AddPlayerResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BroadcastBallRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.room_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("actions.BroadcastBallRequest", len)?;
        if self.room_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("roomId", ToString::to_string(&self.room_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BroadcastBallRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "room_id",
            "roomId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RoomId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "roomId" | "room_id" => Ok(GeneratedField::RoomId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BroadcastBallRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct actions.BroadcastBallRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BroadcastBallRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut room_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RoomId => {
                            if room_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomId"));
                            }
                            room_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(BroadcastBallRequest {
                    room_id: room_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("actions.BroadcastBallRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BroadcastBallResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.x != 0 {
            len += 1;
        }
        if self.y != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("actions.BroadcastBallResponse", len)?;
        if self.x != 0 {
            struct_ser.serialize_field("x", &self.x)?;
        }
        if self.y != 0 {
            struct_ser.serialize_field("y", &self.y)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BroadcastBallResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "x",
            "y",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            X,
            Y,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "x" => Ok(GeneratedField::X),
                            "y" => Ok(GeneratedField::Y),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BroadcastBallResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct actions.BroadcastBallResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BroadcastBallResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut x__ = None;
                let mut y__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::X => {
                            if x__.is_some() {
                                return Err(serde::de::Error::duplicate_field("x"));
                            }
                            x__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Y => {
                            if y__.is_some() {
                                return Err(serde::de::Error::duplicate_field("y"));
                            }
                            y__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(BroadcastBallResponse {
                    x: x__.unwrap_or_default(),
                    y: y__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("actions.BroadcastBallResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BroadcastWinnerRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.player.is_some() {
            len += 1;
        }
        if self.room_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("actions.BroadcastWinnerRequest", len)?;
        if let Some(v) = self.player.as_ref() {
            struct_ser.serialize_field("player", v)?;
        }
        if self.room_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("roomId", ToString::to_string(&self.room_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BroadcastWinnerRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "player",
            "room_id",
            "roomId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Player,
            RoomId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "player" => Ok(GeneratedField::Player),
                            "roomId" | "room_id" => Ok(GeneratedField::RoomId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BroadcastWinnerRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct actions.BroadcastWinnerRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BroadcastWinnerRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut player__ = None;
                let mut room_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Player => {
                            if player__.is_some() {
                                return Err(serde::de::Error::duplicate_field("player"));
                            }
                            player__ = map_.next_value()?;
                        }
                        GeneratedField::RoomId => {
                            if room_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomId"));
                            }
                            room_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(BroadcastWinnerRequest {
                    player: player__,
                    room_id: room_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("actions.BroadcastWinnerRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BroadcastWinnerResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.success.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("actions.BroadcastWinnerResponse", len)?;
        if let Some(v) = self.success.as_ref() {
            struct_ser.serialize_field("success", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BroadcastWinnerResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "success",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Success,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "success" => Ok(GeneratedField::Success),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BroadcastWinnerResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct actions.BroadcastWinnerResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BroadcastWinnerResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut success__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Success => {
                            if success__.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            success__ = map_.next_value()?;
                        }
                    }
                }
                Ok(BroadcastWinnerResponse {
                    success: success__,
                })
            }
        }
        deserializer.deserialize_struct("actions.BroadcastWinnerResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRoomsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("actions.GetRoomsRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRoomsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRoomsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct actions.GetRoomsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetRoomsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GetRoomsRequest {
                })
            }
        }
        deserializer.deserialize_struct("actions.GetRoomsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRoomsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rooms.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("actions.GetRoomsResponse", len)?;
        if !self.rooms.is_empty() {
            struct_ser.serialize_field("rooms", &self.rooms)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRoomsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rooms",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rooms,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "rooms" => Ok(GeneratedField::Rooms),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRoomsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct actions.GetRoomsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetRoomsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rooms__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rooms => {
                            if rooms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rooms"));
                            }
                            rooms__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetRoomsResponse {
                    rooms: rooms__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("actions.GetRoomsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HostRoomRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.room_name.is_empty() {
            len += 1;
        }
        if self.max_size != 0 {
            len += 1;
        }
        if !self.password.is_empty() {
            len += 1;
        }
        if self.player_id != 0 {
            len += 1;
        }
        if !self.player_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("actions.HostRoomRequest", len)?;
        if !self.room_name.is_empty() {
            struct_ser.serialize_field("roomName", &self.room_name)?;
        }
        if self.max_size != 0 {
            let v = super::models::MaxSize::try_from(self.max_size)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.max_size)))?;
            struct_ser.serialize_field("maxSize", &v)?;
        }
        if !self.password.is_empty() {
            struct_ser.serialize_field("password", &self.password)?;
        }
        if self.player_id != 0 {
            struct_ser.serialize_field("playerId", &self.player_id)?;
        }
        if !self.player_name.is_empty() {
            struct_ser.serialize_field("playerName", &self.player_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostRoomRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "room_name",
            "roomName",
            "max_size",
            "maxSize",
            "password",
            "player_id",
            "playerId",
            "player_name",
            "playerName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RoomName,
            MaxSize,
            Password,
            PlayerId,
            PlayerName,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "roomName" | "room_name" => Ok(GeneratedField::RoomName),
                            "maxSize" | "max_size" => Ok(GeneratedField::MaxSize),
                            "password" => Ok(GeneratedField::Password),
                            "playerId" | "player_id" => Ok(GeneratedField::PlayerId),
                            "playerName" | "player_name" => Ok(GeneratedField::PlayerName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostRoomRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct actions.HostRoomRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HostRoomRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut room_name__ = None;
                let mut max_size__ = None;
                let mut password__ = None;
                let mut player_id__ = None;
                let mut player_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RoomName => {
                            if room_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomName"));
                            }
                            room_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxSize => {
                            if max_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxSize"));
                            }
                            max_size__ = Some(map_.next_value::<super::models::MaxSize>()? as i32);
                        }
                        GeneratedField::Password => {
                            if password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            password__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PlayerId => {
                            if player_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("playerId"));
                            }
                            player_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PlayerName => {
                            if player_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("playerName"));
                            }
                            player_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(HostRoomRequest {
                    room_name: room_name__.unwrap_or_default(),
                    max_size: max_size__.unwrap_or_default(),
                    password: password__.unwrap_or_default(),
                    player_id: player_id__.unwrap_or_default(),
                    player_name: player_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("actions.HostRoomRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HostRoomResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.room_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("actions.HostRoomResponse", len)?;
        if self.room_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("roomId", ToString::to_string(&self.room_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostRoomResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "room_id",
            "roomId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RoomId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "roomId" | "room_id" => Ok(GeneratedField::RoomId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostRoomResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct actions.HostRoomResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HostRoomResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut room_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RoomId => {
                            if room_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomId"));
                            }
                            room_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(HostRoomResponse {
                    room_id: room_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("actions.HostRoomResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuitPlayerRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.player.is_some() {
            len += 1;
        }
        if self.room_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("actions.QuitPlayerRequest", len)?;
        if let Some(v) = self.player.as_ref() {
            struct_ser.serialize_field("player", v)?;
        }
        if self.room_id != 0 {
            struct_ser.serialize_field("roomId", &self.room_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuitPlayerRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "player",
            "room_id",
            "roomId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Player,
            RoomId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "player" => Ok(GeneratedField::Player),
                            "roomId" | "room_id" => Ok(GeneratedField::RoomId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuitPlayerRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct actions.QuitPlayerRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuitPlayerRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut player__ = None;
                let mut room_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Player => {
                            if player__.is_some() {
                                return Err(serde::de::Error::duplicate_field("player"));
                            }
                            player__ = map_.next_value()?;
                        }
                        GeneratedField::RoomId => {
                            if room_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomId"));
                            }
                            room_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(QuitPlayerRequest {
                    player: player__,
                    room_id: room_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("actions.QuitPlayerRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuitPlayerResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.success.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("actions.QuitPlayerResponse", len)?;
        if let Some(v) = self.success.as_ref() {
            struct_ser.serialize_field("success", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuitPlayerResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "success",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Success,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "success" => Ok(GeneratedField::Success),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuitPlayerResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct actions.QuitPlayerResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuitPlayerResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut success__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Success => {
                            if success__.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            success__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QuitPlayerResponse {
                    success: success__,
                })
            }
        }
        deserializer.deserialize_struct("actions.QuitPlayerResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReconnectRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("actions.ReconnectRequest", len)?;
        if let Some(v) = self.token.as_ref() {
            struct_ser.serialize_field("token", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReconnectRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "token",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Token,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "token" => Ok(GeneratedField::Token),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReconnectRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct actions.ReconnectRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReconnectRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Token => {
                            if token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("token"));
                            }
                            token__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ReconnectRequest {
                    token: token__,
                })
            }
        }
        deserializer.deserialize_struct("actions.ReconnectRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReconnectResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.room_id != 0 {
            len += 1;
        }
        if self.player_id != 0 {
            len += 1;
        }
        if !self.room_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("actions.ReconnectResponse", len)?;
        if self.room_id != 0 {
            struct_ser.serialize_field("roomId", &self.room_id)?;
        }
        if self.player_id != 0 {
            struct_ser.serialize_field("playerId", &self.player_id)?;
        }
        if !self.room_name.is_empty() {
            struct_ser.serialize_field("roomName", &self.room_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReconnectResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "room_id",
            "roomId",
            "player_id",
            "playerId",
            "room_name",
            "roomName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RoomId,
            PlayerId,
            RoomName,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "roomId" | "room_id" => Ok(GeneratedField::RoomId),
                            "playerId" | "player_id" => Ok(GeneratedField::PlayerId),
                            "roomName" | "room_name" => Ok(GeneratedField::RoomName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReconnectResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct actions.ReconnectResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReconnectResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut room_id__ = None;
                let mut player_id__ = None;
                let mut room_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RoomId => {
                            if room_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomId"));
                            }
                            room_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PlayerId => {
                            if player_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("playerId"));
                            }
                            player_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RoomName => {
                            if room_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomName"));
                            }
                            room_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReconnectResponse {
                    room_id: room_id__.unwrap_or_default(),
                    player_id: player_id__.unwrap_or_default(),
                    room_name: room_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("actions.ReconnectResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemovePlayerRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.player.is_some() {
            len += 1;
        }
        if self.room_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("actions.RemovePlayerRequest", len)?;
        if let Some(v) = self.player.as_ref() {
            struct_ser.serialize_field("player", v)?;
        }
        if self.room_id != 0 {
            struct_ser.serialize_field("roomId", &self.room_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemovePlayerRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "player",
            "room_id",
            "roomId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Player,
            RoomId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "player" => Ok(GeneratedField::Player),
                            "roomId" | "room_id" => Ok(GeneratedField::RoomId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemovePlayerRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct actions.RemovePlayerRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemovePlayerRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut player__ = None;
                let mut room_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Player => {
                            if player__.is_some() {
                                return Err(serde::de::Error::duplicate_field("player"));
                            }
                            player__ = map_.next_value()?;
                        }
                        GeneratedField::RoomId => {
                            if room_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomId"));
                            }
                            room_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(RemovePlayerRequest {
                    player: player__,
                    room_id: room_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("actions.RemovePlayerRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemovePlayerResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.success.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("actions.RemovePlayerResponse", len)?;
        if let Some(v) = self.success.as_ref() {
            struct_ser.serialize_field("success", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemovePlayerResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "success",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Success,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "success" => Ok(GeneratedField::Success),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemovePlayerResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct actions.RemovePlayerResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemovePlayerResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut success__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Success => {
                            if success__.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            success__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RemovePlayerResponse {
                    success: success__,
                })
            }
        }
        deserializer.deserialize_struct("actions.RemovePlayerResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetPlayerReadyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.player_id != 0 {
            len += 1;
        }
        if self.is_ready {
            len += 1;
        }
        if self.room_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("actions.SetPlayerReadyRequest", len)?;
        if self.player_id != 0 {
            struct_ser.serialize_field("playerId", &self.player_id)?;
        }
        if self.is_ready {
            struct_ser.serialize_field("isReady", &self.is_ready)?;
        }
        if self.room_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("roomId", ToString::to_string(&self.room_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetPlayerReadyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "player_id",
            "playerId",
            "is_ready",
            "isReady",
            "room_id",
            "roomId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PlayerId,
            IsReady,
            RoomId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "playerId" | "player_id" => Ok(GeneratedField::PlayerId),
                            "isReady" | "is_ready" => Ok(GeneratedField::IsReady),
                            "roomId" | "room_id" => Ok(GeneratedField::RoomId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetPlayerReadyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct actions.SetPlayerReadyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetPlayerReadyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut player_id__ = None;
                let mut is_ready__ = None;
                let mut room_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PlayerId => {
                            if player_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("playerId"));
                            }
                            player_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::IsReady => {
                            if is_ready__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isReady"));
                            }
                            is_ready__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RoomId => {
                            if room_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomId"));
                            }
                            room_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(SetPlayerReadyRequest {
                    player_id: player_id__.unwrap_or_default(),
                    is_ready: is_ready__.unwrap_or_default(),
                    room_id: room_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("actions.SetPlayerReadyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetPlayerReadyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.player_id != 0 {
            len += 1;
        }
        if self.is_ready {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("actions.SetPlayerReadyResponse", len)?;
        if self.player_id != 0 {
            struct_ser.serialize_field("playerId", &self.player_id)?;
        }
        if self.is_ready {
            struct_ser.serialize_field("isReady", &self.is_ready)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetPlayerReadyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "player_id",
            "playerId",
            "is_ready",
            "isReady",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PlayerId,
            IsReady,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "playerId" | "player_id" => Ok(GeneratedField::PlayerId),
                            "isReady" | "is_ready" => Ok(GeneratedField::IsReady),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetPlayerReadyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct actions.SetPlayerReadyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetPlayerReadyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut player_id__ = None;
                let mut is_ready__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PlayerId => {
                            if player_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("playerId"));
                            }
                            player_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::IsReady => {
                            if is_ready__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isReady"));
                            }
                            is_ready__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SetPlayerReadyResponse {
                    player_id: player_id__.unwrap_or_default(),
                    is_ready: is_ready__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("actions.SetPlayerReadyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StartNextRoundRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.player_id != 0 {
            len += 1;
        }
        if self.room_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("actions.StartNextRoundRequest", len)?;
        if self.player_id != 0 {
            struct_ser.serialize_field("playerId", &self.player_id)?;
        }
        if self.room_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("roomId", ToString::to_string(&self.room_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StartNextRoundRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "player_id",
            "playerId",
            "room_id",
            "roomId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PlayerId,
            RoomId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "playerId" | "player_id" => Ok(GeneratedField::PlayerId),
                            "roomId" | "room_id" => Ok(GeneratedField::RoomId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StartNextRoundRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct actions.StartNextRoundRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StartNextRoundRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut player_id__ = None;
                let mut room_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PlayerId => {
                            if player_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("playerId"));
                            }
                            player_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RoomId => {
                            if room_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomId"));
                            }
                            room_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(StartNextRoundRequest {
                    player_id: player_id__.unwrap_or_default(),
                    room_id: room_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("actions.StartNextRoundRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StartNextRoundResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.success.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("actions.StartNextRoundResponse", len)?;
        if let Some(v) = self.success.as_ref() {
            struct_ser.serialize_field("success", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StartNextRoundResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "success",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Success,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "success" => Ok(GeneratedField::Success),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StartNextRoundResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct actions.StartNextRoundResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StartNextRoundResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut success__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Success => {
                            if success__.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            success__ = map_.next_value()?;
                        }
                    }
                }
                Ok(StartNextRoundResponse {
                    success: success__,
                })
            }
        }
        deserializer.deserialize_struct("actions.StartNextRoundResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnsubscribeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.player_student_id.is_empty() {
            len += 1;
        }
        if self.room_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("actions.UnsubscribeRequest", len)?;
        if !self.player_student_id.is_empty() {
            struct_ser.serialize_field("playerStudentId", &self.player_student_id)?;
        }
        if self.room_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("roomId", ToString::to_string(&self.room_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnsubscribeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "player_student_id",
            "playerStudentId",
            "room_id",
            "roomId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PlayerStudentId,
            RoomId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "playerStudentId" | "player_student_id" => Ok(GeneratedField::PlayerStudentId),
                            "roomId" | "room_id" => Ok(GeneratedField::RoomId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnsubscribeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct actions.UnsubscribeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnsubscribeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut player_student_id__ = None;
                let mut room_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PlayerStudentId => {
                            if player_student_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("playerStudentId"));
                            }
                            player_student_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RoomId => {
                            if room_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomId"));
                            }
                            room_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(UnsubscribeRequest {
                    player_student_id: player_student_id__.unwrap_or_default(),
                    room_id: room_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("actions.UnsubscribeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnsubscribeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.success.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("actions.UnsubscribeResponse", len)?;
        if let Some(v) = self.success.as_ref() {
            struct_ser.serialize_field("success", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnsubscribeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "success",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Success,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "success" => Ok(GeneratedField::Success),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnsubscribeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct actions.UnsubscribeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnsubscribeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut success__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Success => {
                            if success__.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            success__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UnsubscribeResponse {
                    success: success__,
                })
            }
        }
        deserializer.deserialize_struct("actions.UnsubscribeResponse", FIELDS, GeneratedVisitor)
    }
}
