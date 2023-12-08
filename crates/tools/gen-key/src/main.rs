use anyhow::Result;
use lib_utils::b64::b64u_encode;
use rand::RngCore;

fn main() -> Result<()> {
	let mut key = [0u8; 64];
	rand::thread_rng().fill_bytes(&mut key);
	println!("\nGenerated key for HMAC:\n{key:?}");

	let b64u = b64u_encode(key);
	println!("\nKey b64u encoded:\n{b64u}");

	Ok(())
}