use shellfn::shell;
use std::error::Error;


#[shell]
pub fn run_directory(dir: &str) -> Result<impl Iterator<Item=String>, Box<Error>> { r#"
    echo "Running all files in $DIR"
    cd $DIR
    ./*.sh
"# }
#[shell]
pub fn run_file(file: &str) -> Result<impl Iterator<Item=String>, Box<Error>> { r#"
    echo "Running $FILE"
    exec $FILE
"# }


//TODO can support python too
