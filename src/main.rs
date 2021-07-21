use std::process;

use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::signal;

mod json;
mod log;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tokio::spawn(async {
        signal::ctrl_c().await.unwrap();
        process::exit(0);
    });

    let mut formats = Vec::new();
    formats.push(json::JsonFormat::new());
    let mut handler = log::Handler::new(formats);

    let mut stdout = BufWriter::new(io::stdout());
    let mut lines = BufReader::new(io::stdin()).lines();
    while let Some(line) = lines.next_line().await.unwrap() {
        let out = handler.handle(&line);
        stdout.write(out.as_bytes()).await.unwrap();
        stdout.flush().await.unwrap();
    }
}
