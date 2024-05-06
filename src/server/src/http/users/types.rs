/// A wrapper type for all requests/responses from these routes.
#[derive(serde::Serialize, serde::Deserialize)]
pub(super) struct UserBody<T> {
    pub(super) user: T,
}

#[derive(serde::Deserialize)]
pub(super) struct NewUser {
    pub(super) username: String,
    pub(super) email: String,
    pub(super) password: String,
}

#[derive(serde::Deserialize)]
pub(super) struct LoginUser {
    pub(super) email: String,
    pub(super) password: String,
}

#[derive(serde::Deserialize, Default, PartialEq, Eq)]
#[serde(default)]
// fill in any missing fields with `..UpdateUser::default()`
pub(super) struct UpdateUser {
    pub(super) email: Option<String>,
    pub(super) username: Option<String>,
    pub(super) password: Option<String>,
    pub(super) bio: Option<String>,
    pub(super) image: Option<String>,
}

// TODO: separate struct for updating coins?
#[derive(serde::Serialize, serde::Deserialize)]
pub(super) struct User {
    pub(super) email: String,
    pub(super) token: String,
    pub(super) username: String,
    pub(super) bio: String,
    pub(super) image: Option<String>,
    pub(super) coins: u64
}