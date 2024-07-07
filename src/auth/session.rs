use crate::CONNECTION_DATA_LIST;

pub fn session_id_exists(session_id: &str) -> bool {
    let session = CONNECTION_DATA_LIST.read().unwrap();
    session
        .iter()
        .any(|data| data.get_session_id() == session_id)
}

pub fn remove_session_id(session_id: &str) {
    let mut session = CONNECTION_DATA_LIST.write().unwrap();
    session.retain(|data| data.get_session_id() != session_id);
}
