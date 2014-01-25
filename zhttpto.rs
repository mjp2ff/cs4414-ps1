//
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 0.9
//
// Note that this code has serious security risks!  You should not run it 
// on any system with access to sensitive files.
// 
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3

#[feature(globs)];
use std::io::*;
use std::io::net::ip::{SocketAddr};
use std::{str};

static IP: &'static str = "127.0.0.1";
static PORT:        int = 4414;

fn main() {
    let addr = from_str::<SocketAddr>(format!("{:s}:{:d}", IP, PORT)).unwrap();
    let mut acceptor = net::tcp::TcpListener::bind(addr).listen();
    static mut visit_count : int = 0;
    
    println(format!("Listening on [{:s}] ...", addr.to_str()));
    
    for stream in acceptor.incoming() {
        // Spawn a task to handle the connection
        do spawn {
            let mut stream = stream;
            
            match stream {
                Some(ref mut s) => {
                             match s.peer_name() {
                                Some(pn) => {println(format!("Received connection from: [{:s}]", pn.to_str()));},
                                None => ()
                             }
                           },
                None => ()
            }
            
            let mut buf = [0, ..500];
            stream.read(buf);
            let request_str = str::from_utf8(buf);
            println(format!("Received request :\n{:s}", request_str));

            let first_line = request_str.split('\n').next().unwrap();
            let first_line_array = first_line.split(' ').to_owned_vec();
            
            if (first_line_array[0].trim() == "GET" &&
                    first_line_array[2].trim() == "HTTP/1.1" &&
                    first_line_array[1].trim() != "/")  {
                
                let filepath = ~std::path::posix::Path::new(
                    first_line_array[1].trim().split('/').to_owned_vec()[1].trim());
                    // ^ Removes leading '/' in order to get into current directory.
                
                let mut response : ~str = ~"";
                
                if (filepath.is_file()) {
                    
                    if (first_line_array[1].trim().split('.').last().unwrap() == "html") {
                        let mut file = File::open(filepath);
                        stream.write(file.read_to_end());
                    }
                    
                    else { response = response + format!("HTTP/1.1 403 FORBIDDEN"); }
                }

                else { response = response + format!("HTTP/1.1 404 NOT FOUND"); }

                if (!filepath.is_file() || first_line_array[1].trim().split('.').last().unwrap() != "html") {
                    response = response + 
                        format!("\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                        <doctype !html><html><head><title>{:s}</title>
                        <style>body \\{ background-color: \\#111; color: \\#FFEEAA \\}
                                h1 \\{ font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red\\}
                                h2 \\{ font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green\\}
                                p \\{ text-align: center \\}
                         </style></head>
                         <body>
                         <h1>{:s}</h1>
                         <p>
                            This isn't the file you are looking for.<br><br>
                            <img src=\"http://i.imgur.com/5uDhTkr.jpg\" alt=\"Ya 'dun goofed!\">
                         </p>
                         </body></html>\r\n", 
                         if(!filepath.is_file()) { "404 - NOT FOUND" } else { "403 - FORBIDDEN" },
                         if(!filepath.is_file()) { "404 - NOT FOUND" } else { "403 - FORBIDDEN" });
                    stream.write(response.as_bytes());
                }
            }
            else {
                unsafe {
                    visit_count += 1;

                    let response: ~str = 
                        format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                         <doctype !html><html><head><title>Hello, Rust!</title>
                         <style>body \\{ background-color: \\#111; color: \\#FFEEAA \\}
                                h1 \\{ font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red\\}
                                h2 \\{ font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green\\}
                         </style></head>
                         <body>
                         <h1>Greetings, Krusty!</h1>
                         <h2>Visitor count is {:d}.</h2>
                         </body></html>\r\n", visit_count);
                    stream.write(response.as_bytes());
                }
            }
            println!("Connection terminates.");
        }
    }
}
