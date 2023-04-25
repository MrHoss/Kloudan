use std::fs;

pub fn create_dir(path:String) -> std::io::Result<()> {
    println!("{}",&path);

    let main_path = "client_dir";
    fs::create_dir_all(&main_path).unwrap();
    fs::create_dir(format!("{}/{}",&main_path, &path))?;
    Ok(())
}