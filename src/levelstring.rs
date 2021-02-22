use super::triangle::Vector;
use std::collections::HashMap;

pub struct GDObject {
    props: HashMap<u8, String>,
}

impl GDObject {
    pub fn new(id: &str) -> Self {
        let mut props = HashMap::<u8, String>::new();
        props.insert(1, id.to_string());
        GDObject { props }
    }

    pub fn set_prop(&mut self, key: u8, value: &str) {
        (*self).props.insert(key, value.to_string());
    }

    pub fn set_pos(&mut self, pos: Vector) {
        self.set_prop(2, &(pos.x * 30.0).to_string());
        self.set_prop(3, &(pos.y * 30.0).to_string());
    }

    fn get_obj_string(&self) -> String {
        let mut obj_str = String::new();
        for (key, val) in self.props.iter() {
            obj_str += &format!("{},{},", key, val);
        }
        obj_str += ";";
        obj_str
    }
}

pub fn create_level_string(objects: Vec<GDObject>) -> String {
    let mut levelstring = String::from(";");
    for obj in objects.iter() {
        levelstring += &obj.get_obj_string();
    }
    levelstring
}

use quick_xml::Writer;
use std::fs;
use std::io::Cursor;
use std::path::PathBuf;

use base64;
use libflate::{gzip, zlib};
//use std::io::Read;

fn xor(data: Vec<u8>, key: u8) -> Vec<u8> {
    let mut new_data = Vec::new();

    for b in data {
        //let new_byte = u64::from(b).pow(key);
        new_data.push(b ^ key)
    }
    new_data
}

use quick_xml::events::{BytesText, Event};
use quick_xml::Reader;
use std::io::BufReader;

pub fn get_local_levels_path() -> Result<PathBuf, String> {
    Ok(PathBuf::from(match std::env::var("localappdata") {
        Ok(path) => path,
        Err(e) => return Err(e.to_string()),
    })
    .join("GeometryDash/CCLocalLevels.dat"))
}

pub fn encrypt_level_string(ls: String) {
    let path = get_local_levels_path().unwrap();
    let file_content = fs::read_to_string(path.clone()).unwrap();

    //decrypting the savefile
    let xor_encrypted = xor(file_content.as_bytes().to_vec(), 11);
    let replaced = String::from_utf8(xor_encrypted)
        .unwrap()
        .replace("-", "+")
        .replace("_", "/")
        .replace("\0", "");
    let b64 = base64::decode(replaced.as_str()).unwrap();
    let decoder = gzip::Decoder::new(&b64[..]).unwrap();

    //encrypt the ls
    //encrypting level string
    /*def encrypt(dls):
    fin = gzip.compress(dls)
    fin = base64.b64encode(fin)
    fin = fin.decode("utf-8").replace('+', '-').replace('/', '_')
    fin = 'H4sIAAAAAAAAC' + fin[13:]
    return(fin)*/

    //setting level string

    let mut reader = Reader::from_reader(BufReader::new(decoder));
    reader.trim_text(true);

    let mut writer = Writer::new(Cursor::new(Vec::new()));

    let mut buf = Vec::new();

    let mut k4_detected = false;
    let mut done = false;
    let mut k2_detected = false;

    //println!("{}", old_ls);

    loop {
        match reader.read_event(&mut buf) {
            // unescape and decode the text event using the reader encoding
            Ok(Event::Text(e)) => {
                let text = e.unescape_and_decode(&reader).unwrap();
                if k4_detected {
                    let encrypted_ls: String = {
                        let mut ls_encoder = gzip::Encoder::new(Vec::new()).unwrap();
                        ls_encoder.write_all(&ls.as_bytes()).unwrap();
                        let b64_encrypted =
                            base64::encode(&ls_encoder.finish().into_result().unwrap());
                        let fin = b64_encrypted.replace("+", "-").replace("/", "_");
                        "H4sIAAAAAAAAC".to_string() + &fin[13..]
                    };

                    assert!(writer
                        .write_event(Event::Text(BytesText::from_plain_str(&encrypted_ls)))
                        .is_ok());
                    done = true;
                    k4_detected = false;
                } else {
                    assert!(writer.write_event(Event::Text(e)).is_ok())
                }

                if k2_detected {
                    println!("Writing to level: {}", text);
                    k2_detected = false;
                }

                if !done && text == "k4" {
                    k4_detected = true
                }

                if !done && text == "k2" {
                    k2_detected = true
                }
            }
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(e) => assert!(writer.write_event(e).is_ok()),
        }

        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }
    let bytes = writer.into_inner().into_inner();
    //encrypt level save
    use std::io::Write;

    let mut encoder = zlib::Encoder::new(Vec::new()).unwrap();
    encoder.write_all(&bytes).unwrap();
    let compressed = encoder.finish().into_result().unwrap();
    use crc32fast::Hasher;

    let mut hasher = Hasher::new();
    hasher.update(&bytes);
    let checksum = hasher.finalize();

    let data_size = bytes.len() as u32;

    let mut with_signature = b"\x1f\x8b\x08\x00\x00\x00\x00\x00\x00\x0b".to_vec();
    with_signature.extend(&compressed[2..compressed.len() - 4]);
    with_signature.extend(checksum.to_le_bytes().to_vec());
    with_signature.extend(data_size.to_le_bytes().to_vec());

    let encoded = base64::encode(&with_signature)
        .replace("+", "-")
        .replace("/", "_")
        .as_bytes()
        .to_vec();

    let fin = xor(encoded, 11);
    assert!(fs::write(path, fin).is_ok());
}
