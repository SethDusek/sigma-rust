//! Elliptic curve point.

use alloc::string::String;
use core::convert::TryFrom;
use core::default;
use core::ops::{Add, Mul, Neg};
use derive_more::{From, Into};
use elliptic_curve::ops::MulByGenerator;
use k256::elliptic_curve::group::prime::PrimeCurveAffine;
use k256::elliptic_curve::sec1::ToEncodedPoint;
use k256::{AffinePoint, ProjectivePoint, PublicKey, Scalar};
use sigma_ser::vlq_encode::{ReadSigmaVlqExt, WriteSigmaVlqExt};
use sigma_ser::{ScorexParsingError, ScorexSerializable, ScorexSerializeResult};

// /// Elliptic curve point
#[derive(Clone, Copy, From)]
#[cfg_attr(
    feature = "json",
    derive(serde::Serialize, serde::Deserialize),
    serde(into = "String", try_from = "String")
)]
#[allow(missing_docs)]
pub enum EcPoint {
    Affine(AffinePoint),
    Projective(ProjectivePoint),
}
impl Default for EcPoint {
    fn default() -> Self {
        AffinePoint::default().into()
    }
}

impl PartialEq for EcPoint {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (EcPoint::Affine(a1), EcPoint::Affine(a2)) => a1 == a2,
            (EcPoint::Affine(a1), EcPoint::Projective(p1)) => p1.eq_affine(a1).unwrap_u8() == 1,
            (EcPoint::Projective(p1), EcPoint::Affine(a1)) => p1.eq_affine(a1).unwrap_u8() == 1,
            (EcPoint::Projective(p1), EcPoint::Projective(p2)) => p1 == p2,
        }
    }
}

#[allow(clippy::unwrap_used)]
impl core::fmt::Debug for EcPoint {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str("EC:")?;
        f.write_str(&base16::encode_lower(
            &self.scorex_serialize_bytes().unwrap(),
        ))
    }
}

#[allow(clippy::unwrap_used)]
impl core::fmt::Display for EcPoint {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(&base16::encode_lower(
            &self.scorex_serialize_bytes().unwrap(),
        ))
    }
}

impl EcPoint {
    /// Number of bytes to represent any group element as byte array
    pub const GROUP_SIZE: usize = 33;

    /// Attempts to parse from Base16-encoded string
    pub fn from_base16_str(str: String) -> Option<Self> {
        base16::decode(&str)
            .ok()
            .and_then(|bytes| Self::scorex_parse_bytes(&bytes).ok())
    }
}

impl TryFrom<String> for EcPoint {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        EcPoint::from_base16_str(value)
            .ok_or_else(|| String::from("Ecpoint: error parsing from base16-encoded string"))
    }
}

impl From<EcPoint> for String {
    fn from(value: EcPoint) -> String {
        #[allow(clippy::unwrap_used)]
        {
            let bytes = value.scorex_serialize_bytes().unwrap();
            base16::encode_lower(&bytes)
        }
    }
}

impl Eq for EcPoint {}

impl Mul<&EcPoint> for EcPoint {
    type Output = EcPoint;

    fn mul(self, other: &EcPoint) -> EcPoint {
        match (self, other) {
            (EcPoint::Affine(a1), EcPoint::Affine(a2)) => ProjectivePoint::from(a1) + a2,
            (EcPoint::Affine(affine_point), EcPoint::Projective(projective_point)) => {
                projective_point + &affine_point
            }
            (EcPoint::Projective(projective_point), EcPoint::Affine(affine_point)) => {
                projective_point + affine_point
            }
            (EcPoint::Projective(p1), EcPoint::Projective(p2)) => p1 + p2,
        }
        .into()
        // EcPoint(ProjectivePoint::add(self.0, &other.0))
    }
}

impl Neg for EcPoint {
    type Output = EcPoint;

    fn neg(self) -> EcPoint {
        match self {
            EcPoint::Affine(affine_point) => (-affine_point).into(),
            EcPoint::Projective(projective_point) => (-projective_point).into(),
        }
        // EcPoint::from(ProjectivePoint::neg(self.0))
    }
}

/// The generator g of the group is an element of the group such that, when written multiplicatively, every element
/// of the group is a power of g.
pub fn generator() -> EcPoint {
    EcPoint::from(ProjectivePoint::GENERATOR)
}

/// The identity(infinity) element
pub fn identity() -> EcPoint {
    EcPoint::from(ProjectivePoint::IDENTITY)
}

/// Check if point is identity(infinity) element
pub fn is_identity(ge: &EcPoint) -> bool {
    *ge == identity()
}

/// Calculates the inverse of the given group element
pub fn inverse(ec: &EcPoint) -> EcPoint {
    -*ec
}

/// Raises the base GroupElement to the exponent. The result is another GroupElement.
pub fn exponentiate(base: &EcPoint, exponent: &Scalar) -> EcPoint {
    if !is_identity(base) {
        match base {
            EcPoint::Affine(affine_point) => (*affine_point * exponent).into(),
            EcPoint::Projective(projective_point) => (projective_point * exponent).into(),
        }
    } else {
        *base
    }
}

/// Raise the generator g to the exponent. This is faster than exponentiate(&generator(), exponent)
pub fn exponentiate_gen(exponent: &Scalar) -> EcPoint {
    ProjectivePoint::mul_by_generator(exponent).into()
}

impl ScorexSerializable for EcPoint {
    fn scorex_serialize<W: WriteSigmaVlqExt>(&self, w: &mut W) -> ScorexSerializeResult {
        // let now = std::time::Instant::now();
        let caff = match self {
            EcPoint::Affine(affine_point) => *affine_point,
            EcPoint::Projective(projective_point) => projective_point.to_affine(),
        };
        if caff.is_identity().into() {
            // infinity point
            let zeroes = [0u8; EcPoint::GROUP_SIZE];
            w.write_all(&zeroes)?;
        } else {
            w.write_all(caff.to_encoded_point(true).as_bytes())?;
        }
        // println!("{} {:?}", matches!(self, EcPoint::Affine(_)), now.elapsed());
        Ok(())
    }

    fn scorex_parse<R: ReadSigmaVlqExt>(r: &mut R) -> Result<Self, ScorexParsingError> {
        let mut buf = [0; EcPoint::GROUP_SIZE];
        r.read_exact(&mut buf[..])?;
        if buf[0] != 0 {
            let pubkey = PublicKey::from_sec1_bytes(&buf[..]).map_err(|e| {
                ScorexParsingError::Misc(format!("failed to parse PK from bytes: {:?}", e))
            })?;
            Ok(EcPoint::from(*pubkey.as_affine()))
        } else {
            // infinity point
            Ok(EcPoint::from(ProjectivePoint::IDENTITY))
        }
    }
}

/// Arbitrary impl for EcPoint
#[cfg(feature = "arbitrary")]
mod arbitrary {
    use super::*;
    use proptest::prelude::*;

    impl Arbitrary for EcPoint {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            prop_oneof![
                Just(generator()),
                Just(identity()), /*Just(random_element()),*/
            ]
            .boxed()
        }
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
#[cfg(feature = "arbitrary")]
#[allow(clippy::panic)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use sigma_ser::scorex_serialize_roundtrip;

    proptest! {

        #[test]
        fn ser_roundtrip(v in any::<EcPoint>()) {
            prop_assert_eq![scorex_serialize_roundtrip(&v), v];
        }

    }
}
