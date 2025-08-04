#[cfg(test)]
mod tests {
    use std::process::{Command, Stdio};

    #[test]
    fn test_input_output_roundtrip() {
        let input = "530070000\n600195000\n098000060\n800060003\n400803001\n700020006\n060000280\n000419005\n000080079\n";
        let expected = [
            "530070000",
            "600195000",
            "098000060",
            "800060003",
            "400803001",
            "700020006",
            "060000280",
            "000419005",
            "000080079",
        ];
        let output = Command::new(env!("CARGO_BIN_EXE_sudoku_solver"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                use std::io::Write;
                child.stdin.as_mut().unwrap().write_all(input.as_bytes())?;
                let output = child.wait_with_output()?;
                Ok(output)
            })
            .expect("Failed to run binary");
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in expected.iter() {
            assert!(stdout.contains(line), "Output missing line: {}", line);
        }
    }
}
