use std::fs::File;

use crate::pipeline::Pipeline;

pub fn run() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).expect("usage: tutel-play <input>");
    let mut file = File::open(path)?;
    let mut pipeline = Pipeline::new(&mut file)?;
    pipeline.run(&mut file)?;
    Ok(())
}
