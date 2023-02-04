use crate::{Result};
use crate::args::{Encode, Decode};
use crate::png::Png;
use crate::chunk_type::ChunkType;
use crate::chunk::Chunk;

use std::str::FromStr;
use std::fs::File;
use std::io::prelude::*;

pub fn encode(name: Encode) -> Result<()> {
    let mut png_file = Png::from_file(&name.path).unwrap();
    let chunk_type = ChunkType::from_str(&name.chunk_type).unwrap();

    if !chunk_type.is_valid() {
        Err("chunk type is not valid")?;
    }

    let msg_as_bytes = name.message.as_bytes().to_vec();
    let chunk = Chunk::new(chunk_type, msg_as_bytes);

    png_file.append_chunk(chunk);

    let mut file = File::create("encoded.png")?;
    file.write_all(&png_file.as_bytes())?;

    println!("Message was encoded in encoded.png");

    Ok(())
}

pub fn decode(name: Decode) -> Result<()> {
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