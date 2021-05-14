use std::fs;
use std::io;
use std::path::Path;

pub fn data_dir(data_dir: &str) -> io::Result<Option<String>> {
    if Path::new(data_dir).exists() {
        println!("There already exists data from a previous run.");
        println!("Current path: {}", data_dir);
        println!("Options: y -> delete existing data");
        println!("         r -> rename data directory");
        println!("         n -> cancel execution");
        let reply = rprompt::prompt_reply_stdout("(Y/r/n) ")?;

        #[allow(clippy::wildcard_in_or_patterns)]
        match reply.as_str() {
            "" | "y" | "Y" => {
                fs::remove_dir_all(data_dir)?;
                println!("Old data has been removed.");
                Ok(Some(data_dir.to_owned()))
            }
            "r" | "R" => {
                let reply = rprompt::prompt_reply_stdout("New data name: ")?;
                Ok(Some(reply))
            }
            "n" | "N" | _ => Ok(None),
        }
    } else {
        Ok(Some(data_dir.to_owned()))
    }
}
