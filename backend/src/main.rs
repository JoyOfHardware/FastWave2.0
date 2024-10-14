use moon::*;

async fn frontend() -> Frontend {
    Frontend::new()
        .title("FastWave")
        .append_to_head(include_str!("../favicon.html")) // realfavicongenerator.net
        .append_to_head(concat!("<style>", include_str!("../style.css"), "</style>"))
        .append_to_head(concat!(
            "<script>",
            include_str!("../globals.js"),
            "</script>"
        ))
        .append_to_head(concat!(
            "<script type=\"module\">",
            include_str!("../index.js"),
            "</script>"
        ))
}

async fn up_msg_handler(_: UpMsgRequest<()>) {}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_| {}).await
}
