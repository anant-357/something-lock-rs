use pam::Client;

pub fn auth(service: String, username: String, password: String) -> Result<(), pam::PamError> {
    let mut client = Client::with_password(service.as_str()).unwrap();
    client
        .conversation_mut()
        .set_credentials(username, password);
    client.authenticate()
}
