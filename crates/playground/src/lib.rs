use playground_api::Error;
use playground_api::blocking::Client;
use playground_api::endpoints::{
    Channel, CrateType, Edition, ExecuteRequest, ExecuteResponse, Mode,
};

pub fn run_code(code: String) -> Result<ExecuteResponse, Error> {
    let client = Client::default();

    let req = ExecuteRequest::new(
        Channel::Stable,
        Mode::Release,
        Edition::Edition2021,
        CrateType::Binary,
        false,
        false,
        code,
    );

    let res = client.execute(&req)?;
    Ok(res)
}
