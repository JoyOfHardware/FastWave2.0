use moon::*;

async fn frontend() -> Frontend {
    Frontend::new().title("FastWave").append_to_head(concat!(
        "<style>",
        include_str!("../style.css"),
        "</style>"
    ))
}

async fn up_msg_handler(_: UpMsgRequest<()>) {}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_| {}).await
}
