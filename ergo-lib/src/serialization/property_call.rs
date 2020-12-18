use std::io::Error;

use crate::ast::expr::Expr;
use crate::ast::property_call::PropertyCall;
use crate::types::smethod::MethodId;
use crate::types::smethod::SMethod;
use crate::types::stype_companion::TypeId;

use super::sigma_byte_reader::SigmaByteRead;
use super::sigma_byte_writer::SigmaByteWrite;
use super::SerializationError;
use super::SigmaSerializable;

impl SigmaSerializable for PropertyCall {
    fn sigma_serialize<W: SigmaByteWrite>(&self, w: &mut W) -> Result<(), Error> {
        self.method.obj_type.type_id().sigma_serialize(w)?;
        self.method.method_id().sigma_serialize(w)?;
        self.obj.sigma_serialize(w)?;
        Ok(())
    }

    fn sigma_parse<R: SigmaByteRead>(r: &mut R) -> Result<Self, SerializationError> {
        let type_id = TypeId::sigma_parse(r)?;
        let method_id = MethodId::sigma_parse(r)?;
        let obj = Expr::sigma_parse(r)?;
        Ok(PropertyCall {
            obj,
            method: SMethod::from_ids(type_id, method_id),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::expr::Expr;
    use crate::ast::property_call::PropertyCall;
    use crate::serialization::sigma_serialize_roundtrip;
    use crate::types::scontext;

    #[test]
    fn ser_roundtrip_property() {
        let mc = PropertyCall {
            obj: Expr::Context,
            method: scontext::DATA_INPUTS_PROPERTY.clone(),
        }
        .into();
        let expr = Expr::ProperyCall(mc);
        assert_eq![sigma_serialize_roundtrip(&expr), expr];
    }
}
