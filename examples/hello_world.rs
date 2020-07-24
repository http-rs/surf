#[async_std::main]
async fn main() -> Result<(), http_types::Error> {
    femme::start(log::LevelFilter::Info)?;

    let uri = "https://httpbin.org/get";
    let string: String = surf::get(uri).recv_string().await?;
    println!("{}", string);
    Ok(())
}
