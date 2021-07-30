mod json;
mod log;
mod logfmt;

use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    let formats = vec![json::JsonFormat::new_box(), logfmt::LogFormat::new_box()];
    let mut handler = log::Handler::new(formats);

    let mut stdout = BufWriter::new(io::stdout());
    let mut stdin = BufReader::new(io::stdin());

    let mut buf = String::new();
    while stdin.read_line(&mut buf).await? > 0 {
        let out = handler.handle(buf.trim_end());
        stdout.write_all(out.as_bytes()).await?;
        stdout.flush().await?;
        buf.clear();
    }
    Ok(())
}
