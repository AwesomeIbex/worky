use shellfn::shell;
use std::error::Error;


#[shell]
pub fn run_directory(dir: &str) -> Result<impl Iterator<Item=String>, Box<Error>> { r#"
    cd $DIR
    touch logs
    echo "Running all files in $DIR" >> logs
    ./*.sh >> logs
"# }
#[shell]
pub fn run_file_bash(file: &str) -> Result<impl Iterator<Item=String>, Box<Error>> { r#"
    echo "Running $FILE"
    exec $FILE
"# }

//TODO can support python too
