use std::{
    collections::{BTreeMap, HashMap},
    env,
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

use bcrypt::verify;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use protos::auth::{auth_server::Auth, LoginRequest, RegisterRequest, Token};
use sha2::Sha256;
use tonic::{Request, Response, Status};

use crate::database::{
    conn::PgPooledConnection,
    models::{DatabaseUser, NewDatabaseUser},
};

use super::token_err::CheckTokenError;

pub struct Service {
    database: Arc<Mutex<PgPooledConnection>>,
}

impl Service {
    pub fn new(database: PgPooledConnection) -> Self {
        Self {
            database: Arc::new(Mutex::new(database)),
        }
    }
}

#[tonic::async_trait]
impl Auth for Service {
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<Token>, Status> {
        let data = request.into_inner();

        let database = self.database.lock();
        let user = DatabaseUser::find_by_username(&mut database.unwrap(), &data.username)
            .ok_or(Status::unauthenticated("Invalid username or password"))?;

        match verify(data.password, &user.password) {
            Ok(true) => (),
            Ok(false) | Err(_) => {
                return Err(Status::unauthenticated("Invalid username or password"))
            }
        };

        let reply = generate_token(user)
            .map_err(|_| Status::unauthenticated("Invalid username or password"))?;

        Ok(Response::new(reply))
    }

    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<Token>, Status> {
        let data = request.into_inner();
        let database = self.database.lock();

        let password = bcrypt::hash(&data.password, 10)
            .map_err(|_| Status::unknown("Error while creating the user"))?;

        let user = NewDatabaseUser {
            username: &data.username,
            password: &password,
            student_id: &data.student_id,
        };

        let user = DatabaseUser::create(&mut database.unwrap(), user)
            .map_err(|_| Status::already_exists("User already exists in the database"))?;

        let token = generate_token(user)
            .map_err(|_| Status::unknown("Cannot generate a token for the User"))?;

        Ok(Response::new(token))
    }
}

pub struct GenerateTokenError;
pub struct GenerateClaimsError;

fn generate_claims(
    user: DatabaseUser,
) -> Result<BTreeMap<&'static str, String>, GenerateClaimsError> {
    let mut claims: BTreeMap<&str, String> = BTreeMap::new();

    claims.insert("sub", user.student_id.to_string());

    let current_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| GenerateClaimsError)?
        .as_secs();

    claims.insert("iat", current_timestamp.to_string());
    claims.insert("exp", String::from("3600"));

    Ok(claims)
}

fn generate_token(user: DatabaseUser) -> Result<Token, GenerateTokenError> {
    let app_key: String = env::var("USER_KEY").expect("env USER_KEY is not defined");

    let key: Hmac<Sha256> =
        Hmac::new_from_slice(app_key.as_bytes()).map_err(|_| GenerateTokenError)?;

    let claims = generate_claims(user).map_err(|_| GenerateTokenError)?;

    let access_token = claims.sign_with_key(&key).map_err(|_| GenerateTokenError)?;

    Ok(Token { access_token })
}

pub fn resolve_token(env_key: &str) -> Result<Hmac<Sha256>, CheckTokenError> {
    let app_key: String =
        env::var(env_key).unwrap_or_else(|_| panic!("env {} is not defined", env_key));

    Hmac::new_from_slice(app_key.as_bytes()).map_err(|_| CheckTokenError::VerifyTokenError)
}

pub fn get_token<T>(request: &Request<T>) -> &str {
    request
        .metadata()
        .get("x-authorization")
        .ok_or(Status::unauthenticated("No access token specified"))
        .unwrap()
        .to_str()
        .map_err(|_| Status::unauthenticated("No access token specified"))
        .unwrap()
}

pub struct CheckUser<'a> {
    token: String,
    database: &'a mut PgPooledConnection,
}

impl<'a> CheckUser<'a> {
    pub fn new<T>(request: &Request<T>, database: &'a mut PgPooledConnection) -> Self {
        Self {
            token: get_token(request).to_owned(),
            database,
        }
    }

    fn verify_token(&mut self, env_key: &str) -> Result<bool, CheckTokenError> {
        let key = resolve_token(env_key)?;

        let claims = self.token.verify_with_key(&key);

        let verify_token = claims
            .as_ref()
            .map(|_: &HashMap<String, String>| true)
            .unwrap_or(false);

        let sub = claims.as_ref().unwrap().get("sub").unwrap().as_str();
        let verify_student_id = DatabaseUser::student_id_exists(self.database, sub);

        match verify_token && verify_student_id {
            true => Ok(true),
            false => Err(CheckTokenError::VaildationTokenError),
        }
    }

    pub fn check(&mut self) {
        match Self::verify_token(self, "USER_KEY") {
            Ok(true) => (),
            Err(_) | Ok(false) => panic!("Invalid token"),
        }
    }

    pub fn check_owner(&self, room_owner: &str) -> bool {
        self.to_student_id() == room_owner
    }
}

pub trait ToStudentId {
    fn to_student_id(&self) -> String;
}

impl ToStudentId for CheckUser<'_> {
    fn to_student_id(&self) -> String {
        let key = resolve_token("USER_KEY").unwrap();
        let claims: HashMap<String, String> = self.token.verify_with_key(&key).unwrap();

        claims.get("sub").unwrap().to_string()
    }
}
