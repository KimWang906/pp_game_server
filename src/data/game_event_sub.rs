use std::sync::{Arc, Mutex, MutexGuard};

use protos::{datastreams::GameEventUpdate, models::GameSubscription};
use tonic::Streaming;

#[derive(Clone)]
pub struct GameEventSubscription {
    player_id: u32,
    observer: Arc<Mutex<Streaming<GameEventUpdate>>>,
    game_sub: GameSubscription,
}

impl GameEventSubscription {
    pub fn new(
        player_id: u32,
        observer: Streaming<GameEventUpdate>,
        game_sub: GameSubscription,
    ) -> Self {
        Self {
            player_id,
            observer: Arc::new(Mutex::new(observer)),
            game_sub,
        }
    }

    pub fn get_observer(&self) -> MutexGuard<Streaming<GameEventUpdate>> {
        self.observer.lock().unwrap()
    }

    pub fn get_game_sub(&self) -> &GameSubscription {
        &self.game_sub
    }

    pub fn get_player_id(&self) -> u32 {
        self.player_id
    }
}
