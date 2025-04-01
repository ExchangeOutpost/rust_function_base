mod exchange_outpost;
use crate::exchange_outpost::FinData;
use extism_pdk::{encoding, plugin_fn, FnResult, Json, ToBytes};
use serde::Serialize;

#[derive(Serialize, ToBytes)]
#[encoding(Json)]
pub struct Output {}

#[plugin_fn]
pub fn run(fin_data: FinData<f64>) -> FnResult<Output> {
    Ok(Output {})
}