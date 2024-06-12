#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::thread;

    #[test]
    fn test_file_processing() {
        let count = thread::available_parallelism().unwrap().get();
        print!("AVAILABLE PARALLELISM: {count}");
        let mut file = File::create("data/test.txt").unwrap();
        writeln!(file, "Line 1\nLine 2\nLine 3\nLine 4").unwrap();
        let result = crate::file_processor::process_file("data/test.txt").unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_server() {
        let count = thread::available_parallelism().unwrap().get();
        print!("AVAILABLE PARALLELISM: {count}");
        thread::spawn(|| {
            crate::server::start_server("127.0.0.1:1080");
        });
        let mut stream = TcpStream::connect("127.0.0.1:1080").unwrap();
        stream.write_all(b"data/test2.txt").unwrap();
        let mut buffer = [0; 128];
        let size = stream.read(&mut buffer).unwrap();
        let response = String::from_utf8_lossy(&buffer[..size]);
        assert!(response.contains("Line count: 1000"));
    }
}
