use std::net::TcpStream;
use std::io::Write;
use std::fs;
use std::thread;
use std::env;
use std::sync::{Arc, Mutex};

pub fn addpic(stream: Arc<Mutex<TcpStream>>, _xw : usize, _yh: usize, path: &str) -> Result<(), std::io::Error> {
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
           let mut stream = stream.lock().unwrap();
           stream.write(message.as_bytes())?;
       }
   }
   Ok(())
}

fn main() {
   let args: Vec<String> = env::args().collect();
   let host = &args[1];
   let port = &args[2];
   let path = &args[3].clone();
   let stream = TcpStream::connect(format!("{}:{}", host, port)).expect("Could not connect to server");
   let stream = Arc::new(Mutex::new(stream));
   loop {
       let stream_clone = Arc::clone(&stream);
       thread::spawn({
           let path = path.clone();
           move || {
               let _ = addpic(stream_clone, 0, 0, &path);
           }
       });
   }
}
