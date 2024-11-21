use mockall::mock;

mod tests {
    use super::*;
    use std::fs;
    use std::io::{self, Write, Read};
    use std::net::TcpListener;
    use std::thread;
    use std::time::Duration;
    use bencher::{Bencher};

    /// Tests for `TrackedFile` functionality
    #[test]
    fn test_tracked_file_write() -> io::Result<()> {
        let filepath = "testfile.txt";
        {
            let mut tracked_file = TrackedFile::open(filepath, true)?;
            tracked_file.write(b"Hello, world!")?;
        }
        let tracked_file = TrackedFile::open(filepath, false)?;
        assert_eq!(tracked_file.tracker.bytes_written, 13);
        assert_eq!(tracked_file.tracker.write_operations, 1);
        fs::remove_file(filepath)?;
        Ok(())
    }

    #[test]
    fn test_tracked_file_read() -> io::Result<()> {
        let filepath = "testfile.txt";
        fs::write(filepath, b"Hello, world!")?;
        let mut tracked_file = TrackedFile::open(filepath, false)?;
        let mut buffer = vec![0; 13];
        tracked_file.read(&mut buffer)?;
        assert_eq!(buffer, b"Hello, world!");
        assert_eq!(tracked_file.tracker.bytes_read, 13);
        assert_eq!(tracked_file.tracker.read_operations, 1);
        fs::remove_file(filepath)?;
        Ok(())
    }

    #[test]
    fn test_tracked_file_read_empty() -> io::Result<()> {
        let filepath = "emptyfile.txt";
        fs::File::create(filepath)?;
        let mut tracked_file = TrackedFile::open(filepath, false)?;
        let mut buffer = vec![0; 10];
        let bytes_read = tracked_file.read(&mut buffer)?;
        assert_eq!(bytes_read, 0);
        assert_eq!(tracked_file.tracker.bytes_read, 0);
        assert_eq!(tracked_file.tracker.read_operations, 1);
        fs::remove_file(filepath)?;
        Ok(())
    }

    #[test]
    fn test_tracked_file_write_large_data() -> io::Result<()> {
        let filepath = "largefile.txt";
        let data = vec![0u8; 1_000_000];
        {
            let mut tracked_file = TrackedFile::open(filepath, true)?;
            tracked_file.write(&data)?;
        }
        let tracked_file = TrackedFile::open(filepath, false)?;
        assert_eq!(tracked_file.tracker.bytes_written, 1_000_000);
        assert_eq!(tracked_file.tracker.write_operations, 1);
        fs::remove_file(filepath)?;
        Ok(())
    }

    /// Mocking tests for `TrackedSocket`
    mock! {
        pub Socket {}
        impl Read for Socket {
            fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
        }
        impl Write for Socket {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize>;
            fn flush(&mut self) -> io::Result<()>;
        }
    }

    #[test]
    fn test_tracked_socket_write() -> io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let addr = listener.local_addr()?;
        let handle = thread::spawn(move || {
            let (mut socket, _) = listener.accept().unwrap();
            let mut buffer = [0; 13];
            socket.read_exact(&mut buffer).unwrap();
        });

        let mut tracked_socket = TrackedSocket::connect(addr)?;
        tracked_socket.write(b"Hello, world!")?;
        handle.join().unwrap();
        assert_eq!(tracked_socket.tracker.bytes_written, 13);
        assert_eq!(tracked_socket.tracker.write_operations, 1);

        Ok(())
    }

    #[test]
    fn test_tracked_socket_read() -> io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let addr = listener.local_addr()?;
        let handle = thread::spawn(move || {
            let (mut socket, _) = listener.accept().unwrap();
            socket.write_all(b"Hello, world!").unwrap();
        });

        let mut tracked_socket = TrackedSocket::connect(addr)?;
        let mut buffer = vec![0; 13];
        tracked_socket.read(&mut buffer)?;
        assert_eq!(buffer, b"Hello, world!");
        assert_eq!(tracked_socket.tracker.bytes_read, 13);
        assert_eq!(tracked_socket.tracker.read_operations, 1);

        handle.join().unwrap();
        Ok(())
    }

    /// Benchmark for `TrackedFile` write operations
    #[bench]
    fn bench_tracked_file_write(b: &mut Bencher) {
        let filepath = "benchfile.txt";
        b.iter(|| {
            let mut tracked_file = TrackedFile::open(filepath, true).unwrap();
            tracked_file.write(b"Benchmarking tracked file write").unwrap();
        });
        fs::remove_file(filepath).unwrap();
    }

    /// Benchmark for `TrackedSocket` write operations
    #[bench]
    fn bench_tracked_socket_write(b: &mut Bencher) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let handle = thread::spawn(move || {
            listener.accept().unwrap();
        });

        b.iter(|| {
            let mut tracked_socket = TrackedSocket::connect(addr).unwrap();
            tracked_socket.write(b"Benchmarking tracked socket write").unwrap();
        });

        handle.join().unwrap();
    }
}