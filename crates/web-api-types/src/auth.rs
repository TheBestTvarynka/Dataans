use nutype::nutype;
use serde::{Deserialize, Serialize};

#[nutype(validate(not_empty), derive(Debug, Serialize, Deserialize, AsRef, Deref, TryFrom))]
pub struct Username(String);

#[nutype(validate(not_empty), derive(Debug, Serialize, Deserialize, AsRef, Deref, TryFrom))]
pub struct Password(String);

#[nutype(
    validate(predicate = |token| !token.is_empty()),
    derive(Debug, Serialize, Deserialize, AsRef, Deref, TryFrom),
)]
pub struct InvitationToken(Vec<u8>);

#[derive(Debug, Serialize, Deserialize)]
pub struct SignUpRequest {
    pub invitation_token: InvitationToken,
    pub username: Username,
    pub password: Password,
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
