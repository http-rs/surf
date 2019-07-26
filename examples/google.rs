#![feature(async_await)]
type Exception = Box<dyn std::error::Error + Send + Sync + 'static>;

#[runtime::main]
async fn main() -> Result<(), Exception> {
    femme::start(log::LevelFilter::Info)?;

    let uri = "https://google.com";
    let string = surf::get(uri).recv_string().await?;
    println!("{}", string);

    Ok(())
}
