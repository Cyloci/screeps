use rand::{rngs::StdRng, RngCore, SeedableRng};

/// implement a custom randomness generator for the getrandom crate,
/// because the `js` feature expects the Node.js WebCrypto API to be available
/// (it's not available in the Screeps Node.js environment)
pub fn custom_getrandom(buf: &mut [u8]) -> Result<(), getrandom::Error> {
    let mut rng = StdRng::seed_from_u64(js_sys::Math::random().to_bits());
    rng.fill_bytes(buf);
    Ok(())
}
