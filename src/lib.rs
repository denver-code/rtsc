use std::{error::Error, process, fs};

#[cfg(windows)]
pub const NPM: &'static str = "npm.cmd";

#[cfg(not(windows))]
pub const NPM: &'static str = "npm";

pub struct Config {
    pub project_name: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments, please use \"rtsc <project_name>\"");
        } 

        let project_name = args[1].clone();

        Ok(Config {project_name})
    }
}

fn create_dir(path: String) {
    std::fs::create_dir_all(path)
        .unwrap_or_else(|e| panic!("Error creating dir: {}", e));
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    create_dir(format!("{}/", config.project_name).to_string());
    create_dir(format!("{}/src/", config.project_name).to_string());

    process::Command::new(NPM)
        .arg("init")
        .arg("-y")
        .arg("--location=project")
        .status()?;

    fs::copy(
        "./package.json", 
        format!("./{}/package.json", config.project_name)
    ).unwrap_or_else(
        |e|
        panic!("Error while coping file: {}", e)
    );
    
    fs::remove_file(
        "./package.json"
    ).unwrap_or_else(
        |e|
        panic!("Error while deleting file: {}", e)
    );

    process::Command::new(NPM)
        .arg("install")
        .arg("express")
        .arg("dotenv")
        .arg("--location=project")
        .arg("--prefix")
        .arg(format!("./{}", config.project_name))
        .status()?;
    
    process::Command::new(NPM)
        .arg("install")
        .arg("--location=global")
        .arg("typescript")
        .status()?;

    fs::write(format!("./{}/tsconfig.json", config.project_name), "{ \
        \"compilerOptions\": { \
            \"target\": \"es2016\",                                 
            \"module\": \"commonjs\",                                 
            \"esModuleInterop\": true,                         
            \"forceConsistentCasingInFileNames\": true,      
            \"strict\": true,                                  
            \"skipLibCheck\": true                          
        },
        }
        ".to_string()).expect("Unable to write file");

        fs::write(format!("./{}/src/index.ts", config.project_name), "".to_string()).expect("Unable to write file");

    Ok(())
}
