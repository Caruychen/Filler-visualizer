use std::process::{Command, Stdio};
use json;

pub struct Arena {
    pub replay: String
}

impl Arena {
    fn call_cmd(filler: &str, args: &mut [&str]) -> String {
        let output = Command::new(filler)
            .args(args)
            .stdout(Stdio::piped())
            .output()
            .unwrap();
        let stdout = String::from_utf8(output.stdout).unwrap();
        stdout
    }

    pub fn run(filler: &str, args: &mut [&str]) -> Arena {
        let stdout = Arena::call_cmd(filler, args);
        let mut data = vec![];
        let mut index = 0;
        let mut is_begun = false;
        for line in stdout.split("\n") {
            let is_mapln = line.chars().count() > 3 && (&line[..3]).parse::<f64>().is_ok();
            if line.starts_with("Plateau") {
                data.push(vec![]);
                index += 1;
                is_begun = true;
            }
            else if is_begun && is_mapln {
                data[index - 1].push(&line[4..]);
            }
        }
    let replay = json::stringify(data);

    Arena { replay }
    }

    pub fn get_replay(&self) -> &String {
        &self.replay
    }
}
