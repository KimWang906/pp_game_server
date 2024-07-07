use std::{
    rc::Rc,
    sync::{Arc, Mutex, MutexGuard, RwLock},
};

use protos::{datastreams::RoomEventUpdate, models::RoomSubscription};
use tonic::Streaming;

#[derive(Clone)]
pub struct RoomEventSubscription
where
    RoomEventSubscription: Send + Sync,
{
    observer: Arc<Mutex<Streaming<RoomEventUpdate>>>,
    subscription: RoomSubscription,
}

impl RoomEventSubscription {
    pub fn new(observer: Streaming<RoomEventUpdate>, subscription: RoomSubscription) -> Self {
        Self {
            observer: Arc::new(Mutex::new(observer)),
            subscription,
        }
    }

    pub fn get_observer(&self) -> MutexGuard<Streaming<RoomEventUpdate>> {
        self.observer.lock().unwrap()
    }

    pub fn get_subscription(&self) -> &RoomSubscription {
        &self.subscription
    }
}
