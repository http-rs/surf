fn main() {
    futures::block_on({
        let res = surf::get("https://google.com").send_text().await?;
        dbg!(res.body);
    })
}
