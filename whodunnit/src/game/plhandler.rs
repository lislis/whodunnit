use std::process::{Command};
use std::fs::File;
use std::path::Path;
use std::str::Utf8Error;
use std::io;

use crate::game::utils;

#[derive(Debug)]
pub struct PLHandler {
    tmp_path: String,
    db_path: String,
    tmp_file: File,
    db_file: File,
    goal: String,
    prolog_string: String
}

impl PLHandler {
    pub fn new(tmp_file: &str, db_path: &str) -> Result<Self, std::io::Error> {
        let tmp_path: String =  tmp_file.to_string();
        let db_path: String = db_path.to_string();

        let tmp_file = utils::open_or_create_file_from_path(Path::new(&tmp_path))?;
        let db_file = utils::open_or_create_file_from_path(Path::new(&db_path))?;

        Ok(Self {
            tmp_path,
            db_path,
            tmp_file,
            db_file,
            goal: String::new(),
            prolog_string: String::new()
        })
    }

    pub fn get_current_goal(&self) -> &str {
        &self.goal
    }

    pub fn prompt_goal(&mut self) -> Result<(), std::io::Error> {
        let mut buffer = Default::default();
        println!("State your goal (solve for X, no . at the end):");
        let _goal = io::stdin().read_line(&mut buffer)?;
        self.goal = buffer.trim().to_string();

        self.format_prolog_query();
        Ok(())
    }

    pub fn read_output(&self) -> Result<String, Utf8Error> {
        let cat = Command::new("cat")
            .arg(&self.tmp_path)
            .output()
            .expect("Could not call cat");
        let out = std::str::from_utf8(&cat.stdout).unwrap().to_string();
        Ok(out)
    }

    pub fn query_prolog(&self) -> Result<std::process::Output, std::io::Error> {
        let child = Command::new("swipl")
            .arg("-s")
            .arg(&self.db_path)
            .arg("-g")
            .arg(&self.prolog_string)
            .arg("-g")
            .arg("halt.")
            .output();
        child
    }

    fn format_prolog_query<'a>(&mut self) {
        self.prolog_string = format!("{}, open(\"{}\", write, Stream, []), format(Stream, X, []).", &self.goal, &self.tmp_path);
    }

}
