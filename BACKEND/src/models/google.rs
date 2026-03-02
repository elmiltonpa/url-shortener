use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GoogleTokenResponse {
    pub access_token: String,
    pub expires_in: i32,
    pub scope: String,
    pub token_type: String,
    pub id_token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GoogleUserInfo {
    pub sub: String,          // ID único de Google
    pub name: String,         // Nombre completo
    pub given_name: String,   // Nombre
    pub family_name: String,  // Apellido
    pub picture: String,      // URL de la foto de perfil
    pub email: String,        // Email del usuario
    pub email_verified: bool, // Si el email está verificado
}
