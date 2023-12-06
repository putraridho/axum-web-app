mod error;
pub mod pwd;
pub mod token;

use hmac::{Hmac, Mac};
use sha2::Sha512;

use crate::utils::b64u_encode;

pub use self::error::{Error, Result};

pub struct EncryptContent {
	pub content: String,
	pub salt: String,
}

pub fn encrypt_into_b64u(
	key: &[u8],
	enc_content: &EncryptContent,
) -> Result<String> {
	let EncryptContent { content, salt } = enc_content;

	// -- Create a HMAC-SHA-512 from key.
	let mut hmac_sha512 =
		Hmac::<Sha512>::new_from_slice(key).map_err(|_| Error::KeyFailHmac)?;

	// -- Add content.
	hmac_sha512.update(content.as_bytes());
	hmac_sha512.update(salt.as_bytes());

	// -- Finalize and b64 encode.
	let hmac_result = hmac_sha512.finalize();
	let result = b64u_encode(hmac_result.into_bytes());

	Ok(result)
}

#[cfg(test)]
mod tests {
	use super::*;
	use anyhow::Result;
	use rand::RngCore;

	#[test]
	fn test_encrypt_into_b64u_ok() -> Result<()> {
		let mut fx_key = [0u8; 64];
		rand::thread_rng().fill_bytes(&mut fx_key);

		let fx_enc_content = EncryptContent {
			content: "hello world".to_string(),
			salt: "some pepper".to_string(),
		};

		// TODO: Need to fix fx_key, and precompute fx_res.
		let fx_res = encrypt_into_b64u(&fx_key, &fx_enc_content)?;

		let res = encrypt_into_b64u(&fx_key, &fx_enc_content)?;

		assert_eq!(res, fx_res);

		Ok(())
	}
}
