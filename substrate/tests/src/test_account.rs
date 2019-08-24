extern crate rand;
extern crate ed25519_dalek;
extern crate schnorrkel;
extern crate sha2;

use rand::Rng;
use rand::rngs::OsRng;
use ed25519_dalek::Keypair as Ed25519KeyPair;
use ed25519_dalek::Signature;
use schnorrkel::Keypair as Sr25519KeyPair;
use schnorrkel::{MiniSecretKey, SecretKey};
use sha2::Sha512;


struct Ed25519;

impl Crypto for Ed25519 {
    type Pair = ed25519::Pair;
    type Public = ed25519::Public;

    fn pair_from_suri(suri: &str, password_override: Option<&str>) -> Self::Pair {
        ed25519::Pair::from_legacy_string(suri, password_override)
    }
}

//#[cfg(feature = "std")]
//pub fn get_ed25519_pair() {
//    println!("into get ");
//    let mut csprng: OsRng = OsRng::new().unwrap();
//    // let keypair: Ed25519KeyPair = Ed25519KeyPair::generate(&mut csprng);
//    let keypair: Ed25519KeyPair = Ed25519KeyPair::generate();
//    println!("public is {:?}, private is {:?}", keypair.public, keypair.secret);
//}

//#[cfg(all(feature = "std", feature = "sha2"))]
//pub fn get_sr25519_pair() {
//    let mut csprng: OsRng = OsRng::new().unwrap();
//    let keypair: Sr25519KeyPair = Sr25519KeyPair::generate_with(&mut csprng);
//    println!("public is {:?}, private is {:?}", keypair.public, keypair.secret);
//}
//
//#[cfg(feature = "std")]
//fn main() {
//    println!("main");
//    get_ed25519_pair();
//    get_sr25519_pair();
//}


//fn main() -> Result<(), std::io::Error> {
//    get_ed25519_pair();
//    Ok(())
//}
