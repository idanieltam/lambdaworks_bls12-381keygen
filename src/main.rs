use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::{curve::BLS12381Curve};
use lambdaworks_math::{cyclic_group::IsGroup, elliptic_curve::traits::IsEllipticCurve};
use lambdaworks_math::traits::{AsBytes, ByteConversion};
use lambdaworks_math::unsigned_integer::element::U256;

/// The function generates the public key from a given secret key
/// using the BLS12-381 curve.
fn main() {
    let secret_key = "0x6C616D6264617370";
    let pk = generate_public_key_hex(secret_key);

    // public key in hex: "EFC2D10AD531CEBF2B8C7B4325BC93ED91E6477D260304C1F9ECC7BA0E6F5711"
    println!("public key in hex: {:?}", pk);
}

fn generate_public_key_hex(secret_key: &str) -> String {
    let generator = BLS12381Curve::generator();
    let private_key = U256::from_hex(secret_key).unwrap();
    let public_key = generator.operate_with_self(private_key);
    let public_key_bytes = public_key.as_bytes();
    let pk = U256::from_bytes_be(&public_key_bytes).unwrap();
    pk.to_hex()
}