use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    str::FromStr,
};

use args::Args;
use chunk::Chunk;
use chunk_type::ChunkType;
use clap::Parser;
use png::Png;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

fn main() {
    let args = Args::parse();

    match args.command {
        commands::Commands::Encode {
            file_path,
            chunk_type,
            message,
        } => {
            encode(file_path, chunk_type, message).unwrap();
        }
        commands::Commands::Decode {
            file_path,
            chunk_type,
        } => decode(file_path, chunk_type).unwrap(),
        commands::Commands::Remove {
            file_path,
            chunk_type,
        } => remove(file_path, chunk_type).unwrap(),
        commands::Commands::Print { file_path } => print_png(file_path).unwrap(),
    }
}

fn print_png(file_path: String) -> Result<(), String> {
    let png_file = fs::read(&file_path).unwrap();
    let png_file = Png::try_from(png_file.as_slice())?;
    print!("{}", png_file);
    Ok(())
}

fn remove(file_path: String, chunk_type: String) -> Result<(), String> {
    let png_file = fs::read(&file_path).unwrap();
    let mut png_file = Png::try_from(png_file.as_slice())?;

    let chunk = png_file.remove_first_chunk(&chunk_type)?;

    let original_file_path = Path::new(&file_path);

    let new_filename = match original_file_path.file_stem() {
        Some(stem) => format!("{}_with_secret_removed", stem.to_string_lossy()),
        None => "image_with_secret_removed".to_string(),
    };

    let new_path = original_file_path
        .parent()
        .unwrap_or(Path::new("."))
        .join(format!("{}.png", new_filename));

    let mut file = File::create(&new_path).unwrap();
    file.write_all(&png_file.as_bytes()).unwrap();

    match chunk.data_as_string() {
        Ok(msg) => {
            println!("Removed message: {:?}", msg);
        }
        Err(_) => {}
    }

    println!("Png with secret removed crated at: {:?}", new_path);
    Ok(())
}

fn decode(file_path: String, chunk_type: String) -> Result<(), String> {
    let png_file = fs::read(&file_path).unwrap();
    let png_file = Png::try_from(png_file.as_slice())?;

    let chunk = png_file
        .chunk_by_type(&chunk_type)
        .ok_or_else(|| format!("There is no chunk type {} in the PNG", chunk_type))?;

    let msg = chunk.data_as_string().map_err(|e| e.to_string())?;

    println!("{}", msg);
    Ok(())
}

fn encode(file_path: String, chunk_type: String, message: String) -> Result<(), String> {
    let png_file = fs::read(&file_path).unwrap();
    let mut png_file = Png::try_from(png_file.as_slice()).unwrap();
    let chunk_type = ChunkType::from_str(&chunk_type).unwrap();
    let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());

    png_file.append_chunk(chunk);

    let original_file_path = Path::new(&file_path);

    let new_filename = match original_file_path.file_stem() {
        Some(stem) => format!("{}_with_secret", stem.to_string_lossy()),
        None => "image_with_secret".to_string(),
    };

    let new_path = original_file_path
        .parent()
        .unwrap_or(Path::new("."))
        .join(format!("{}.png", new_filename));

    let mut file = File::create(&new_path).unwrap();
    file.write_all(&png_file.as_bytes()).unwrap();

    println!("Png with secret crated at: {:?}", new_path);

    Ok(())
}
