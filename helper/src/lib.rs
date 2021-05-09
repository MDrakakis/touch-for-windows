pub mod args_and_create_file {
    use std::env;
    use std::fs::File;
    use std::path::Path;

    pub fn get_user_args() -> Vec<String> {
        let args: Vec<String> = env::args().collect();
        args
    }

    pub fn create_file(args: &Vec<String>) -> std::io::Result<()> {
        // args.len() == 2 is query and filename
        if !args.is_empty() && args.len() == 2 {
            let filename = &args[1];

            if !Path::new(&filename.to_string()).exists() {
                match File::create(filename.to_string()) {
                    Err(e) => return Err(e),
                    Ok(f) => f,
                };
            } else {
                println!("File already exists...")
            }
        } else {
            println!("Argument command error!");
        }

        Ok(())
    }
}
