use clap::Parser;
use wapc::WapcHost;
use wapc_codec::messagepack::{deserialize, serialize};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    bin: String,
    #[clap(short, long, value_parser)]
    arg: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let request_url = format!("http://localhost:3000/{}.wasm", args.bin);
    println!("Making request to {} for binary {}", request_url, args.bin);
    let raw_resp = reqwest::get(&request_url).await?;
    let buf = raw_resp.bytes().await?;

    let engine = wasmtime_provider::WasmtimeEngineProvider::new(&buf, None)?;

    let guest = WapcHost::new(
        Box::new(engine),
        Some(Box::new(move |_a, _b, _c, _d, _e| Ok(vec![]))),
    )?;

    println!("Finished initializing");

    let callresult = guest.call("sayHello", &serialize(args.arg)?)?;
    let result: String = deserialize(&callresult)?;
    println!("The result of the wasm call is {}", result);
    Ok(())
}
