use crate::chain::parameters::{Parameter, Parameters};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct ParametersJson {
    #[cfg_attr(feature = "json", serde(rename = "blockVersion"))]
    pub block_version: i32,
    #[cfg_attr(feature = "json", serde(rename = "storageFeeFactor"))]
    pub storage_fee_factor: i32,
    #[cfg_attr(feature = "json", serde(rename = "minValuePerByte"))]
    pub min_value_per_byte: i32,
    #[cfg_attr(feature = "json", serde(rename = "maxBlockSize"))]
    pub max_block_size: i32,
    #[cfg_attr(feature = "json", serde(rename = "maxBlockCost"))]
    pub max_block_cost: i32,
    #[cfg_attr(feature = "json", serde(rename = "tokenAccessCost"))]
    pub token_access_cost: i32,
    #[cfg_attr(feature = "json", serde(rename = "inputCost"))]
    pub input_cost: i32,
    #[cfg_attr(feature = "json", serde(rename = "dataInputCost"))]
    pub data_input_cost: i32,
    #[cfg_attr(feature = "json", serde(rename = "outputCost"))]
    pub output_cost: i32,
}

impl From<ParametersJson> for Parameters {
    fn from(v: ParametersJson) -> Self {
        let mut parameters_table = HashMap::new();
        parameters_table.insert(Parameter::StorageFeeFactor, v.storage_fee_factor);
        parameters_table.insert(Parameter::MinValuePerByte, v.min_value_per_byte);
        parameters_table.insert(Parameter::TokenAccessCost, v.token_access_cost);
        parameters_table.insert(Parameter::InputCost, v.input_cost);
        parameters_table.insert(Parameter::DataInputCost, v.data_input_cost);
        parameters_table.insert(Parameter::OutputCost, v.output_cost);
        parameters_table.insert(Parameter::MaxBlockSize, v.max_block_size);
        parameters_table.insert(Parameter::BlockVersion, v.block_version);
        parameters_table.insert(Parameter::MaxBlockCost, v.max_block_cost);
        Self { parameters_table }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use crate::chain::parameters::Parameters;

    #[test]
    fn parse_parameters() {
        let node_info_json = r#"
           {
               "outputCost": 194,
               "tokenAccessCost": 100,
               "maxBlockCost": 8001091,
               "height": 1259520,
               "maxBlockSize": 1271009,
               "dataInputCost": 100,
               "blockVersion": 3,
               "inputCost": 2407,
               "storageFeeFactor": 1250000,
               "minValuePerByte": 360
           }
        "#;
        let params: Parameters = serde_json::from_str(node_info_json).unwrap();
        assert_eq!(params.output_cost(), 194);
        assert_eq!(params.token_access_cost(), 100);
        assert_eq!(params.max_block_cost(), 8001091);
        assert_eq!(params.max_block_size(), 1271009);
        assert_eq!(params.data_input_cost(), 100);
        assert_eq!(params.block_version(), 3);
        assert_eq!(params.input_cost(), 2407);
        assert_eq!(params.storage_fee_factor(), 1250000);
        assert_eq!(params.min_value_per_byte(), 360);
    }
}
