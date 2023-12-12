use hmac::{Hmac, Mac};
use lib_utils::b64::b64u_encode;
use sha2::Sha512;

use crate::{auth_config, pwd::ContentToHash};

use super::{Error, Result, Scheme};

pub struct Scheme01;

impl Scheme for Scheme01 {
	fn hash(&self, to_hash: &ContentToHash) -> Result<String> {
		let key = &auth_config().PWD_KEY;
		hash(key, to_hash)
	}

	fn validate(&self, to_hash: &ContentToHash, raw_pwd_ref: &str) -> Result<()> {
		let raw_pwd_new = self.hash(to_hash)?;
		if raw_pwd_new == raw_pwd_ref {
			Ok(())
		} else {
			Err(Error::PwdValidate)
		}
	}
}

fn hash(key: &[u8], to_hash: &ContentToHash) -> Result<String> {
	let ContentToHash { content, salt } = to_hash;

	let mut hmac_sha512 =
		Hmac::<Sha512>::new_from_slice(key).map_err(|_| Error::Key)?;

	hmac_sha512.update(content.as_bytes());
	hmac_sha512.update(salt.as_bytes());

	let hmac_result = hmac_sha512.finalize();
	let result_bytes = hmac_result.into_bytes();

	let result = b64u_encode(result_bytes);

	Ok(result)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::auth_config;
	use anyhow::Result;
	use uuid::Uuid;

	#[test]
	fn test_scheme_01_hash_into_b64u_ok() -> Result<()> {
		let fx_salt = Uuid::parse_str("f05e8961-d6ad-4086-9e78-a6de065e5453")?;
		let fx_key = &auth_config().PWD_KEY;
		let fx_to_hash = ContentToHash {
			content: "hello world".to_string(),
			salt: fx_salt,
		};
		let fx_res = "DmDn0hgLVbx7mr4-0idVLoRnpBjRdSACamGKkcdIlwmkuWPSSGA1zS0vZzlyMAXQ6nubesWZzYbjI-pEPTyflw";

		let res = hash(fx_key, &fx_to_hash)?;

		assert_eq!(res, fx_res);

		Ok(())
	}
}
