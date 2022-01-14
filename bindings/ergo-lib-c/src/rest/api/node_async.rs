use std::time::Duration;

use ergo_lib_c_core::rest::api::node_async::rest_api_node_get_info_async;
use ergo_lib_c_core::rest::api::node_async::CompletedCallback;
use ergo_lib_c_core::rest::api::runtime::RestApiRuntimePtr;
use ergo_lib_c_core::rest::node_conf::NodeConfPtr;
use ergo_lib_c_core::rest::node_info::NodeInfo;
use ergo_lib_c_core::Error;
use ergo_lib_c_core::ErrorPtr;

#[no_mangle]
pub unsafe extern "C" fn ergo_lib_rest_api_node_get_info_async(
    runtime_ptr: RestApiRuntimePtr,
    node_conf_ptr: NodeConfPtr,
    timeout_sec: u32,
    callback: CompletedCallback<NodeInfo>,
) -> ErrorPtr {
    let res = rest_api_node_get_info_async(
        runtime_ptr,
        node_conf_ptr,
        Duration::from_secs(timeout_sec as u64),
        callback,
    );
    Error::c_api_from(res)
}
