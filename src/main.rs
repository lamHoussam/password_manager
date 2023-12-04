extern crate steganography;


pub fn encode_password(msg: &str, original_image_path: &str, output_image_path: &str) {
    let binding = msg.to_string();
    let msg_bytes = steganography::util::str_to_bytes(&binding);
    let destination_image = steganography::util::file_as_dynamic_image(original_image_path.to_string());

    let enc = steganography::encoder::Encoder::new(msg_bytes, destination_image);

    let result = enc.encode_alpha();
    steganography::util::save_image_buffer(result, output_image_path.to_string());

    println!("Text encoded successfully!");
}

pub fn decode_password(output_image_path: &str) {
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
    //Convert those bytes into a string we can read
    let message = steganography::util::bytes_to_str(clean_buffer.as_slice());
    //Print it out!
    println!("{:?}", message);

}

fn main() {
    let original_image_path = "images/rust-social.jpg";
    let output_image_path = "images/rust-social.png";

    encode_password("Houssam", original_image_path, output_image_path);

    decode_password(output_image_path);
}
