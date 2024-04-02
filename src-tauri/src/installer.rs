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
}

async fn download_exe(directory: PathBuf) -> PathBuf
{
    // create directory
    if directory.exists()
    {
        fs::remove_dir_all(&directory).unwrap();
    }
    if let Err(_) = fs::create_dir_all(&directory) {
        // launch as administrator
        let current_exe = std::env::current_exe().unwrap();
        crate::process_utility::run_cmd_as_admin(&current_exe.to_string_lossy(), &["--create-directory", &directory.to_string_lossy()]);
    } else {
        if let Ok(mut child) = std::process::Command::new("cacls").args(&[&directory.to_string_lossy(), "/t", "/e", "/g", "Everyone:f"]).spawn() {
            child.wait().unwrap();
        }
    }

    // download file
    let url = "https://pricing-new.mardens.com/api/clients/latest";
    let destination = directory.join("pricing-app.exe");
    if let Ok(response) = reqwest::get(url).await {
        let mut file = std::fs::File::create(&destination).unwrap();
        let content = response.bytes().await.unwrap();
        file.write_all(&content).unwrap();
    }

    return destination;
}

fn create_start_menu_shortcut(destination_file: &PathBuf)
{
    if let Some(user_dirs) = UserDirs::new() {
        let shortcut = user_dirs.home_dir()
            .join("AppData")
            .join("Roaming")
            .join("Microsoft")
            .join("Windows")
            .join("Start Menu")
            .join("Programs")
            .join("Mardens Pricing App")
            .join("Pricing App.lnk");
        let shell = ShellLink::new(destination_file).unwrap();
        shell.create_lnk(shortcut).unwrap()
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
        shell.create_lnk(shortcut).unwrap()
    }
}

fn create_desktop_shortcut(destination_file: &PathBuf)
{
    if let Some(user_dirs) = UserDirs::new() {
        let shortcut = user_dirs.desktop_dir().unwrap()
            .join("Pricing App.lnk");
        let shell = ShellLink::new(destination_file).unwrap();
        shell.create_lnk(shortcut).unwrap()
    }
}
