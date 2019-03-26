use symlink;
use std::path::Path;
use std::path::PathBuf;
use std::error::Error;
use std::fs::*; 
use std::fs; 
use std::io::Read; 
pub fn create_base_sym(file_path: PathBuf){
   for entry in fs::read_dir(file_path) {
        if let Ok(entry) = entry { 
            let display = file_path.display();
            let mut fileName = entry.file_name();
            fileName = fileName + ".base"; 
            let the_file: File = match File::open(&file_path) { 
                Err(why) => panic!("couldn't open {}: {}", display,
                                                           why.description()),
                Ok(file) => file, 
            };
            let mut s = String::new(); 

            match the_file.read_to_string(&mut s) { 
                Err(why) => panic!("Couldn't read {}: {}", display,
                                                           why.description()), 
                Ok(_) => print!("{} contains:\n{}", display, s), 
            }
            file_path.pop();
            file_path.push(fileName); 
            

            let mut base_file: File = match File::create(&file_path) { 
                Err(why) => panic!("couldn't create {}: {}",
                                            display, 
                                            why.description())
                Ok(base_file) => base_file,
            }
            
            match base_file.write_all(s.as_bytes()) {
                Err(why) => { 
                    panic!("couldn't write to {}: {}", display, 
                                                       why.description())
                }, 

                Ok(_) => println!("Successfully wrote to .base"),
            }
        }
   }
}


