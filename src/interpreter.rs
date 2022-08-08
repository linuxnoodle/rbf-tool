use std::io::Read;

fn get_corresponding_bracket(code: &str, b_loc: usize) -> usize{
    let mut c_loc = b_loc;
    let mut counter = 1;
    let weight: i32 = if code.chars().nth(b_loc) == Some('[') { 1 } else { -1 };
    while counter > 0 {
        if weight.is_positive() {
            c_loc += 1; 
        } else {
            c_loc -= 1;
        }

        if code.chars().nth(c_loc) == Some('[') {
            counter += weight;
        } else if code.chars().nth(c_loc) == Some(']') {
            counter -= weight;
        }
    }
    c_loc
}

pub fn interpret(code: &str) {
    let mut mem: [u8; 30000] = [0; 30000];
    let mut mem_ptr = 0;
    let mut index = 0;
    while index <= code.len() {
        // print current memory state for the first 256 cells
        /*print!("\n");
        for i in 0..256 {
            print!("{:03}", mem[i]);
            if i % 16 == 15 {
                println!();
            } else {
                print!(" ");
            }
        }*/
        match code.chars().nth(index) {
            Some('>') => {
                mem_ptr = if mem_ptr == 29999 { 0 } else { mem_ptr + 1 };
            },
            Some('<') => {
                mem_ptr = if mem_ptr == 0 { 29999 } else { mem_ptr - 1 };
            },
            Some('+') => {
                mem[mem_ptr] = if mem[mem_ptr] == 255 { 0 } else { mem[mem_ptr] + 1 };
            },
            Some('-') => {
                mem[mem_ptr] = if mem[mem_ptr] == 0 { 255 } else { mem[mem_ptr] - 1 };
            },
            Some('.') => {
                print!("{}", mem[mem_ptr] as char);
            },
            Some(',') => {
                let input: Option<i32> = std::io::stdin()
                    .bytes()
                    .next()
                    .and_then(|b| b.ok())
                    .map(|b| b as i32);
                mem[mem_ptr] = match input {
                    Some(i) => i as u8,
                    None => 0,
                };
            },
            Some('[') => {
                if mem[mem_ptr] == 0 {
                    index = get_corresponding_bracket(code, index);
                }
            },
            Some(']') => {
                if mem[mem_ptr] != 0 {
                    index = get_corresponding_bracket(code, index);
                }
            },
            _ => ()
        }
        index += 1;
    }
}
