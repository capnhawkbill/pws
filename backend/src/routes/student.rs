use crate::auth::Auth;

/// Testing route for student
#[get("/student")]
pub fn student(auth: Auth) -> String {
    format!("Hello {}", auth.username)
}
