use lambda::lambda;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod mos;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Deserialize)]
struct WeatherRequestEvent {
    icao: String,
}

#[derive(Serialize)]
struct WeatherRequestOutput {
    message: String,
}

#[lambda]
#[tokio::main]
async fn main(e: WeatherRequestEvent) -> Result<Value, Error> {
    let mos = match mos::get(&e.icao) {
        Ok(mos) => mos,
        Err(err) => return Err(Box::new(err)),
    };

    Ok(Value::String(mos.raw))
}

// fn main() {
//     let mos = match mos::get("KFIT") {
//         Ok(mos) => mos,
//         Err(err) => panic!(err),
//     };

//     println!("{:?}", mos);
// }
