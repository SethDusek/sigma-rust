//! Constant(Literal) IR node

use crate::base16_str::Base16Str;
use crate::bigint256::BigInt256;
use crate::chain::ergo_box::ErgoBox;
use crate::chain::token::TokenId;
use crate::mir::value::CollKind;
use crate::reference::Ref;
use crate::serialization::SigmaParsingError;
use crate::serialization::SigmaSerializable;
use crate::serialization::SigmaSerializationError;
use crate::sigma_protocol::sigma_boolean::SigmaBoolean;
use crate::sigma_protocol::sigma_boolean::SigmaProofOfKnowledgeTree;
use crate::sigma_protocol::sigma_boolean::SigmaProp;
use crate::sigma_protocol::sigma_boolean::{ProveDhTuple, ProveDlog};
use crate::types::stuple::STuple;
use crate::types::stuple::TupleItems;
use crate::types::stype::LiftIntoSType;
use crate::types::stype::SType;
use ergo_chain_types::ADDigest;
use ergo_chain_types::Base16DecodedBytes;
use ergo_chain_types::Digest32;
use ergo_chain_types::EcPoint;
use impl_trait_for_tuples::impl_for_tuples;
use sigma_util::AsVecI8;
use sigma_util::AsVecU8;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt::Formatter;
use std::rc::Rc;

mod constant_placeholder;

pub use constant_placeholder::*;

use super::avl_tree_data::AvlTreeData;
use super::avl_tree_data::AvlTreeFlags;
use super::value::NativeColl;
use super::value::StoreWrapped;
use super::value::Value;

use thiserror::Error;

#[derive(PartialEq, Eq, Clone)]
/// Constant
pub struct Constant<'ctx> {
    /// Constant type
    pub tpe: SType,
    /// Constant value
    pub v: Literal<'ctx>,
}

#[derive(PartialEq, Eq, Clone)]
/// Possible values for `Constant`
pub enum Literal<'ctx> {
    /// Unit
    Unit,
    /// Boolean
    Boolean(bool),
    /// i8
    Byte(i8),
    /// Short
    Short(i16),
    /// Int
    Int(i32),
    /// Long
    Long(i64),
    /// Big integer
    BigInt(BigInt256),
    /// Sigma property
    SigmaProp(Box<SigmaProp>),
    /// GroupElement
    GroupElement(Box<EcPoint>),
    /// AVL tree
    AvlTree(Box<AvlTreeData>),
    /// Ergo box
    CBox(Ref<'ctx, ErgoBox>),
    /// Collection
    Coll(CollKind<Literal<'ctx>>),
    /// Option type
    Opt(Box<Option<Literal<'ctx>>>),
    /// Tuple (arbitrary type values)
    Tup(TupleItems<Literal<'ctx>>),
}

impl std::fmt::Debug for Constant<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format!("{:?}: {:?}", self.v, self.tpe).fmt(f)
    }
}

impl std::fmt::Display for Constant<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.v.fmt(f)
    }
}

impl std::fmt::Debug for Literal<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Coll(CollKind::NativeColl(NativeColl::CollByte(i8_bytes))) => {
                base16::encode_lower(&i8_bytes.as_vec_u8()).fmt(f)
            }
            Literal::Coll(CollKind::WrappedColl { elem_tpe: _, items }) => items.fmt(f),
            Literal::Opt(boxed_opt) => boxed_opt.fmt(f),
            Literal::Tup(items) => items.fmt(f),
            Literal::Unit => ().fmt(f),
            Literal::Boolean(v) => v.fmt(f),
            Literal::Byte(v) => v.fmt(f),
            Literal::Short(v) => v.fmt(f),
            Literal::Int(v) => v.fmt(f),
            Literal::Long(v) => v.fmt(f),
            Literal::BigInt(v) => v.fmt(f),
            Literal::SigmaProp(v) => v.fmt(f),
            Literal::GroupElement(v) => v.fmt(f),
            Literal::AvlTree(v) => v.fmt(f),
            Literal::CBox(v) => v.fmt(f),
        }
    }
}

impl std::fmt::Display for Literal<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Coll(CollKind::NativeColl(NativeColl::CollByte(i8_bytes))) => {
                write!(f, "Coll[Byte](")?;
                for (i, b) in i8_bytes.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", b)?;
                }
                write!(f, ")")
            }
            Literal::Coll(CollKind::WrappedColl { elem_tpe, items }) => {
                write!(f, "Coll[{}](", elem_tpe)?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    item.fmt(f)?;
                }
                write!(f, ")")
            }
            Literal::Opt(boxed_opt) => {
                if let Some(v) = &**boxed_opt {
                    write!(f, "Some(")?;
                    v.fmt(f)?;
                    write!(f, ")")
                } else {
                    write!(f, "None")
                }
            }
            Literal::Tup(items) => {
                write!(f, "(")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    item.fmt(f)?;
                }
                write!(f, ")")
            }
            Literal::Unit => write!(f, "()"),
            Literal::Boolean(v) => v.fmt(f),
            Literal::Byte(v) => v.fmt(f),
            Literal::Short(v) => v.fmt(f),
            Literal::Int(v) => v.fmt(f),
            Literal::Long(v) => write!(f, "{}L", v),
            Literal::BigInt(v) => v.fmt(f),
            Literal::SigmaProp(v) => v.fmt(f),
            Literal::GroupElement(v) => v.fmt(f),
            Literal::AvlTree(v) => write!(f, "AvlTree({:?})", v),
            Literal::CBox(v) => write!(f, "ErgoBox({:?})", v),
        }
    }
}

impl<'ctx> From<()> for Literal<'ctx> {
    fn from(_: ()) -> Literal<'ctx> {
        Literal::Unit
    }
}

impl<'ctx> From<bool> for Literal<'ctx> {
    fn from(v: bool) -> Literal<'ctx> {
        Literal::Boolean(v)
    }
}

impl<'ctx> From<i8> for Literal<'ctx> {
    fn from(v: i8) -> Literal<'ctx> {
        Literal::Byte(v)
    }
}

impl<'ctx> From<i16> for Literal<'ctx> {
    fn from(v: i16) -> Literal<'ctx> {
        Literal::Short(v)
    }
}

impl<'ctx> From<i32> for Literal<'ctx> {
    fn from(v: i32) -> Literal<'ctx> {
        Literal::Int(v)
    }
}

impl<'ctx> From<i64> for Literal<'ctx> {
    fn from(v: i64) -> Literal<'ctx> {
        Literal::Long(v)
    }
}

impl<'ctx> From<BigInt256> for Literal<'ctx> {
    fn from(v: BigInt256) -> Literal<'ctx> {
        Literal::BigInt(v)
    }
}

impl<'ctx> From<SigmaProp> for Literal<'ctx> {
    fn from(v: SigmaProp) -> Literal<'ctx> {
        Literal::SigmaProp(Box::new(v))
    }
}

impl<'ctx> From<EcPoint> for Literal<'ctx> {
    fn from(v: EcPoint) -> Literal<'ctx> {
        Literal::GroupElement(Box::new(v))
    }
}

impl<'ctx> From<Ref<'ctx, ErgoBox>> for Literal<'ctx> {
    fn from(b: Ref<'ctx, ErgoBox>) -> Self {
        Literal::CBox(b)
    }
}

impl<'ctx> From<ErgoBox> for Literal<'ctx> {
    fn from(b: ErgoBox) -> Self {
        Literal::CBox(Rc::new(b).into())
    }
}

impl<'ctx> From<Vec<u8>> for Literal<'ctx> {
    fn from(v: Vec<u8>) -> Self {
        Literal::Coll(CollKind::NativeColl(NativeColl::CollByte(v.as_vec_i8())))
    }
}

impl<'ctx> From<Digest32> for Literal<'ctx> {
    fn from(v: Digest32) -> Self {
        let bytes: Vec<u8> = v.into();
        Literal::Coll(CollKind::NativeColl(NativeColl::CollByte(
            bytes.as_vec_i8(),
        )))
    }
}

impl<'ctx> From<TokenId> for Literal<'ctx> {
    fn from(v: TokenId) -> Self {
        Digest32::from(v).into()
    }
}

impl<'ctx> From<Vec<i8>> for Literal<'ctx> {
    fn from(v: Vec<i8>) -> Self {
        Literal::Coll(CollKind::NativeColl(NativeColl::CollByte(v)))
    }
}

impl<'ctx, T: LiftIntoSType + StoreWrapped + Into<Literal<'ctx>>> From<Vec<T>> for Literal<'ctx> {
    fn from(v: Vec<T>) -> Self {
        Literal::Coll(CollKind::WrappedColl {
            elem_tpe: T::stype(),
            items: v.into_iter().map(|i| i.into()).collect(),
        })
    }
}

impl<'ctx, T: LiftIntoSType + Into<Literal<'ctx>>> From<Option<T>> for Literal<'ctx> {
    fn from(opt: Option<T>) -> Self {
        Literal::Opt(Box::new(opt.map(|e| e.into())))
    }
}

impl<'ctx> TryFrom<Value<'ctx>> for Constant<'ctx> {
    type Error = String;
    #[allow(clippy::unwrap_used)]
    fn try_from(value: Value<'ctx>) -> Result<Self, Self::Error> {
        match value {
            Value::Boolean(b) => Ok(Constant::from(b)),
            Value::Byte(b) => Ok(Constant::from(b)),
            Value::Short(s) => Ok(Constant::from(s)),
            Value::Int(i) => Ok(Constant::from(i)),
            Value::Long(l) => Ok(Constant::from(l)),
            Value::BigInt(b) => Ok(Constant::from(b)),
            Value::Unit => Ok(Constant {
                tpe: SType::SUnit,
                v: Literal::Unit,
            }),
            Value::SigmaProp(s) => Ok(Constant::from(*s)),
            Value::GroupElement(e) => Ok(Constant::from(*e)),
            Value::CBox(i) => Ok(Constant::from(i)),
            Value::Coll(coll) => {
                let (v, tpe) = match coll {
                    CollKind::NativeColl(n) => (
                        Literal::Coll(CollKind::NativeColl(n)),
                        SType::SColl(Box::new(SType::SByte)),
                    ),
                    CollKind::WrappedColl { elem_tpe, items } => {
                        let mut new_items = Vec::with_capacity(items.len());
                        for v in items {
                            let c = Constant::try_from(v)?;
                            new_items.push(c.v);
                        }
                        (
                            Literal::Coll(CollKind::WrappedColl {
                                elem_tpe: elem_tpe.clone(),
                                items: new_items,
                            }),
                            SType::SColl(Box::new(elem_tpe)),
                        )
                    }
                };
                Ok(Constant { v, tpe })
            }
            Value::Opt(lit) => match *lit {
                Some(v) => {
                    let c = Constant::try_from(v)?;
                    Ok(Constant {
                        v: Literal::Opt(Box::new(Some(c.v))),
                        tpe: c.tpe,
                    })
                }
                None => Err("Can't convert from Value::Opt(None) to Constant".into()),
            },
            Value::Tup(t) => {
                if let Ok(t) = t.try_mapped::<_, _, String>(|v| {
                    let c = Constant::try_from(v)?;
                    Ok((c.v, c.tpe))
                }) {
                    let tuple_items = t.mapped_ref(|(l, _)| l.clone());
                    let tuple_item_types = SType::STuple(STuple {
                        items: t.mapped(|(_, tpe)| tpe),
                    });
                    Ok(Constant {
                        v: Literal::Tup(tuple_items),
                        tpe: tuple_item_types,
                    })
                } else {
                    Err("Can't convert Value:Tup element".into())
                }
            }
            Value::AvlTree(a) => Ok(Constant::from(*a)),
            Value::Context => Err("Cannot convert Value::Context into Constant".into()),
            Value::Header(_) => Err("Cannot convert Value::Header(_) into Constant".into()),
            Value::PreHeader(_) => Err("Cannot convert Value::PreHeader(_) into Constant".into()),
            Value::Global => Err("Cannot convert Value::Global into Constant".into()),
            Value::Lambda(_) => Err("Cannot convert Value::Lambda(_) into Constant".into()),
        }
    }
}

impl<'ctx> From<()> for Constant<'ctx> {
    fn from(_: ()) -> Self {
        Constant {
            tpe: SType::SUnit,
            v: Literal::Unit,
        }
    }
}

impl<'ctx> From<bool> for Constant<'ctx> {
    fn from(v: bool) -> Self {
        Constant {
            tpe: bool::stype(),
            v: v.into(),
        }
    }
}

impl<'ctx> From<i8> for Constant<'ctx> {
    fn from(v: i8) -> Self {
        Constant {
            tpe: i8::stype(),
            v: v.into(),
        }
    }
}

impl<'ctx> From<i16> for Constant<'ctx> {
    fn from(v: i16) -> Self {
        Constant {
            tpe: i16::stype(),
            v: v.into(),
        }
    }
}

impl<'ctx> From<i32> for Constant<'ctx> {
    fn from(v: i32) -> Self {
        Constant {
            tpe: i32::stype(),
            v: v.into(),
        }
    }
}

impl<'ctx> From<i64> for Constant<'ctx> {
    fn from(v: i64) -> Self {
        Constant {
            tpe: i64::stype(),
            v: v.into(),
        }
    }
}

impl<'ctx> From<SigmaProp> for Constant<'ctx> {
    fn from(v: SigmaProp) -> Self {
        Constant {
            tpe: SType::SSigmaProp,
            v: v.into(),
        }
    }
}

impl<'ctx> From<EcPoint> for Constant<'ctx> {
    fn from(v: EcPoint) -> Constant<'ctx> {
        Constant {
            tpe: SType::SGroupElement,
            v: v.into(),
        }
    }
}

impl<'ctx> From<&'ctx ErgoBox> for Constant<'ctx> {
    fn from(b: &'ctx ErgoBox) -> Self {
        Constant {
            tpe: SType::SBox,
            v: Literal::CBox(Ref::Borrowed(b)),
        }
    }
}

impl<'ctx> From<ErgoBox> for Constant<'ctx> {
    fn from(b: ErgoBox) -> Self {
        Constant {
            tpe: SType::SBox,
            v: Literal::CBox(Rc::new(b).into()),
        }
    }
}

impl<'ctx> From<Ref<'ctx, ErgoBox>> for Constant<'ctx> {
    fn from(b: Ref<'ctx, ErgoBox>) -> Self {
        Constant {
            tpe: SType::SBox,
            v: Literal::CBox(b),
        }
    }
}

impl<'ctx> From<Vec<u8>> for Constant<'ctx> {
    fn from(v: Vec<u8>) -> Self {
        Constant {
            tpe: SType::SColl(Box::new(SType::SByte)),
            v: v.into(),
        }
    }
}

impl<'ctx> From<Digest32> for Constant<'ctx> {
    fn from(v: Digest32) -> Self {
        Constant {
            tpe: SType::SColl(Box::new(SType::SByte)),
            v: v.into(),
        }
    }
}

impl<'ctx> From<TokenId> for Constant<'ctx> {
    fn from(v: TokenId) -> Self {
        Digest32::from(v).into()
    }
}

impl<'ctx> From<Vec<i8>> for Constant<'ctx> {
    fn from(v: Vec<i8>) -> Self {
        Constant {
            tpe: SType::SColl(Box::new(SType::SByte)),
            v: v.into(),
        }
    }
}

impl<'ctx, T: LiftIntoSType + StoreWrapped + Into<Constant<'ctx>>> From<Vec<T>> for Constant<'ctx> {
    fn from(v: Vec<T>) -> Self {
        Constant {
            tpe: Vec::<T>::stype(),
            v: Literal::Coll(CollKind::WrappedColl {
                elem_tpe: T::stype(),
                items: v.into_iter().map(|i| i.into().v).collect(),
            }),
        }
    }
}

impl<'ctx, T: LiftIntoSType + Into<Constant<'ctx>>> From<Option<T>> for Constant<'ctx> {
    fn from(opt: Option<T>) -> Self {
        Constant {
            tpe: SType::SOption(Box::new(T::stype())),
            v: Literal::Opt(Box::new(opt.map(|e| e.into().v))),
        }
    }
}

impl<'ctx> From<ProveDlog> for Constant<'ctx> {
    fn from(v: ProveDlog) -> Self {
        Constant::from(SigmaProp::from(SigmaBoolean::from(
            SigmaProofOfKnowledgeTree::from(v),
        )))
    }
}

impl<'ctx> From<ProveDhTuple> for Constant<'ctx> {
    fn from(dht: ProveDhTuple) -> Self {
        Constant::from(SigmaProp::from(SigmaBoolean::from(
            SigmaProofOfKnowledgeTree::from(dht),
        )))
    }
}

impl<'ctx> From<SigmaBoolean> for Constant<'ctx> {
    fn from(sb: SigmaBoolean) -> Self {
        Constant::from(SigmaProp::from(sb))
    }
}

impl<'ctx> From<BigInt256> for Constant<'ctx> {
    fn from(b: BigInt256) -> Self {
        Constant {
            tpe: SType::SBigInt,
            v: Literal::BigInt(b),
        }
    }
}

impl<'ctx> From<AvlTreeData> for Constant<'ctx> {
    fn from(a: AvlTreeData) -> Self {
        Constant {
            tpe: SType::SAvlTree,
            v: Literal::AvlTree(Box::new(a)),
        }
    }
}

impl<'ctx> From<AvlTreeFlags> for Constant<'ctx> {
    fn from(a: AvlTreeFlags) -> Self {
        Constant {
            tpe: SType::SByte,
            v: Literal::Byte(a.serialize() as i8),
        }
    }
}

impl<'ctx> From<ADDigest> for Constant<'ctx> {
    fn from(a: ADDigest) -> Self {
        Constant {
            tpe: SType::SColl(Box::new(SType::SByte)),
            v: Literal::Coll(CollKind::NativeColl(NativeColl::CollByte(a.into()))),
        }
    }
}

#[allow(clippy::unwrap_used)]
#[allow(clippy::from_over_into)]
#[impl_for_tuples(2, 4)]
impl<'ctx> Into<Constant<'ctx>> for Tuple {
    fn into(self) -> Constant<'ctx> {
        let constants: Vec<Constant> = [for_tuples!(  #( Tuple.into() ),* )].to_vec();
        let (types, values): (Vec<SType>, Vec<Literal>) =
            constants.into_iter().map(|c| (c.tpe, c.v)).unzip();
        Constant {
            tpe: SType::STuple(types.try_into().unwrap()),
            v: Literal::Tup(values.try_into().unwrap()),
        }
    }
}

/// Extract value wrapped in a type
pub trait TryExtractInto<F> {
    /// Extract value of the given type from any type (e.g. ['Constant'], [`super::value::Value`])
    /// on which [`TryExtractFrom`] is implemented
    fn try_extract_into<T: TryExtractFrom<F>>(self) -> Result<T, TryExtractFromError>;
}

impl<F> TryExtractInto<F> for F {
    fn try_extract_into<T: TryExtractFrom<F>>(self) -> Result<T, TryExtractFromError> {
        T::try_extract_from(self)
    }
}

/// Underlying type is different from requested value type
#[derive(Error, PartialEq, Eq, Debug, Clone)]
#[error("Failed TryExtractFrom: {0}")]
pub struct TryExtractFromError(pub String);

/// Extract underlying value if type matches
pub trait TryExtractFrom<T>: Sized {
    /// Extract the value or return an error if type does not match
    fn try_extract_from(v: T) -> Result<Self, TryExtractFromError>;
}

impl<'ctx, T: TryExtractFrom<Literal<'ctx>> + 'ctx> TryExtractFrom<Constant<'ctx>> for T {
    fn try_extract_from(cv: Constant<'ctx>) -> Result<Self, TryExtractFromError> {
        T::try_extract_from(cv.v)
    }
}

impl<'ctx> TryExtractFrom<Literal<'ctx>> for () {
    fn try_extract_from(cv: Literal) -> Result<(), TryExtractFromError> {
        match cv {
            Literal::Unit => Ok(()),
            _ => Err(TryExtractFromError(format!(
                "expected Unit, found {:?}",
                cv
            ))),
        }
    }
}

impl<'ctx> TryExtractFrom<Literal<'ctx>> for bool {
    fn try_extract_from(cv: Literal) -> Result<bool, TryExtractFromError> {
        match cv {
            Literal::Boolean(v) => Ok(v),
            _ => Err(TryExtractFromError(format!(
                "expected bool, found {:?}",
                cv
            ))),
        }
    }
}

impl<'ctx> TryExtractFrom<Literal<'ctx>> for i8 {
    fn try_extract_from(cv: Literal) -> Result<i8, TryExtractFromError> {
        match cv {
            Literal::Byte(v) => Ok(v),
            _ => Err(TryExtractFromError(format!("expected i8, found {:?}", cv))),
        }
    }
}

impl<'ctx> TryExtractFrom<Literal<'ctx>> for i16 {
    fn try_extract_from(cv: Literal) -> Result<i16, TryExtractFromError> {
        match cv {
            Literal::Short(v) => Ok(v),
            _ => Err(TryExtractFromError(format!("expected i16, found {:?}", cv))),
        }
    }
}

impl<'ctx> TryExtractFrom<Literal<'ctx>> for i32 {
    fn try_extract_from(cv: Literal) -> Result<i32, TryExtractFromError> {
        match cv {
            Literal::Int(v) => Ok(v),
            _ => Err(TryExtractFromError(format!("expected i32, found {:?}", cv))),
        }
    }
}

impl<'ctx> TryExtractFrom<Literal<'ctx>> for i64 {
    fn try_extract_from(cv: Literal) -> Result<i64, TryExtractFromError> {
        match cv {
            Literal::Long(v) => Ok(v),
            _ => Err(TryExtractFromError(format!("expected i64, found {:?}", cv))),
        }
    }
}

impl<'ctx> TryExtractFrom<Literal<'ctx>> for EcPoint {
    fn try_extract_from(cv: Literal) -> Result<EcPoint, TryExtractFromError> {
        match cv {
            Literal::GroupElement(v) => Ok(*v),
            _ => Err(TryExtractFromError(format!(
                "expected EcPoint, found {:?}",
                cv
            ))),
        }
    }
}

impl<'ctx> TryExtractFrom<Literal<'ctx>> for SigmaProp {
    fn try_extract_from(cv: Literal) -> Result<SigmaProp, TryExtractFromError> {
        match cv {
            Literal::SigmaProp(v) => Ok(*v),
            _ => Err(TryExtractFromError(format!(
                "expected SigmaProp, found {:?}",
                cv
            ))),
        }
    }
}

impl<'ctx> TryExtractFrom<Literal<'ctx>> for Ref<'ctx, ErgoBox> {
    fn try_extract_from(c: Literal<'ctx>) -> Result<Self, TryExtractFromError> {
        match c {
            Literal::CBox(b) => Ok(b),
            _ => Err(TryExtractFromError(format!(
                "expected ErgoBox, found {:?}",
                c
            ))),
        }
    }
}

impl<'ctx> TryExtractFrom<Literal<'ctx>> for ErgoBox {
    fn try_extract_from(c: Literal) -> Result<Self, TryExtractFromError> {
        match c {
            Literal::CBox(b) => Ok((&*b).clone()),
            _ => Err(TryExtractFromError(format!(
                "expected ErgoBox, found {:?}",
                c
            ))),
        }
    }
}

impl<'ctx, T: TryExtractFrom<Literal<'ctx>> + StoreWrapped + 'ctx> TryExtractFrom<Literal<'ctx>>
    for Vec<T>
{
    fn try_extract_from(c: Literal<'ctx>) -> Result<Self, TryExtractFromError> {
        match c {
            Literal::Coll(coll) => match coll {
                CollKind::WrappedColl {
                    elem_tpe: _,
                    items: v,
                } => v.into_iter().map(T::try_extract_from).collect(),
                _ => Err(TryExtractFromError(format!(
                    "expected {:?}, found {:?}",
                    std::any::type_name::<Self>(),
                    coll
                ))),
            },
            _ => Err(TryExtractFromError(format!(
                "expected {:?}, found {:?}",
                std::any::type_name::<Self>(),
                c
            ))),
        }
    }
}

impl<'ctx> TryExtractFrom<Literal<'ctx>> for Vec<i8> {
    fn try_extract_from(v: Literal) -> Result<Self, TryExtractFromError> {
        match v {
            Literal::Coll(v) => match v {
                CollKind::NativeColl(NativeColl::CollByte(bs)) => Ok(bs),
                _ => Err(TryExtractFromError(format!(
                    "expected {:?}, found {:?}",
                    std::any::type_name::<Self>(),
                    v
                ))),
            },
            _ => Err(TryExtractFromError(format!(
                "expected {:?}, found {:?}",
                std::any::type_name::<Self>(),
                v
            ))),
        }
    }
}

impl<'ctx> TryExtractFrom<Literal<'ctx>> for Vec<u8> {
    fn try_extract_from(v: Literal) -> Result<Self, TryExtractFromError> {
        use sigma_util::FromVecI8;
        Vec::<i8>::try_extract_from(v).map(Vec::<u8>::from_vec_i8)
    }
}

impl<'ctx> TryExtractFrom<Literal<'ctx>> for Digest32 {
    fn try_extract_from(v: Literal) -> Result<Self, TryExtractFromError> {
        use sigma_util::FromVecI8;
        let bytes = Vec::<i8>::try_extract_from(v).map(Vec::<u8>::from_vec_i8)?;
        Digest32::try_from(bytes).map_err(|e| {
            TryExtractFromError(format!("failed to extract Digest32 with error: {:?}", e))
        })
    }
}

impl<'ctx> TryExtractFrom<Literal<'ctx>> for TokenId {
    fn try_extract_from(v: Literal) -> Result<Self, TryExtractFromError> {
        Digest32::try_extract_from(v).map(Into::into)
    }
}

impl<'ctx> TryExtractFrom<Literal<'ctx>> for Literal<'ctx> {
    fn try_extract_from(v: Literal<'ctx>) -> Result<Self, TryExtractFromError> {
        Ok(v)
    }
}

impl<'ctx> TryExtractFrom<Literal<'ctx>> for BigInt256 {
    fn try_extract_from(v: Literal) -> Result<Self, TryExtractFromError> {
        match v {
            Literal::BigInt(bi) => Ok(bi),
            _ => Err(TryExtractFromError(format!(
                "expected {:?}, found {:?}",
                std::any::type_name::<Self>(),
                v
            ))),
        }
    }
}

impl<'ctx> TryExtractFrom<Literal<'ctx>> for AvlTreeData {
    fn try_extract_from(v: Literal) -> Result<Self, TryExtractFromError> {
        match v {
            Literal::AvlTree(a) => Ok(*a),
            _ => Err(TryExtractFromError(format!(
                "expected {:?}, found {:?}",
                std::any::type_name::<Self>(),
                v
            ))),
        }
    }
}

impl<'ctx, T: TryExtractFrom<Literal<'ctx>>> TryExtractFrom<Literal<'ctx>> for Option<T> {
    fn try_extract_from(v: Literal<'ctx>) -> Result<Self, TryExtractFromError> {
        match v {
            Literal::Opt(opt) => opt.map(T::try_extract_from).transpose(),
            _ => Err(TryExtractFromError(format!(
                "expected Option, found {:?}",
                v
            ))),
        }
    }
}

#[impl_for_tuples(2, 4)]
impl<'ctx> TryExtractFrom<Literal<'ctx>> for Tuple {
    fn try_extract_from(v: Literal<'ctx>) -> Result<Self, TryExtractFromError> {
        match v {
            Literal::Tup(items) => {
                let mut iter = items.iter();
                Ok(for_tuples!( ( #(
                                Tuple::try_extract_from(
                                    iter
                                        .next()
                                        .cloned()
                                        .ok_or_else(|| TryExtractFromError("not enough items in STuple".to_string()))?
                                )?
                                ),* ) ))
            }
            _ => Err(TryExtractFromError(format!(
                "expected Context, found {:?}",
                v
            ))),
        }
    }
}

impl<'ctx> TryFrom<Literal<'ctx>> for ProveDlog {
    type Error = TryExtractFromError;
    fn try_from(cv: Literal) -> Result<Self, Self::Error> {
        match cv {
            Literal::SigmaProp(sp) => match sp.value() {
                SigmaBoolean::ProofOfKnowledge(SigmaProofOfKnowledgeTree::ProveDlog(
                    prove_dlog,
                )) => Ok(prove_dlog.clone()),
                _ => Err(TryExtractFromError(format!(
                    "expected ProveDlog, found {:?}",
                    sp
                ))),
            },
            _ => Err(TryExtractFromError(format!(
                "expected SigmaProp, found {:?}",
                cv
            ))),
        }
    }
}

impl<'ctx> Base16Str for &Constant<'ctx> {
    fn base16_str(&self) -> Result<String, SigmaSerializationError> {
        self.sigma_serialize_bytes()
            .map(|bytes| base16::encode_lower(&bytes))
    }
}

impl<'ctx> Base16Str for Constant<'ctx> {
    fn base16_str(&self) -> Result<String, SigmaSerializationError> {
        self.sigma_serialize_bytes()
            .map(|bytes| base16::encode_lower(&bytes))
    }
}

impl<'ctx> TryFrom<Base16DecodedBytes> for Constant<'ctx> {
    type Error = SigmaParsingError;

    fn try_from(value: Base16DecodedBytes) -> Result<Self, Self::Error> {
        Constant::sigma_parse_bytes(&value.0)
    }
}

#[cfg(feature = "arbitrary")]
#[allow(clippy::unwrap_used)]
#[allow(clippy::todo)]
/// Arbitrary impl
pub(crate) mod arbitrary {
    use std::convert::TryFrom;

    use super::*;
    use crate::mir::value::CollKind;
    use crate::types::stuple::STuple;
    use proptest::collection::vec;
    use proptest::prelude::*;

    extern crate derive_more;
    use derive_more::From;
    use derive_more::TryInto;

    fn primitive_type_value() -> BoxedStrategy<Constant<'static>> {
        prop_oneof![
            any::<bool>().prop_map_into(),
            any::<i8>().prop_map_into(),
            any::<i16>().prop_map_into(),
            any::<i32>().prop_map_into(),
            any::<i64>().prop_map_into(),
            any::<i64>().prop_map(|v| BigInt256::from(v).into()),
            any::<EcPoint>().prop_map_into(),
            any::<SigmaProp>().prop_map_into(),
            // although it's not strictly a primitive type, byte array is widely used as one
            vec(any::<i8>(), 0..100).prop_map_into(),
        ]
        .boxed()
    }

    fn coll_from_constant(c: Constant, length: usize) -> Constant {
        Constant {
            tpe: SType::SColl(Box::new(c.tpe.clone())),
            v: Literal::Coll(if c.tpe == SType::SByte {
                let mut values: Vec<i8> = Vec::with_capacity(length);
                let byte: i8 = c.v.try_extract_into().unwrap();
                for _ in 0..length {
                    values.push(byte);
                }
                CollKind::NativeColl(NativeColl::CollByte(values))
            } else {
                let mut values: Vec<Literal> = Vec::with_capacity(length);
                for _ in 0..length {
                    values.push(c.v.clone());
                }
                CollKind::WrappedColl {
                    elem_tpe: c.tpe,
                    items: values,
                }
            }),
        }
    }

    fn const_with_type(tpe: SType) -> BoxedStrategy<Constant<'static>> {
        match tpe {
            SType::SAny => any::<Constant>(),
            SType::SBoolean => any::<bool>().prop_map_into().boxed(),
            SType::SByte => any::<i8>().prop_map_into().boxed(),
            SType::SShort => any::<i16>().prop_map_into().boxed(),
            SType::SInt => any::<i32>().prop_map_into().boxed(),
            SType::SLong => any::<i64>().prop_map_into().boxed(),
            SType::SBigInt => any::<i64>().prop_map(|v| BigInt256::from(v).into()).boxed(),
            SType::SGroupElement => any::<EcPoint>().prop_map_into().boxed(),
            SType::SSigmaProp => any::<SigmaProp>().prop_map_into().boxed(),
            SType::SBox => any::<ErgoBox>().prop_map_into().boxed(),
            SType::SAvlTree => any::<AvlTreeData>().prop_map_into().boxed(),
            // SType::SOption(tpe) =>
            SType::SOption(tpe) => match *tpe {
                SType::SBoolean => any::<Option<bool>>().prop_map_into().boxed(),
                SType::SByte => any::<Option<i8>>().prop_map_into().boxed(),
                SType::SShort => any::<Option<i16>>().prop_map_into().boxed(),
                SType::SInt => any::<Option<i32>>().prop_map_into().boxed(),
                SType::SLong => any::<Option<i64>>().prop_map_into().boxed(),
                _ => todo!(),
            },
            SType::SColl(elem_tpe) => match *elem_tpe {
                SType::SBoolean => vec(any::<bool>(), 0..400).prop_map_into().boxed(),
                SType::SByte => vec(any::<u8>(), 0..400).prop_map_into().boxed(),
                SType::SShort => vec(any::<i16>(), 0..400).prop_map_into().boxed(),
                SType::SInt => vec(any::<i32>(), 0..400).prop_map_into().boxed(),
                SType::SLong => vec(any::<i64>(), 0..400).prop_map_into().boxed(),
                SType::SSigmaProp => vec(any::<SigmaProp>(), 0..3).prop_map_into().boxed(),
                _ => todo!(),
            },
            // SType::STuple(_) => {}
            _ => todo!("{0:?} not yet implemented", tpe),
        }
    }

    impl Default for ArbConstantParams {
        fn default() -> Self {
            ArbConstantParams::AnyWithDepth(1)
        }
    }

    /// Parameters for arbitrary Constant generation
    #[derive(PartialEq, Eq, Debug, Clone, From, TryInto)]
    pub enum ArbConstantParams {
        /// Constant of any type with a structrure of a given depth
        AnyWithDepth(u8),
        /// Constant of a given type
        Exact(SType),
    }

    impl Arbitrary for Constant<'static> {
        type Parameters = ArbConstantParams;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
            match args {
                ArbConstantParams::AnyWithDepth(depth) => {
                    prop_oneof![primitive_type_value().prop_recursive(
                        depth as u32,
                        16,
                        8,
                        |elem| {
                            prop_oneof![
                                // Coll[_]
                                elem.clone().prop_map(|c| coll_from_constant(c, 0)),
                                elem.clone().prop_map(|c| coll_from_constant(c, 1)),
                                elem.clone().prop_map(|c| coll_from_constant(c, 2)),
                                elem.clone().prop_map(|c| coll_from_constant(c, 10)),
                                // no Option[_] since it cannot be serialized (for now)
                                // // Some(v)
                                // elem.clone().prop_map(|c| Constant {
                                //     tpe: SType::SOption(Box::new(c.tpe)),
                                //     v: Value::Opt(Box::new(Some(c.v)))
                                // }),
                                // // None
                                // elem.prop_map(|c| Constant {
                                //     tpe: SType::SOption(Box::new(c.tpe)),
                                //     v: Value::Opt(Box::new(None))
                                // })

                                // Tuple
                                vec(elem, 2..=4).prop_map(|constants| Constant {
                                    tpe: SType::STuple(
                                        STuple::try_from(
                                            constants
                                                .clone()
                                                .into_iter()
                                                .map(|c| c.tpe)
                                                .collect::<Vec<SType>>()
                                        )
                                        .unwrap()
                                    ),
                                    v: Literal::Tup(
                                        constants
                                            .into_iter()
                                            .map(|c| c.v)
                                            .collect::<Vec<Literal>>()
                                            .try_into()
                                            .unwrap()
                                    )
                                }),
                            ]
                        }
                    )]
                    .boxed()
                }
                ArbConstantParams::Exact(tpe) => const_with_type(tpe),
            }
        }
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
#[allow(clippy::panic)]
pub mod tests {
    use super::*;
    use core::fmt;
    use proptest::prelude::*;

    fn test_constant_roundtrip<T>(v: T)
    where
        T: TryExtractInto<T>
            + TryExtractFrom<Literal<'static>>
            + Into<Constant<'static>>
            + fmt::Debug
            + Eq
            + Clone
            + 'static,
    {
        let constant: Constant = v.clone().into();
        let v_extracted: T = constant.try_extract_into::<T>().unwrap();
        assert_eq!(v, v_extracted);
    }

    #[test]
    fn unit_roundtrip() {
        test_constant_roundtrip(());
    }

    proptest! {

        #![proptest_config(ProptestConfig::with_cases(8))]

        #[test]
        fn bool_roundtrip(v in any::<bool>()) {
            test_constant_roundtrip(v);
        }

        #[test]
        fn i8_roundtrip(v in any::<i8>()) {
            test_constant_roundtrip(v);
        }

        #[test]
        fn i16_roundtrip(v in any::<i16>()) {
            test_constant_roundtrip(v);
        }

        #[test]
        fn i32_roundtrip(v in any::<i32>()) {
            test_constant_roundtrip(v);
        }

        #[test]
        fn i64_roundtrip(v in any::<i64>()) {
            test_constant_roundtrip(v);
        }

        #[test]
        fn bigint_roundtrip(raw in any::<i64>()) {
            let v = BigInt256::from(raw);
            test_constant_roundtrip(v);
        }

        #[test]
        fn group_element_roundtrip(v in any::<EcPoint>()) {
            test_constant_roundtrip(v);
        }

        #[test]
        fn sigma_prop_roundtrip(v in any::<SigmaProp>()) {
            test_constant_roundtrip(v);
        }

        #[test]
        fn vec_i8_roundtrip(v in any::<Vec<i8>>()) {
            test_constant_roundtrip(v);
        }

        #[test]
        fn vec_u8_roundtrip(v in any::<Vec<u8>>()) {
            // eprintln!("{:?}", Constant::from(v.clone()));
            test_constant_roundtrip(v);
        }

        #[test]
        fn token_id_roundtrip(v in any::<TokenId>()) {
            // eprintln!("{:?}", Constant::from(v.clone()));
            test_constant_roundtrip(v);
        }

        #[test]
        fn digest32_roundtrip(v in any::<Digest32>()) {
            test_constant_roundtrip(v);
        }


        #[test]
        fn vec_i16_roundtrip(v in any::<Vec<i16>>()) {
            test_constant_roundtrip(v);
        }

        #[test]
        fn vec_i32_roundtrip(v in any::<Vec<i32>>()) {
            test_constant_roundtrip(v);
        }

        #[test]
        fn vec_i64_roundtrip(v in any::<Vec<i64>>()) {
            // eprintln!("{:?}", Constant::from(v.clone()));
            test_constant_roundtrip(v);
        }

        #[test]
        fn vec_bigint_roundtrip(raw in any::<Vec<i64>>()) {
            let v: Vec<BigInt256> = raw.into_iter().map(BigInt256::from).collect();
            // eprintln!("{:?}", Constant::from(v.clone()));
            test_constant_roundtrip(v);
        }

        #[test]
        fn vec_option_bigint_roundtrip(raw in any::<Vec<i64>>()) {
            let v: Vec<Option<BigInt256>> = raw.into_iter().map(|i| Some(BigInt256::from(i))).collect();
            // eprintln!("{:?}", Constant::from(v.clone()));
            test_constant_roundtrip(v);
        }

        #[test]
        fn vec_sigmaprop_roundtrip(v in any::<Vec<SigmaProp>>()) {
            test_constant_roundtrip(v);
        }

        #[test]
        fn option_primitive_type_roundtrip(v in any::<Option<i64>>()) {
            test_constant_roundtrip(v);
        }

        #[test]
        fn option_nested_vector_type_roundtrip(v in any::<Option<Vec<(i64, bool)>>>()) {
            // eprintln!("{:?}", Constant::from(v.clone()));
            test_constant_roundtrip(v);
        }

        #[test]
        fn option_nested_tuple_type_roundtrip(v in any::<Option<(i64, bool)>>()) {
            test_constant_roundtrip(v);
        }


        #[test]
        fn tuple_primitive_types_roundtrip(v in any::<(i64, bool)>()) {
            // let constant: Constant = v.into();
            // eprintln!("{:?}", constant);
            test_constant_roundtrip(v);
        }

        #[test]
        fn tuple_nested_types_roundtrip(v in any::<(Option<i64>, Vec<SigmaProp>)>()) {
            test_constant_roundtrip(v);
        }

    }
}
