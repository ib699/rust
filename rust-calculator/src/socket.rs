use std::{
    fs::File,
    io::Read,
    io::Write,
    net::{TcpListener, TcpStream},
};

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    println!("Request Accepted");

    let get = b"GET / HTTP/1.1\r\n";
    let calculate: &[u8; 26] = b"POST /calculate HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let mut file = File::open("src/ui/claculator.html").unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else if buffer.starts_with(calculate) {
        // geting body from main request
        let lines = String::from_utf8_lossy(&buffer[..]);
        let json_start = lines.find("[");
        let json_end = lines.rfind("]");

        let mut contents = "".to_string();

        if let (Some(start), Some(end)) = (json_start, json_end) {
            let json_part = &lines[start + 1..end];

            // Split the JSON part into two elements
            let parts: Vec<&str> = json_part.split(",").collect();

            if let Some(first_part) = parts.get(0) {
                let clean_part = first_part.replace("\"", "");
                let mut split_command_string = clean_part.split_whitespace();

                // Extract the first number and fist_simb
                let first_num = split_command_string.next().unwrap_or("");
                let fist_simb = split_command_string.next().unwrap_or("");

                // Extract the second number
                let second_num = parts.get(1).unwrap_or(&"1").replace("\"", "").replace(" ", "");

                // now calculating equation
                let mut operator = 0;
                match fist_simb {
                    "*" => operator = 1,
                    "+" => operator = 2,
                    "/" => operator = 3,
                    "-" => operator = 4,
                    _ => println!("bad data!!!!"),
                }

                // parse string to int
                let int_fist_num: i32 = first_num.parse().unwrap();
                let int_second_line: i32 = second_num.parse().unwrap_or(0);
                
                let mut answer: i32=0;
                match operator {
                    1 => answer = int_fist_num * int_second_line, //println!("Answer is: {}", int_fist_num * int_second_line),
                    2 => answer = int_fist_num + int_second_line, //println!("Answer is: {}", int_fist_num + int_second_line),
                    3 => answer = int_fist_num / int_second_line, //println!("Answer is: {}", f64::from(int_fist_num) / f64::from(int_second_line)),
                    4 => answer = int_fist_num - int_second_line, //println!("Answer is: {}", int_fist_num - int_second_line),
                    _ => println!("Wrong lines bye!!!!"),
                }

                contents = format!("{{\"sum\": {}}}", answer);
            }
        }

        // sending response to client
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        println!("calculated equation :) ");
    } else {
        println!("Wrong Request!!!!");
    }
}
