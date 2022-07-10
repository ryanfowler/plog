mod json;
mod log;
mod logfmt;

use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    let formats: Vec<Box<dyn log::Format>> = vec![
        Box::new(json::JsonFormat {}),
        Box::new(logfmt::LogFormat {}),
    ];
    let mut handler = log::Handler::new(formats);

    let mut stdout = io::stdout();
    let mut stdin = BufReader::new(io::stdin());

    let mut buf = String::new();
    while stdin.read_line(&mut buf).await? > 0 {
        let out = handler.handle(buf.trim_end());
        stdout.write_all(out.as_bytes()).await?;
        buf.clear();
    }
    Ok(())
}
