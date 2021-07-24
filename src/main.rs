mod json;
mod log;
mod logfmt;

use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    let formats = vec![json::JsonFormat::new_box(), logfmt::LogFormat::new_box()];
    let mut handler = log::Handler::new(formats);

    let mut stdout = BufWriter::new(io::stdout());
    let mut lines = BufReader::new(io::stdin()).lines();
    while let Some(line) = lines.next_line().await? {
        let out = handler.handle(&line);
        stdout.write(out.as_bytes()).await?;
        stdout.flush().await?;
    }
    Ok(())
}
