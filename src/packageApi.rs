// packageApi.rs

use std::fs;
use std::io::Write;
use std::path::Path;

use crate::log;
use crate::logExit;

// package api
pub async fn packageApi(values: &Vec<String>) -> () {
  let valuesLen: usize = values.len();
  if valuesLen == 0 && values[0].len() != 0 {
    log("err", &format!("There are no values in the package request"));
    logExit(1);
  }
  //
  let mut error: bool = false;

  let _type: &String = &values[0];
  if _type == "local" && valuesLen >= 2 {
    if packageLocal(&values[1]) != 0 {
      error = true;
    }
  } else
  if _type == "local-delete" && valuesLen >= 2 {
    if packageLocalDelete(&values[1]) != 0 {
      error = true;
    }
  } else {
    error = true;
  }

  if error {
    log("err", &format!("Incorrect command for package request"));
    logExit(1);
  }
}

// check that the directory already exists
fn checkDirectoryExist(packageName: &str) -> usize {
  let packageDirPath: &Path = Path::new(packageName);
  if packageDirPath.exists() {
    return 1;
  }
  return 0;
}
// check if the JSON file already exists
fn checkJsonExist(packageName: &str) -> usize {
    let packageJsonPath = if packageName.len() != 0 {
        format!("{}/{}", packageName, "package.json")
    } else {
        String::from("package.json")
    };
    let packageJsonInCurrentDir: &Path = Path::new(&packageJsonPath);
    if packageJsonInCurrentDir.exists() {
        return 1;
    }
    return 0;
}

fn packageLocal(packageName: &str) -> usize {
    if checkDirectoryExist(packageName) != 0 { 
      log("err", &format!("Package '{}' already exists", packageName));
      return 1; 
    }
    if checkJsonExist("") != 0 { 
      log("err", "You cannot create a new package in the package.json file");
      return 1; 
    }

    // create directory
    match fs::create_dir_all(packageName) {
      Ok(_) => log("ok", &format!("Directory '{}' created successfully", packageName)),
      Err(e) => {
        println!("Error creating directory '{}': {}", packageName, e);
        return 1;
      }
    }
    // create main.rt file
    let mainFilePath = Path::new(packageName).join("main.rt");
    match fs::File::create(&mainFilePath) {
      Ok(_) => log("ok", &format!("File 'main.rt' created successfully")),
      Err(e) => {
        log("err", &format!("Error creating file 'main.rt': {}", e));
        return 1;
      }
    }
    // create JSON file
    let packageJsonPath = Path::new(packageName).join("package.json");
    let mut packageJsonFile = match fs::File::create(&packageJsonPath) {
      Ok(file) => file,
      Err(e) => {
        log("err", &format!("Error creating file 'package.json': {}", e));
        return 1;
      }
    };

    // content
    let packageJsonContent = format!(
        r#"{{
  "name": "{}",
  "version": "1.0.0",
  "dependencies": {{}}
}}"#,
      packageName
    );
    // write json content
    if let Err(e) = packageJsonFile.write_all(packageJsonContent.as_bytes()) {
      log("err", &format!("Error writing to 'package.json': {}", e));
      return 1;
    } else {
      log("ok", "File 'package.json' created successfully");
    }

    return 0;
}

fn packageLocalDelete(packageName: &str) -> usize {
    if checkDirectoryExist(packageName) != 1 { 
      log("err", &format!("Package '{}' does not exist", packageName));
      return 1; 
    }
    if checkJsonExist(packageName) != 1 { 
      log("err", "You can't delete a package without package.json inside");
      return 1; 
    }

    // delete package directory and files
    return match fs::remove_dir_all(Path::new(packageName)) {
        Ok(_) => {
            log("ok", &format!("Package '{}' and its contents have been removed successfully", packageName));
            0
        }
        Err(e) => {
            log("err", &format!("Error removing directory '{}': {}", packageName, e));
            1
        }
    }
}
