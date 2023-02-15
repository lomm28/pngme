use crate::{Result};
use crate::args::{Encode, DecodeRemove, Print};
use crate::png::Png;
use crate::chunk_type::ChunkType;
use crate::chunk::Chunk;

use std::str::FromStr;
use std::fs::{File, remove_file};
use std::io::prelude::*;
use std::path::{PathBuf, Path};

fn rewrite_file(file_path: &str, png_file: Png) -> Result<&str> {
    let file_name = Path::new(file_path).file_name().unwrap().to_str().unwrap();

    remove_file(file_name)?;
    let mut file = File::create(&file_name)?;
    file.write_all(&png_file.as_bytes())?;

    Ok(file_name)
}

pub fn encode(name: Encode) -> Result<()> {
    let mut png_file = Png::from_file(&name.path).unwrap();
    let chunk_type = ChunkType::from_str(&name.chunk_type).unwrap();

    if !chunk_type.is_valid() {
        Err("chunk type is not valid")?;
    }

    let msg_as_bytes = name.message.as_bytes().to_vec();
    let chunk = Chunk::new(chunk_type, msg_as_bytes);

    png_file.append_chunk(chunk);
    let file_name = rewrite_file(&name.path, png_file);

    println!("Message was encoded in {}", file_name.unwrap());

    Ok(())
}

pub fn decode(name: DecodeRemove) -> Result<()> {
    let mut png_file = Png::from_file(&name.path).unwrap();
    let chunk = png_file.chunk_by_type(&name.chunk_type);

    match chunk {
        None => {
            println!("no chunk with chunk type `{}` found", &name.chunk_type); 
        },
        Some(chnk) => {
            let encoded_message = chnk.data_as_string().unwrap();

            println!("encoded message: {}", encoded_message);
        },
    }

    Ok(())
}

pub fn remove(name: DecodeRemove) -> Result<()> {
    let mut png_file = Png::from_file(&name.path).unwrap();
    let chunk = png_file.chunk_by_type(&name.chunk_type);

    match chunk {
        None => {
            println!("no chunk with chunk type `{}` found", &name.chunk_type); 
        },
        Some(chnk) => {
            png_file.remove_chunk(&name.chunk_type);
            rewrite_file(&name.path, png_file);

            println!("chunk was removed");
        },
    }

    Ok(())
}

pub fn print(name: Print) -> Result<()> {
    let png_file = Png::from_file(&name.path).unwrap();
    println!("{}", png_file);

    Ok(())
}