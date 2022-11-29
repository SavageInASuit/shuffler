use std::io::{self, Read, Write};
use rand::prelude::*;

fn load_input(inp: &mut impl Read) -> String {
    let mut input = String::new();
    
    inp.read_to_string(&mut input).unwrap();

    input
}

fn shuffle_lines(input: &str) -> Vec<&str> {
    let mut rng = rand::thread_rng();
    let mut lines: Vec<&str> = input.lines().collect();
    lines.shuffle(&mut rng);

    lines
}

fn handle_output_error(err: io::Error) {
    // supress error in release, as stream may have just been closed
    //  e.g. when piping output to `head -n 10` whith more than 10 lines to output
    #[cfg(debug_assertions)]
    println!("Error writing to stdout: {}", err);
}

fn output_lines(out: &mut impl Write, lines: Vec<&str>) {
    for line in lines {
        match out.write(format!("{}\n", line).as_bytes()) {
            Ok(_) => (),
            Err(e) => handle_output_error(e),
        }
    }
}

fn main() {
    // Read input data
    let input = load_input(&mut io::stdin());

    // Shuffle the data
    let lines = shuffle_lines(&input);

    // Output the shuffled data
    output_lines(&mut io::stdout(), lines);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestReader {
        data: String,
    }

    impl Read for TestReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let data = self.data.as_bytes();
            let len = self.data.len();
            buf[..len].copy_from_slice(data);
            Ok(len)
        }

        fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
            buf.push_str(&self.data);
            Ok(self.data.len())
        }
    }

    #[test]
    fn test_load_input() {
        let input = "hello\nworld\ncup";
        let mut reader = TestReader { data: input.to_owned() };
        let output = load_input(&mut reader);

        assert_eq!(input, output);
    }

    #[test]
    fn test_shuffle_lines() {
        let input = "hello\nworld\ncup";
        let lines = shuffle_lines(input);
        
        assert!(lines.len() == 3);
        for x in input.lines() {
            assert!(lines.contains(&x));
        }
    }

    #[test]
    fn test_output_lines() {
        let mut writer = Vec::new();
        let lines = vec!["hello", "world", "cup"];
        output_lines(&mut writer, lines);

        assert_eq!(writer, b"hello\nworld\ncup\n");
    }
}