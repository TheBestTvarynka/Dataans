use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
use uuid::Uuid;

use super::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignUpRequest {
    pub invitation_token: InvitationToken,
    pub username: Username,
    pub password: Password,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInRequest {
    pub username: Username,
    pub password: Password,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: PrimitiveDateTime,
    pub expiration_date: PrimitiveDateTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde() {
        let data = SignUpRequest {
            invitation_token: InvitationToken::try_from(vec![1, 2, 3, 4]).unwrap(),
            username: Username::try_from("tbt").unwrap(),
            password: Password::try_from("quest1!").unwrap(),
        };

        let json = serde_json::to_string(&data).unwrap();
        println!("{}", json);

        let raw = "{\"invitation_token\":[1,2,3,4],\"username\":\"tbt\",\"password\":\"quest1!\"}";
        let data = serde_json::from_str::<SignUpRequest>(raw).unwrap();
        println!("{:?}", data);
    }
}
