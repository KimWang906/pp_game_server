// @generated
impl serde::Serialize for GameEventUpdate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.game_event.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("datastreams.GameEventUpdate", len)?;
        if let Some(v) = self.game_event.as_ref() {
            struct_ser.serialize_field("gameEvent", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GameEventUpdate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "game_event",
            "gameEvent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GameEvent,
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
                            "gameEvent" | "game_event" => Ok(GeneratedField::GameEvent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GameEventUpdate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct datastreams.GameEventUpdate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GameEventUpdate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut game_event__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::GameEvent => {
                            if game_event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gameEvent"));
                            }
                            game_event__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GameEventUpdate {
                    game_event: game_event__,
                })
            }
        }
        deserializer.deserialize_struct("datastreams.GameEventUpdate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RoomEventUpdate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.room_event.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("datastreams.RoomEventUpdate", len)?;
        if let Some(v) = self.room_event.as_ref() {
            struct_ser.serialize_field("roomEvent", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RoomEventUpdate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "room_event",
            "roomEvent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RoomEvent,
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
                            "roomEvent" | "room_event" => Ok(GeneratedField::RoomEvent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RoomEventUpdate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct datastreams.RoomEventUpdate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RoomEventUpdate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut room_event__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RoomEvent => {
                            if room_event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomEvent"));
                            }
                            room_event__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RoomEventUpdate {
                    room_event: room_event__,
                })
            }
        }
        deserializer.deserialize_struct("datastreams.RoomEventUpdate", FIELDS, GeneratedVisitor)
    }
}
