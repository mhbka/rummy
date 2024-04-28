/// A wrapper type for all requests/responses from these routes.
#[derive(serde::Serialize, serde::Deserialize)]
struct UserBody<T> {
    user: T,
}

#[derive(serde::Deserialize)]
struct NewUser {
    username: String,
    email: String,
    password: String,
}

#[derive(serde::Deserialize)]
struct LoginUser {
    email: String,
    password: String,
}

#[derive(serde::Deserialize, Default, PartialEq, Eq)]
#[serde(default)] // fill in any missing fields with `..UpdateUser::default()`
struct UpdateUser {
    email: Option<String>,
    username: Option<String>,
    password: Option<String>,
    bio: Option<String>,
    image: Option<String>,
}

// TODO: separate struct or something for updating coins

#[derive(serde::Serialize, serde::Deserialize)]
struct User {
    email: String,
    token: String,
    username: String,
    bio: String,
    image: Option<String>,
    coins: u64
}