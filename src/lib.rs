mod exchange_outpost;
use crate::exchange_outpost::FinData;
use extism_pdk::{FnResult, Json, ToBytes, encoding, plugin_fn};
use serde::Serialize;

#[derive(Serialize, ToBytes)]
#[encoding(Json)]
pub struct Output {}

#[plugin_fn]
pub fn run(fin_data: FinData) -> FnResult<Output> {
    Ok(Output {})
}
