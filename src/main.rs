use std::io;
fn main() {
    let mut decoding = false;
    for arg in std::env::args() {
        if arg.eq_ignore_ascii_case("-d") || arg.eq_ignore_ascii_case("--decode") {
            decoding = true;
        }
    }
    let mut buffer: String = String::new(); 
    io::stdin().read_line(&mut buffer).expect("failed to read input");
    buffer = buffer.trim_end().to_owned(); 
    let words = buffer.split(" ");
    let mut total_shift = 0;
    let mut output_buffer: String = String::new();
    for word in words {
        let word_len_shift: i16 = (word.len() - 1).try_into().unwrap();
        for character in word.chars() {
            let character = character.to_lowercase().next().unwrap();
            total_shift += 1;
            match character {
                // consonants
                'b' | 'c' | 'd' | 'f' | 'g' | 'h' | 'j' | 'k' | 'l' | 'm' | 'n' | 'p' | 'q' | 'r' | 's' | 't' | 'v' | 'w' | 'x' | 'z' => 
                {
                    let mut index: i16 = ['b' , 'c' , 'd' , 'f' , 'g' , 'h' , 'j' , 'k' , 'l' , 'm' , 'n' , 'p' , 'q' , 'r' , 's' , 't' , 'v' , 'w' , 'x' , 'z']
                    .iter().position(|&current| current.eq_ignore_ascii_case(&character)).expect("not in array").try_into().unwrap();
                    let shift = total_shift + word_len_shift;
                    if decoding {
                        index -= shift;
                    } else {
                        index += shift;
                    }
                    index = index % 20;
                    if index < 0 {
                        index = 20 + index
                    }
                    output_buffer += ['b' , 'c' , 'd' , 'f' , 'g' , 'h' , 'j' , 'k' , 'l' , 'm' , 'n' , 'p' , 'q' , 'r' , 's' , 't' , 'v' , 'w' , 'x' , 'z'][index as usize].to_string().as_str()
                }
                // vowels (y is always a vowel)
                'a' | 'e' | 'i' | 'o' | 'u' | 'y'  => {
                    let mut index: i16 = ['a','e','i','o','u','y']
                    .iter().position(|&current| current.eq_ignore_ascii_case(&character)).expect("not in array").try_into().unwrap();
                    let shift = total_shift + word_len_shift;
                    if decoding {
                        index -= shift;
                    } else {
                        index += shift;
                    }
                    index = index % 6;
                    if index < 0 {
                        index = 6 + index
                    }
                    output_buffer += ['a','e','i','o','u','y'][index as usize].to_string().as_str()
                }
                // anything else
                _ => {
                    output_buffer += character.to_string().as_str();
                }
            }
        } 
        output_buffer += " ";
    }
    println!("{}",output_buffer);
}