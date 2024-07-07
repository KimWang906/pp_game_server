// @generated
impl serde::Serialize for Color {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Red => "RED",
            Self::Green => "GREEN",
            Self::Blue => "BLUE",
            Self::Yellow => "YELLOW",
            Self::Orange => "ORANGE",
            Self::Pink => "PINK",
            Self::Cyan => "CYAN",
            Self::White => "WHITE",
            Self::Black => "BLACK",
            Self::Gray => "GRAY",
            Self::LightGray => "LIGHT_GRAY",
            Self::DarkGray => "DARK_GRAY",
            Self::Magenta => "MAGENTA",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Color {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "RED",
            "GREEN",
            "BLUE",
            "YELLOW",
            "ORANGE",
            "PINK",
            "CYAN",
            "WHITE",
            "BLACK",
            "GRAY",
            "LIGHT_GRAY",
            "DARK_GRAY",
            "MAGENTA",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Color;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "RED" => Ok(Color::Red),
                    "GREEN" => Ok(Color::Green),
                    "BLUE" => Ok(Color::Blue),
                    "YELLOW" => Ok(Color::Yellow),
                    "ORANGE" => Ok(Color::Orange),
                    "PINK" => Ok(Color::Pink),
                    "CYAN" => Ok(Color::Cyan),
                    "WHITE" => Ok(Color::White),
                    "BLACK" => Ok(Color::Black),
                    "GRAY" => Ok(Color::Gray),
                    "LIGHT_GRAY" => Ok(Color::LightGray),
                    "DARK_GRAY" => Ok(Color::DarkGray),
                    "MAGENTA" => Ok(Color::Magenta),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for GameEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.event_code != 0 {
            len += 1;
        }
        if self.winner != 0 {
            len += 1;
        }
        if self.current_player_id != 0 {
            len += 1;
        }
        if self.paddle.is_some() {
            len += 1;
        }
        if !self.leaderboard.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("models.GameEvent", len)?;
        if self.event_code != 0 {
            let v = game_event::EventCode::try_from(self.event_code)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.event_code)))?;
            struct_ser.serialize_field("eventCode", &v)?;
        }
        if self.winner != 0 {
            struct_ser.serialize_field("winner", &self.winner)?;
        }
        if self.current_player_id != 0 {
            struct_ser.serialize_field("currentPlayerId", &self.current_player_id)?;
        }
        if let Some(v) = self.paddle.as_ref() {
            struct_ser.serialize_field("paddle", v)?;
        }
        if !self.leaderboard.is_empty() {
            struct_ser.serialize_field("leaderboard", &self.leaderboard)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GameEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "event_code",
            "eventCode",
            "winner",
            "current_player_id",
            "currentPlayerId",
            "paddle",
            "leaderboard",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EventCode,
            Winner,
            CurrentPlayerId,
            Paddle,
            Leaderboard,
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
                            "eventCode" | "event_code" => Ok(GeneratedField::EventCode),
                            "winner" => Ok(GeneratedField::Winner),
                            "currentPlayerId" | "current_player_id" => Ok(GeneratedField::CurrentPlayerId),
                            "paddle" => Ok(GeneratedField::Paddle),
                            "leaderboard" => Ok(GeneratedField::Leaderboard),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GameEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct models.GameEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GameEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut event_code__ = None;
                let mut winner__ = None;
                let mut current_player_id__ = None;
                let mut paddle__ = None;
                let mut leaderboard__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EventCode => {
                            if event_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventCode"));
                            }
                            event_code__ = Some(map_.next_value::<game_event::EventCode>()? as i32);
                        }
                        GeneratedField::Winner => {
                            if winner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("winner"));
                            }
                            winner__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CurrentPlayerId => {
                            if current_player_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentPlayerId"));
                            }
                            current_player_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Paddle => {
                            if paddle__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paddle"));
                            }
                            paddle__ = map_.next_value()?;
                        }
                        GeneratedField::Leaderboard => {
                            if leaderboard__.is_some() {
                                return Err(serde::de::Error::duplicate_field("leaderboard"));
                            }
                            leaderboard__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GameEvent {
                    event_code: event_code__.unwrap_or_default(),
                    winner: winner__.unwrap_or_default(),
                    current_player_id: current_player_id__.unwrap_or_default(),
                    paddle: paddle__,
                    leaderboard: leaderboard__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("models.GameEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for game_event::EventCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::GameStarted => "GAME_STARTED",
            Self::GameWon => "GAME_WON",
            Self::PaddleMoved => "PADDLE_MOVED",
            Self::PlayerQuit => "PLAYER_QUIT",
            Self::NextRound => "NEXT_ROUND",
            Self::InternalServerError => "INTERNAL_SERVER_ERROR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for game_event::EventCode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "GAME_STARTED",
            "GAME_WON",
            "PADDLE_MOVED",
            "PLAYER_QUIT",
            "NEXT_ROUND",
            "INTERNAL_SERVER_ERROR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = game_event::EventCode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "GAME_STARTED" => Ok(game_event::EventCode::GameStarted),
                    "GAME_WON" => Ok(game_event::EventCode::GameWon),
                    "PADDLE_MOVED" => Ok(game_event::EventCode::PaddleMoved),
                    "PLAYER_QUIT" => Ok(game_event::EventCode::PlayerQuit),
                    "NEXT_ROUND" => Ok(game_event::EventCode::NextRound),
                    "INTERNAL_SERVER_ERROR" => Ok(game_event::EventCode::InternalServerError),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for GameSubscription {
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
        if self.paddle_direction != 0 {
            len += 1;
        }
        if self.winner_id != 0 {
            len += 1;
        }
        if self.first_subscription {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("models.GameSubscription", len)?;
        if self.room_id != 0 {
            struct_ser.serialize_field("roomId", &self.room_id)?;
        }
        if self.player_id != 0 {
            struct_ser.serialize_field("playerId", &self.player_id)?;
        }
        if self.paddle_direction != 0 {
            let v = PaddleDirection::try_from(self.paddle_direction)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.paddle_direction)))?;
            struct_ser.serialize_field("paddleDirection", &v)?;
        }
        if self.winner_id != 0 {
            struct_ser.serialize_field("winnerId", &self.winner_id)?;
        }
        if self.first_subscription {
            struct_ser.serialize_field("firstSubscription", &self.first_subscription)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GameSubscription {
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
            "paddle_direction",
            "paddleDirection",
            "winner_id",
            "winnerId",
            "first_subscription",
            "firstSubscription",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RoomId,
            PlayerId,
            PaddleDirection,
            WinnerId,
            FirstSubscription,
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
                            "paddleDirection" | "paddle_direction" => Ok(GeneratedField::PaddleDirection),
                            "winnerId" | "winner_id" => Ok(GeneratedField::WinnerId),
                            "firstSubscription" | "first_subscription" => Ok(GeneratedField::FirstSubscription),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GameSubscription;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct models.GameSubscription")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GameSubscription, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut room_id__ = None;
                let mut player_id__ = None;
                let mut paddle_direction__ = None;
                let mut winner_id__ = None;
                let mut first_subscription__ = None;
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
                        GeneratedField::PaddleDirection => {
                            if paddle_direction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paddleDirection"));
                            }
                            paddle_direction__ = Some(map_.next_value::<PaddleDirection>()? as i32);
                        }
                        GeneratedField::WinnerId => {
                            if winner_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("winnerId"));
                            }
                            winner_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FirstSubscription => {
                            if first_subscription__.is_some() {
                                return Err(serde::de::Error::duplicate_field("firstSubscription"));
                            }
                            first_subscription__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GameSubscription {
                    room_id: room_id__.unwrap_or_default(),
                    player_id: player_id__.unwrap_or_default(),
                    paddle_direction: paddle_direction__.unwrap_or_default(),
                    winner_id: winner_id__.unwrap_or_default(),
                    first_subscription: first_subscription__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("models.GameSubscription", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HostSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_score != 0 {
            len += 1;
        }
        if self.ball_spped != 0 {
            len += 1;
        }
        if self.paddle_speed != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("models.HostSettings", len)?;
        if self.max_score != 0 {
            struct_ser.serialize_field("maxScore", &self.max_score)?;
        }
        if self.ball_spped != 0 {
            struct_ser.serialize_field("ballSpped", &self.ball_spped)?;
        }
        if self.paddle_speed != 0 {
            struct_ser.serialize_field("paddleSpeed", &self.paddle_speed)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_score",
            "maxScore",
            "ball_spped",
            "ballSpped",
            "paddle_speed",
            "paddleSpeed",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxScore,
            BallSpped,
            PaddleSpeed,
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
                            "maxScore" | "max_score" => Ok(GeneratedField::MaxScore),
                            "ballSpped" | "ball_spped" => Ok(GeneratedField::BallSpped),
                            "paddleSpeed" | "paddle_speed" => Ok(GeneratedField::PaddleSpeed),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct models.HostSettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HostSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_score__ = None;
                let mut ball_spped__ = None;
                let mut paddle_speed__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MaxScore => {
                            if max_score__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxScore"));
                            }
                            max_score__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BallSpped => {
                            if ball_spped__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ballSpped"));
                            }
                            ball_spped__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PaddleSpeed => {
                            if paddle_speed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paddleSpeed"));
                            }
                            paddle_speed__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(HostSettings {
                    max_score: max_score__.unwrap_or_default(),
                    ball_spped: ball_spped__.unwrap_or_default(),
                    paddle_speed: paddle_speed__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("models.HostSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MaxSize {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::DoublePlayers => "DOUBLE_PLAYERS",
            Self::QuadruplePlayers => "QUADRUPLE_PLAYERS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for MaxSize {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DOUBLE_PLAYERS",
            "QUADRUPLE_PLAYERS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MaxSize;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "DOUBLE_PLAYERS" => Ok(MaxSize::DoublePlayers),
                    "QUADRUPLE_PLAYERS" => Ok(MaxSize::QuadruplePlayers),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Paddle {
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
        if self.position.is_some() {
            len += 1;
        }
        if self.width != 0 {
            len += 1;
        }
        if self.height != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("models.Paddle", len)?;
        if self.player_id != 0 {
            struct_ser.serialize_field("playerId", &self.player_id)?;
        }
        if let Some(v) = self.position.as_ref() {
            struct_ser.serialize_field("position", v)?;
        }
        if self.width != 0 {
            struct_ser.serialize_field("width", &self.width)?;
        }
        if self.height != 0 {
            struct_ser.serialize_field("height", &self.height)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Paddle {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "player_id",
            "playerId",
            "position",
            "width",
            "height",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PlayerId,
            Position,
            Width,
            Height,
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
                            "position" => Ok(GeneratedField::Position),
                            "width" => Ok(GeneratedField::Width),
                            "height" => Ok(GeneratedField::Height),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Paddle;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct models.Paddle")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Paddle, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut player_id__ = None;
                let mut position__ = None;
                let mut width__ = None;
                let mut height__ = None;
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
                        GeneratedField::Position => {
                            if position__.is_some() {
                                return Err(serde::de::Error::duplicate_field("position"));
                            }
                            position__ = map_.next_value()?;
                        }
                        GeneratedField::Width => {
                            if width__.is_some() {
                                return Err(serde::de::Error::duplicate_field("width"));
                            }
                            width__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Paddle {
                    player_id: player_id__.unwrap_or_default(),
                    position: position__,
                    width: width__.unwrap_or_default(),
                    height: height__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("models.Paddle", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for paddle::Position {
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
        let mut struct_ser = serializer.serialize_struct("models.Paddle.Position", len)?;
        if self.x != 0 {
            struct_ser.serialize_field("x", &self.x)?;
        }
        if self.y != 0 {
            struct_ser.serialize_field("y", &self.y)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for paddle::Position {
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
            type Value = paddle::Position;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct models.Paddle.Position")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<paddle::Position, V::Error>
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
                Ok(paddle::Position {
                    x: x__.unwrap_or_default(),
                    y: y__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("models.Paddle.Position", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PaddleDirection {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Up => "UP",
            Self::Down => "DOWN",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PaddleDirection {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UP",
            "DOWN",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PaddleDirection;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UP" => Ok(PaddleDirection::Up),
                    "DOWN" => Ok(PaddleDirection::Down),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Player {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.ready {
            len += 1;
        }
        if self.score != 0 {
            len += 1;
        }
        if self.color != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("models.Player", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.ready {
            struct_ser.serialize_field("ready", &self.ready)?;
        }
        if self.score != 0 {
            struct_ser.serialize_field("score", &self.score)?;
        }
        if self.color != 0 {
            let v = Color::try_from(self.color)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.color)))?;
            struct_ser.serialize_field("color", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Player {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "ready",
            "score",
            "color",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Ready,
            Score,
            Color,
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
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "ready" => Ok(GeneratedField::Ready),
                            "score" => Ok(GeneratedField::Score),
                            "color" => Ok(GeneratedField::Color),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Player;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct models.Player")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Player, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut ready__ = None;
                let mut score__ = None;
                let mut color__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Ready => {
                            if ready__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ready"));
                            }
                            ready__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Score => {
                            if score__.is_some() {
                                return Err(serde::de::Error::duplicate_field("score"));
                            }
                            score__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Color => {
                            if color__.is_some() {
                                return Err(serde::de::Error::duplicate_field("color"));
                            }
                            color__ = Some(map_.next_value::<Color>()? as i32);
                        }
                    }
                }
                Ok(Player {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    ready: ready__.unwrap_or_default(),
                    score: score__.unwrap_or_default(),
                    color: color__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("models.Player", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Room {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.password_exists {
            len += 1;
        }
        if self.settings.is_some() {
            len += 1;
        }
        if self.max_size != 0 {
            len += 1;
        }
        if self.count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("models.Room", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.password_exists {
            struct_ser.serialize_field("passwordExists", &self.password_exists)?;
        }
        if let Some(v) = self.settings.as_ref() {
            struct_ser.serialize_field("settings", v)?;
        }
        if self.max_size != 0 {
            let v = MaxSize::try_from(self.max_size)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.max_size)))?;
            struct_ser.serialize_field("maxSize", &v)?;
        }
        if self.count != 0 {
            struct_ser.serialize_field("count", &self.count)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Room {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "password_exists",
            "passwordExists",
            "settings",
            "max_size",
            "maxSize",
            "count",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            PasswordExists,
            Settings,
            MaxSize,
            Count,
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
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "passwordExists" | "password_exists" => Ok(GeneratedField::PasswordExists),
                            "settings" => Ok(GeneratedField::Settings),
                            "maxSize" | "max_size" => Ok(GeneratedField::MaxSize),
                            "count" => Ok(GeneratedField::Count),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Room;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct models.Room")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Room, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut password_exists__ = None;
                let mut settings__ = None;
                let mut max_size__ = None;
                let mut count__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PasswordExists => {
                            if password_exists__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passwordExists"));
                            }
                            password_exists__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Settings => {
                            if settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            settings__ = map_.next_value()?;
                        }
                        GeneratedField::MaxSize => {
                            if max_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxSize"));
                            }
                            max_size__ = Some(map_.next_value::<MaxSize>()? as i32);
                        }
                        GeneratedField::Count => {
                            if count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Room {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    password_exists: password_exists__.unwrap_or_default(),
                    settings: settings__,
                    max_size: max_size__.unwrap_or_default(),
                    count: count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("models.Room", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RoomEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.event_code != 0 {
            len += 1;
        }
        if !self.players.is_empty() {
            len += 1;
        }
        if self.max_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("models.RoomEvent", len)?;
        if self.event_code != 0 {
            let v = room_event::EventCode::try_from(self.event_code)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.event_code)))?;
            struct_ser.serialize_field("eventCode", &v)?;
        }
        if !self.players.is_empty() {
            struct_ser.serialize_field("players", &self.players)?;
        }
        if self.max_count != 0 {
            struct_ser.serialize_field("maxCount", &self.max_count)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RoomEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "event_code",
            "eventCode",
            "players",
            "max_count",
            "maxCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EventCode,
            Players,
            MaxCount,
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
                            "eventCode" | "event_code" => Ok(GeneratedField::EventCode),
                            "players" => Ok(GeneratedField::Players),
                            "maxCount" | "max_count" => Ok(GeneratedField::MaxCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RoomEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct models.RoomEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RoomEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut event_code__ = None;
                let mut players__ = None;
                let mut max_count__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EventCode => {
                            if event_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventCode"));
                            }
                            event_code__ = Some(map_.next_value::<room_event::EventCode>()? as i32);
                        }
                        GeneratedField::Players => {
                            if players__.is_some() {
                                return Err(serde::de::Error::duplicate_field("players"));
                            }
                            players__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxCount => {
                            if max_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxCount"));
                            }
                            max_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(RoomEvent {
                    event_code: event_code__.unwrap_or_default(),
                    players: players__.unwrap_or_default(),
                    max_count: max_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("models.RoomEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for room_event::EventCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::RoomDestroy => "ROOM_DESTROY",
            Self::AddPlayer => "ADD_PLAYER",
            Self::RemovePlayer => "REMOVE_PLAYER",
            Self::PlayerReadyChanged => "PLAYER_READY_CHANGED",
            Self::PlayerStateChanged => "PLAYER_STATE_CHANGED",
            Self::GameStart => "GAME_START",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for room_event::EventCode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ROOM_DESTROY",
            "ADD_PLAYER",
            "REMOVE_PLAYER",
            "PLAYER_READY_CHANGED",
            "PLAYER_STATE_CHANGED",
            "GAME_START",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = room_event::EventCode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ROOM_DESTROY" => Ok(room_event::EventCode::RoomDestroy),
                    "ADD_PLAYER" => Ok(room_event::EventCode::AddPlayer),
                    "REMOVE_PLAYER" => Ok(room_event::EventCode::RemovePlayer),
                    "PLAYER_READY_CHANGED" => Ok(room_event::EventCode::PlayerReadyChanged),
                    "PLAYER_STATE_CHANGED" => Ok(room_event::EventCode::PlayerStateChanged),
                    "GAME_START" => Ok(room_event::EventCode::GameStart),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for RoomSubscription {
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
        if self.destroy {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("models.RoomSubscription", len)?;
        if self.room_id != 0 {
            struct_ser.serialize_field("roomId", &self.room_id)?;
        }
        if self.player_id != 0 {
            struct_ser.serialize_field("playerId", &self.player_id)?;
        }
        if self.destroy {
            struct_ser.serialize_field("destroy", &self.destroy)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RoomSubscription {
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
            "destroy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RoomId,
            PlayerId,
            Destroy,
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
                            "destroy" => Ok(GeneratedField::Destroy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RoomSubscription;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct models.RoomSubscription")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RoomSubscription, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut room_id__ = None;
                let mut player_id__ = None;
                let mut destroy__ = None;
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
                        GeneratedField::Destroy => {
                            if destroy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destroy"));
                            }
                            destroy__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RoomSubscription {
                    room_id: room_id__.unwrap_or_default(),
                    player_id: player_id__.unwrap_or_default(),
                    destroy: destroy__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("models.RoomSubscription", FIELDS, GeneratedVisitor)
    }
}
