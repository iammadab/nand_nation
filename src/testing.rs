use std::env;
use std::fs;

pub(crate) struct TestReader {}

impl TestReader {
    pub(crate) fn read(test_file_name: &'static str) -> Vec<String> {
        let current_directory = env::current_dir().unwrap();
        let file_path = current_directory
            .join("src/test_files")
            .join(test_file_name);
        let file_content = fs::read_to_string(file_path).unwrap();

        let tokens: Vec<String> = file_content
            .lines()
            .flat_map(|line| line.split("|"))
            .map(str::trim)
            .map(str::to_string)
            .filter(|a| !a.is_empty())
            .collect();

        tokens
    }
}
