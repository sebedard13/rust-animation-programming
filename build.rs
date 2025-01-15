
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn get_output_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string).join("target").join(build_type);
    PathBuf::from(path)
}

fn get_all_files_recursive(path: &PathBuf, files: &mut Vec<PathBuf>){
    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir(){
            get_all_files_recursive(&path, files);
        }else{
            files.push(path);
        }
    }
}

fn main() {
    let target_dir = get_output_path();
    
    
    let rsc = Path::join(&env::current_dir().unwrap(), Path::new("rsc"));
    if rsc.exists() {
        let mut files = Vec::new();
        get_all_files_recursive(&rsc, &mut files);
        for file in files {
            let dest = Path::join(Path::new(&target_dir.join("rsc")), file.strip_prefix(&rsc).expect("Should be able to strip prefix of rsc folder of file in rsc folder"));
            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent).expect("Should be able to create parent directories");
            }
            fs::copy(file, dest).expect("Should be able to copy file");
        }
        println!("cargo:rerun-if-changed=rsc");
    }
}