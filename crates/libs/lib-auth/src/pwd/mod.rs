use std::str::FromStr;

use lazy_regex::regex_captures;
use uuid::Uuid;

pub use self::error::{Error, Result};
pub use self::scheme::SchemeStatus;
use self::scheme::{get_scheme, Scheme, DEFAULT_SCHEME};

mod error;
mod scheme;

#[cfg_attr(test, derive(Clone))]
pub struct ContentToHash {
	pub content: String,
	pub salt: Uuid,
}

pub async fn hash_pwd(to_hash: ContentToHash) -> Result<String> {
	tokio::task::spawn_blocking(move || hash_for_scheme(DEFAULT_SCHEME, to_hash))
		.await
		.map_err(|_| Error::FailSpawnBlockForHash)?
}

pub async fn validate_pwd(
	to_hash: ContentToHash,
	pwd_ref: String,
) -> Result<SchemeStatus> {
	let PwdParts {
		scheme_name,
		hashed,
	} = pwd_ref.parse()?;

	let scheme_status = if scheme_name == DEFAULT_SCHEME {
		SchemeStatus::Ok
	} else {
		SchemeStatus::Outdated
	};

	tokio::task::spawn_blocking(move || {
		validate_for_scheme(&scheme_name, to_hash, hashed)
	})
	.await
	.map_err(|_| Error::FailSpawnBlockForValidate)??;

	Ok(scheme_status)
}

fn hash_for_scheme(scheme_name: &str, to_hash: ContentToHash) -> Result<String> {
	let pwd_hashed = get_scheme(scheme_name)?.hash(&to_hash)?;

	Ok(format!("#{scheme_name}#{pwd_hashed}"))
}

fn validate_for_scheme(
	scheme_name: &str,
	to_hash: ContentToHash,
	pwd_ref: String,
) -> Result<()> {
	get_scheme(scheme_name)?.validate(&to_hash, &pwd_ref)?;
	Ok(())
}
struct PwdParts {
	scheme_name: String,
	hashed: String,
}

impl FromStr for PwdParts {
	type Err = Error;

	fn from_str(pwd_with_scheme: &str) -> Result<Self> {
		regex_captures!(r#"^#(\w+)#(.*)"#, pwd_with_scheme)
			.map(|(_, scheme, hashed)| Self {
				scheme_name: scheme.to_string(),
				hashed: hashed.to_string(),
			})
			.ok_or(Error::PwdWithSchemeFailedParse)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use anyhow::Result;

	#[tokio::test]
	async fn test_multi_scheme_ok() -> Result<()> {
		// -- Setup & Fixtures
		let fx_salt = Uuid::parse_str("f05e8961-d6ad-4086-9e78-a6de065e5453")?;
		let fx_to_hash = ContentToHash {
			content: "hello world".to_string(),
			salt: fx_salt,
		};

		// -- Exec
		let pwd_hashed = hash_for_scheme("01", fx_to_hash.clone())?;
		let pwd_validate = validate_pwd(fx_to_hash.clone(), pwd_hashed).await?;

		// -- Check
		assert!(
			matches!(pwd_validate, SchemeStatus::Outdated),
			"status should be SchemeStatus::Outdated"
		);

		Ok(())
	}
}
