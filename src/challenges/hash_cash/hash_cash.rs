use crate::challenges::challenge_interface::{ChallengeTrait};
use crate::challenges::hash_cash::{MD5HashCashInput, MD5HashCashOutput};

#[derive(Clone)]
struct Md5Buffer {
    a: u32,
    b: u32,
    c: u32,
    d: u32
}
#[derive(Clone)]
struct AuxInput {
    x: u32,
    y: u32,
    z: u32
}


fn bin_to_dec(binary: &[u8]) -> isize {
    let mut bin_str = "".to_string();
    let le_binary: Vec<&u8> = binary.into_iter().rev().collect();
    for bit in le_binary {
        bin_str += &bit.to_string();
    }
    let decimal = isize::from_str_radix(&bin_str.to_string(), 2).unwrap();
    decimal
}

fn dec_to_bin(dec: u8, e: &str, len: u8) -> Vec<u8> {
    let mut x = dec;
    let mut bv = Vec::new();
    while x > 0 {
        let b = x % 2;
        x = x / 2;
        bv.push(b);
    }

    match e {
        "little" => {
            while bv.len() < len as usize{
                bv.insert(bv.len(), 0);
            }
            bv
        },
        "big" => {
            let mut bv_copy: Vec<u8> = bv.into_iter().rev().collect();
            while bv_copy.len() < len as usize{
                bv_copy.insert(0, 0);
            }
            bv_copy
        }
        _ => panic!("Something went really wrong - no endianness received")
    }
}

fn append_padding_bits(input: &[u8]) -> Vec<u8> {
    let byte_vec = input.to_vec();
    let mut bit_vec = Vec::new();
    for byte_slice in byte_vec {
        bit_vec.append(&mut dec_to_bin(byte_slice, "little", 8));
    }
    bit_vec.append(&mut dec_to_bin(1, "big", 8));
    while bit_vec.len() % 512 != 448 {
        bit_vec.push(0);
    }

    bit_vec
}

fn transform_into_bytes(mut bit_vec: Vec<u8>, input: String) -> Vec<u8> {

    let length = input.chars().count() * 8;
    let length_bytes = length.to_le_bytes();
    for byte in length_bytes.iter() {
        bit_vec.append(&mut dec_to_bin(*byte, "little", 8));
    }
    bit_vec
}

fn set_up_md_buffer() -> Md5Buffer {

    let buffer = Md5Buffer {
        a: 0x67452301,
        b: 0xEFCDAB89,
        c: 0x98BADCFE,
        d: 0x10325476
    };
    buffer
}

struct ProcessMessage {}

impl ProcessMessage {

    fn generate_output(mut buffer: Md5Buffer, bit_vector: Vec<u8>) -> Md5Buffer {
        let mut buf_clone = buffer.clone();

        let n = bit_vector.len() / 32;
        for chunk in 0..(n / 16) {

            let begin = chunk * 512;
            let mut x = Vec::new();
            for j in 0..16 {
                x.push(&bit_vector[begin + (j * 32)..begin + (j * 32) + 32])
            }

            buffer = Md5Buffer{
                a: buf_clone.a,
                b: buf_clone.b,
                c: buf_clone.c,
                d: buf_clone.d
            };

            let mut x_int = Vec::new();
            for word in x {
                x_int.push(bin_to_dec(word));
            }
            let mut temp: u32 = 0;
            let mut k = 0;
            let mut s: [u32; 4] = [0, 0, 0, 0];
            let t = ProcessMessage::t_table();

            for i in 0..64 {

                let input = AuxInput {
                    x: buffer.b,
                    y: buffer.c,
                    z: buffer.d
                };
                if i <= 15 {
                    k = i;
                    s = [7, 12, 17, 22];
                    temp = ProcessMessage::f(input.clone());
                } else if 16 <= i && i <= 31 {
                    k = ((5 * i) + 1) % 16;
                    s = [5, 9, 14, 20];
                    temp = ProcessMessage::g(input.clone());
                } else if 32 <= i && i <= 47 {
                    k = ((3 * i) + 5) % 16;
                    s = [4, 11, 16, 23];
                    temp = ProcessMessage::h(input.clone());
                } else if 48 <= i && i <= 63 {
                    k = (7 * i) % 16;
                    s = [6, 10, 15, 21];
                    temp = ProcessMessage::i(input.clone());
                }

                temp = temp.wrapping_add(x_int[k] as u32);
                temp = temp.wrapping_add(t[i]);
                temp = temp.wrapping_add(buffer.a);
                temp = ProcessMessage::rotate_left(temp, s[i%4].into());
                temp = temp.wrapping_add(buffer.b);

                buffer.a = buffer.d;
                buffer.d = buffer.c;
                buffer.c = buffer.b;
                buffer.b = temp;
            }

            buf_clone.a = buf_clone.a.wrapping_add(buffer.a);
            buf_clone.b = buf_clone.b.wrapping_add(buffer.b);
            buf_clone.c = buf_clone.c.wrapping_add(buffer.c);
            buf_clone.d = buf_clone.d.wrapping_add(buffer.d);
        }
        buf_clone

    }
    fn f(input: AuxInput) -> u32 {
        (input.x & input.y) | (!input.x & input.z)
    }

    fn g(input: AuxInput) -> u32 {
        (input.x & input.z) | (input.y & !input.z)
    }

    fn h(input: AuxInput) -> u32 {
        input.x ^ input.y ^ input.z
    }

    fn i(input: AuxInput) -> u32 {
        input.y ^ (input.x | !input.z)
    }

    fn rotate_left(x: u32, n: u32) -> u32 {
        (x << n) | (x >> (32 - n))
    }

    fn t_table() -> Vec<u32> {
        let mut t = Vec::new();
        for i in 1..65 {
            let t_c = (2f64.powi(32) * (i as f64).sin().abs()) as u32;
            t.push(t_c);
        }
        t
    }
}

fn format_buffer(buffer: Md5Buffer) -> String{
    let bytes_a : [u8; 4] = (buffer.a as u32).to_le_bytes();
    let bytes_b : [u8; 4] = (buffer.b as u32).to_le_bytes();
    let bytes_c : [u8; 4] = (buffer.c as u32).to_le_bytes();
    let bytes_d : [u8; 4] = (buffer.d as u32).to_le_bytes();

    let msg = format!("{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}\
    {:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
                      bytes_a[0], bytes_a[1], bytes_a[2], bytes_a[3],
                      bytes_b[0], bytes_b[1], bytes_b[2], bytes_b[3],
                      bytes_c[0], bytes_c[1], bytes_c[2], bytes_c[3],
                      bytes_d[0], bytes_d[1], bytes_d[2], bytes_d[3]
    );
    msg
}


pub struct MD5HashCashChallenge {
    input: MD5HashCashInput,

}


impl ChallengeTrait for MD5HashCashChallenge {
    type Input = MD5HashCashInput;
    type Output = MD5HashCashOutput;

    fn name() -> String {
        return "MD5HashCash".to_string();
    }

    fn new(input: Self::Input) -> Self {

        return MD5HashCashChallenge{
            input
        };
    }

    fn solve(&self) -> MD5HashCashOutput {
        let mut seed : u64 = 2;
        let message_to_test : &str = &self.input.message;
        let mut string_bits = "".to_owned();
        let mut output;
        loop {
            let hex_string = format!("{:X}", seed);
            string_bits.push_str(&"0".repeat(16 - hex_string.chars().count()));
            string_bits.push_str(&hex_string);
            string_bits.push_str(&message_to_test);
            let hash_from_seed = format_buffer(ProcessMessage::generate_output(set_up_md_buffer(), transform_into_bytes(append_padding_bits(string_bits.as_bytes()), string_bits.to_string())));
            let prefix = match isize::from_str_radix(&convert_to_binary_from_hex( &*hash_from_seed )[0..self.input.complexity as usize], 2) {
                Ok(prefix) => prefix,
                Err(_) => 0
            };
            if prefix == 0 {
                output = MD5HashCashOutput {
                    seed: hex_string.parse::<u64>().unwrap(),
                    hashcode: hash_from_seed,
                };
                return output;
            }
            seed += 1;
        }
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        todo!();
    }

}
fn convert_to_binary_from_hex(hex: &str) -> String {
    hex.chars().map(to_binary).collect()
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use super::*;

    fn test_hash(input: &String, expect: String) {
        assert_eq!(format_buffer(ProcessMessage::generate_output(set_up_md_buffer(), transform_into_bytes(append_padding_bits(input.as_bytes()), input.to_string()))), expect);
    }
    #[test]
    fn test_cases() {
        let before = Instant::now();
        test_hash(&"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".to_string(), "d174ab98d277d9f5a5611c2c9f419d9f".to_string().to_uppercase());
        test_hash_cash();
        println!("Elapsed time: {:.2?}", before.elapsed());
    }

    
    fn test_hash_cash() {
        let input = MD5HashCashInput {
            complexity: 9,
            message : "Hello".to_string(),
        };
        let challenge = MD5HashCashChallenge {
            input
        };
        println!("before solve");

        let output = challenge.solve();
        println!("after solve");


        assert_eq!(output.hashcode.to_string() , "00057B92ACDC1641929946FA0E792CC8".to_string() );

    }
}

