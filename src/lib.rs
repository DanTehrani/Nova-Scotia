
use nova_snark::{
    traits::{Group}
};

pub mod circom;

pub type G1 = secpq_curves::secq256k1::Point;
pub type F1 = <G1 as Group>::Scalar;
pub type G2 = secpq_curves::secp256k1::Point;
pub type F2 = <G2 as Group>::Scalar;