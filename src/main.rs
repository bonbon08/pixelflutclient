use std::net::TcpStream;
use std::io::Write;
use std::fs;
use std::thread;


pub fn addpic(stream: &mut TcpStream, _xw : usize, _yh: usize, path: &str) -> Result<(), std::io::Error> {
   let content = fs::read_to_string(path)?;
   let mut lines: Vec<Vec<String>> = Vec::new();
   for line in content.lines() {
       let mut pixels: Vec<String> = Vec::new();
       for pixel in line.split('p') {
           let mut color = String::new();
           for part in pixel.split('s') {
               if !part.is_empty() {
               match part.parse::<u8>() {
                  Ok(val) => color += &format!("{:02X}", val),
                  Err(e) => println!("{}",e),
               }
               }
           }
           pixels.push(color);
       }
       lines.push(pixels);
   }
    for (y, line) in lines.iter().enumerate() {
       for (x, pixel) in line.iter().enumerate() {
           let message = format!("PX {} {} {}\n", x, y, pixel);
           stream.write(message.as_bytes())?;
       }
   }
   Ok(())
}



fn main() {
    let host = "127.0.0.1";
    let port = 1337;
    let stream = TcpStream::connect(format!("{}:{}", host, port)).expect("Could not connect to server");
    loop {
        let mut stream_clone = stream.try_clone().expect("Failed to clone stream");
        thread::spawn(move || {
            let _ = addpic(&mut stream_clone, 0, 0, "testpic_1.npf");
        });
    }
  }
  