use std::fs::{self, File};
use std::io;
use std::path::Path;
use std::time::Duration;

use anyhow::Result;
use clap::Parser;
use http::StatusCode;

#[derive(Parser, Debug)]
struct Args {
    /// ダウンロード開始回
    #[clap(long, short)]
    start: Option<u32>,
    /// ダウンロード終了回
    #[clap(long, short)]
    end: Option<u32>,
    /// 出力先ディレクトリ
    #[clap(long, short)]
    output: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let mut times = args.start.unwrap_or(1);
    let end = args.end.unwrap_or(0);
    let name = args.output.unwrap_or_else(|| "output".to_string());
    let output_dir = Path::new(&name);

    // ディレクトリ作成
    if !output_dir.exists() {
        fs::create_dir(output_dir)?;
    }

    while times <= end {
        let url = format!("https://omocoro.heteml.net/radio/tokumei/{:0>3}.mp3", times);

        let filename = format!("{:0>3}.mp3", times);
        let dst = output_dir.join(filename);
        let res = reqwest::get(&url).await?;
        if res.status() != StatusCode::OK {
            println!("CANNOT DOWNLOAD: {}", url);
        } else {
            println!("DOWNLOAD: {} to {}", url, dst.display());
            let bytes = res.bytes().await?;
            let mut output = File::create(dst)?;
            io::copy(&mut bytes.as_ref(), &mut output)?;
        }

        times += 1;
        tokio::time::sleep(Duration::new(1, 0)).await;
    }

    Ok(())
}
