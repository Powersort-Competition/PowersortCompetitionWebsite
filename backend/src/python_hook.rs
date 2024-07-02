use std::{env, str};
use std::process::Command;

pub async fn run_python_script(arr_tmpfile_path: String)
{
    let python_bin = env::var("PYTHON_BIN").expect("PYTHON_BIN must be set");
    
    let output = Command::new(python_bin)
        .arg("/media/shayan/Secondary1/Git Repos/PowersortCompetitionWebsite/backend/py_assets/Entrypoint.py")
        .arg(arr_tmpfile_path)
        .output()
        .expect("Failed to execute Python server-side computation command!");
   
    // Convert from bytes to string.
    let output_str = str::from_utf8(&output.stdout).unwrap();
    
    println!("Python script output: {}", output_str);
}