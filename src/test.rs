#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::thread;
    use std::time::Instant;

    #[test]
    fn test_file_processing() {
        let start = Instant::now();
        let mut file = File::create("data/test.txt").unwrap();
        writeln!(file, "Line 1\nLine 2\nLine 3\nLine 4").unwrap();
        let result = crate::file_processor::count_lines("data/test.txt").unwrap();
        assert_eq!(result, 4);
        let duration = start.elapsed();
        println!("Time elapsed in test_file_processing() is: {:?}", duration);
    }

    #[test]
    fn test_file_processing_2() {
        let start = Instant::now();
        let result = crate::file_processor::count_lines("data/test2.txt").unwrap();
        assert_eq!(result, 1005);
        let duration = start.elapsed();
        println!("Time elapsed in test_file_processing() is: {:?}", duration);
    }

    #[test]
    fn test_server() {
        thread::spawn(|| {
            crate::server::start_server("127.0.0.1:1080");
        });
        let mut stream = TcpStream::connect("127.0.0.1:1080").unwrap();
        stream.write_all(b"data/test.txt").unwrap();
        let mut buffer = [0; 128];
        let size = stream.read(&mut buffer).unwrap();
        let response = String::from_utf8_lossy(&buffer[..size]);
        assert!(response.contains("Line count: 1005"));
    }

    #[test]
    fn test_server_2() {
        thread::spawn(|| {
            crate::server::start_server("127.0.0.1:1080");
        });
        let mut stream = TcpStream::connect("127.0.0.1:1080").unwrap();
        stream.write_all(b"data/test2.txt").unwrap();
        let mut buffer = [0; 128];
        let size = stream.read(&mut buffer).unwrap();
        let response = String::from_utf8_lossy(&buffer[..size]);
        assert!(response.contains("Line count: 1005"));
    }
}
