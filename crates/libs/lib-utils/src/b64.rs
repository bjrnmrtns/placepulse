use base64::engine::{general_purpose, Engine};

pub fn b64u_encode(content: impl AsRef<[u8]>) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(content)
}
