use std::thread::sleep;
use std::{fs, io::Error, io::ErrorKind};
use std::time::{Duration, Instant};
use std::path::{Path, PathBuf};

// main function for file
// should return downloaded file path if exists otherwise return, io::Error
pub fn file_downloaded(file_name: String) -> Result<String, Error> {
    let dir_path = PathBuf::from("/Users/martindurak/Downloads");
    // TODO refactor this function
    match pool_download_dir(&dir_path, file_name) {
        Ok(file_path) => {
            println!("File downloaded successfully: {:?}", file_path);
            Ok(file_path.to_string_lossy().to_string())
        },
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e)
        }
    }
}

fn pool_download_dir(dir: &Path, link_file_name: String) -> Result<PathBuf, Error> {
   let start_time = Instant::now();
   let mut file_path: Option<PathBuf> = None;
   let mut last_size = 0;
   let mut founded = false;
   let link_file_name_lower_case = link_file_name.to_ascii_lowercase();

   while start_time.elapsed() < Duration::from_secs(3600)
   {
        if !founded
        {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    let file_name = match path.file_name(){
                        Some(file) => file.to_string_lossy(),
                        None => continue,
                    };

                    if file_name.to_ascii_lowercase().contains(&link_file_name_lower_case)
                    {
                        file_path = Some(path.to_path_buf());
                        founded = true;
                        break;
                    }
                }
            }
        }

        if let Some(ref path) = file_path{
            let metadata = fs::metadata(path)?;
            if last_size > 0 &&  metadata.len() == last_size 
            {
                // size is not changing, so it should be downloaded
                return Ok(Path::new(path).to_path_buf());

            }
            last_size = metadata.len();
        }
        sleep(Duration::from_millis(100));
   }
    // File with the specified name not found
    Err(Error::new(ErrorKind::NotFound, "File not found"))
}

    // it is not possible to implement if we don`t know how big file is.
    // we can know only:
    //  Clip ID302105
    // Resolution 3840 x 2160
    // Length00 : 29
    // Frame Rate 23.97
    // CameraRED DSMC2 HELIUM 8K S35 (RED Weapon Helium/RED Epic-W Helium)
    // Available Formats
    // HD .mp4
    // 4K .mp4
    // 4K HQ .mov`

