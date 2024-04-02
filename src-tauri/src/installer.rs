use std::fs;
use std::io::Write;
use std::path::PathBuf;

use directories::UserDirs;
use mslnk::ShellLink;
use tauri::command;

#[command]
pub async fn install(location: String, create_desktop_shortcut: bool, create_start_menu_shortcut: bool, start_with_windows: bool)
{
    // get destination file
    let destination_file = download_exe(PathBuf::from(location)).await;

    // create desktop shortcut
    if create_desktop_shortcut {
        self::create_desktop_shortcut(&destination_file);
    }

    // create start menu shortcut
    if create_start_menu_shortcut {
        self::create_start_menu_shortcut(&destination_file);
    }
    // start with windows
    if start_with_windows {
        create_startup_shortcut(&destination_file);
    }
}

#[command]
pub fn start_application(exe: String)
{
    std::process::Command::new(exe).spawn().unwrap();
    std::process::exit(0);
}

#[command]
pub fn exit_application()
{
    std::process::exit(0);
}

async fn download_exe(directory: PathBuf) -> PathBuf
{
    // create directory
    if directory.exists()
    {
        fs::remove_dir_all(&directory).unwrap();
    }
    if let Err(_) = fs::create_dir_all(&directory) {}

    // download file
    let url = "https://pricing-new.mardens.com/api/clients/latest";
    let destination = directory.join("pricing-app.exe");
    if let Ok(response) = reqwest::get(url).await {
        let mut file = std::fs::File::create(&destination).unwrap();
        let content = response.bytes().await.unwrap();
        file.write_all(&content).unwrap();

        let url = "https://pricing-new.mardens.com/api/clients/updater";
        let destination = directory.join("updater.exe");
        if let Ok(response) = reqwest::get(url).await {
            let mut file = std::fs::File::create(&destination).unwrap();
            let content = response.bytes().await.unwrap();
            file.write_all(&content).unwrap();
        }
    }

    return destination;
}

#[command]
pub fn get_default_install_location() -> String
{
    if let Some(user_dirs) = UserDirs::new() {
        return user_dirs.home_dir().join("AppData").join("Local").join("Mardens Inc.").join("Pricing App").to_string_lossy().to_string();
    }
    return "".to_string();
}

fn create_start_menu_shortcut(destination_file: &PathBuf)
{
    if let Some(user_dirs) = UserDirs::new() {
        // create directory
        let directory = user_dirs.home_dir()
            .join("AppData")
            .join("Roaming")
            .join("Microsoft")
            .join("Windows")
            .join("Start Menu")
            .join("Programs")
            .join("Mardens Inc.");
        if let Ok(_) = fs::create_dir_all(&directory) {
            let shortcut = directory.join("Pricing App.lnk");
            let shell = ShellLink::new(destination_file).unwrap();
            if shell.create_lnk(shortcut).ok().is_none() {
                println!("Failed to create start menu shortcut");
            }
        }
    }
}

fn create_startup_shortcut(destination_file: &PathBuf)
{
    if let Some(user_dirs) = UserDirs::new() {
        let shortcut = user_dirs.home_dir()
            .join("AppData")
            .join("Roaming")
            .join("Microsoft")
            .join("Windows")
            .join("Start Menu")
            .join("Programs")
            .join("Startup")
            .join("Pricing App.lnk");
        let shell = ShellLink::new(destination_file).unwrap();
        if shell.create_lnk(shortcut).ok().is_none() {
            println!("Failed to create startup shortcut");
        }
    }
}

fn create_desktop_shortcut(destination_file: &PathBuf)
{
    if let Some(user_dirs) = UserDirs::new() {
        let shortcut = user_dirs.desktop_dir().unwrap()
            .join("Pricing App.lnk");
        let shell = ShellLink::new(destination_file).unwrap();
        if shell.create_lnk(shortcut).ok().is_none() {
            println!("Failed to create desktop shortcut");
        }
    }
}
