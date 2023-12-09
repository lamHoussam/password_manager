extern crate steganography;
use clap::Parser as ClapParser;

use std::io::Read;
use serde_json::{Value, Map};
use std::fs::File;

#[derive(ClapParser, Debug)]
#[command()]
struct Args {
    //// Data file path for global variables
    // #[arg(short, long)]
    // data: Option<String>,
    //// Sample markdown file
    // #[arg(short, long)]
    // sample: String,
    //// Output markdown file
    // #[arg(short, long)]
    // output: String,

    /// Original picture path
    # [arg(short, long)]
    picture_path: String,

    /// Output picture path
    # [arg(short, long)]
    output: String,

    /// Word to encode
    # [arg(short, long)]
    word: String,
}

fn encode_password(msg: &str, original_image_path: &str, output_image_path: &str) {
    let binding = msg.to_string();
    let msg_bytes = steganography::util::str_to_bytes(&binding);
    let destination_image = steganography::util::file_as_dynamic_image(original_image_path.to_string());

    let enc = steganography::encoder::Encoder::new(msg_bytes, destination_image);

    let result = enc.encode_alpha();
    steganography::util::save_image_buffer(result, output_image_path.to_string());

    println!("Text encoded successfully!");
}

fn decode_password(output_image_path: &str) -> String {
    let encoded_image = steganography::util::file_as_image_buffer(output_image_path.to_string());
    let dec = steganography::decoder::Decoder::new(encoded_image);
    //Decode the image by reading the alpha channel
    let out_buffer = dec.decode_alpha();
    //If there is no alpha, it's set to 255 by default so we filter those out
    let clean_buffer: Vec<u8> = out_buffer.into_iter()
        .filter(|b| {
            *b != 0xff_u8
        })
        .collect();
    
    let message = steganography::util::bytes_to_str(clean_buffer.as_slice());
    return message.to_string();
}

fn add_new_platform(platform: &str, password: &str, mut settings_map: &Map<String, Value>) {
//    settings_map
}


fn main() {
    let settings_file_path = "settings.json";
    let mut file_content = String::new();
    let mut file = File::open(settings_file_path).expect("Failed to open settings file");

    file.read_to_string(&mut file_content).expect("Failed to read settings file");
    let settings: Value = serde_json::from_str(&file_content).expect("Failed to parse JSON");

    println!("{:#?}", settings);

    let psswrd = settings["password"].as_str().unwrap();
    let pictures_paths = settings["pictures_paths"].as_array().unwrap();

    println!("pass: {}", psswrd);


    let platform = "Lamlih";

    for value in pictures_paths {
        let map = value.as_object().unwrap();
        let name = map["name"].as_str().unwrap();
        
        if name == platform {
            let path = map["path"].as_str().unwrap();
            println!("{}", path);
        }
    }



    let original_image_path = "images/rust-social.jpg";
    let output_image_path = "images/rust-social.png";

    let msg = "Houssam";

    encode_password(msg, original_image_path, output_image_path);
    let decoded_msg = decode_password(output_image_path);

    println!("Decoded message : {:?}", decoded_msg);
}

