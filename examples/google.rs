fn main() {
    futures::block_on({
        let text = surf::get("https://google.com").send_text().await?;
        dbg!(text);
    })
}
