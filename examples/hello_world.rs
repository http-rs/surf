type Exception = Box<dyn std::error::Error + Send + Sync + 'static>;

#[runtime::main]
async fn main() -> Result<(), Exception> {
    femme::start(log::LevelFilter::Info)?;

    let uri = "https://httpbin.org/get";
    let string = surf::get(uri).recv_string().await?;
    println!("{}", string);

    Ok(())
}
