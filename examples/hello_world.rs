type Exception = Box<dyn std::error::Error + Send + Sync + 'static>;
use async_std::task;

fn main()  {
    femme::start(log::LevelFilter::Info);

    task::block_on(async {
        let uri = "https://httpbin.org/get";
        let string = surf::get(uri).recv_string().await;
        println!("{}", string.unwrap());
    })
}
