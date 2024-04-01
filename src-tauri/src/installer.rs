use std::io::Write;
use std::path::PathBuf;

use directories::UserDirs;
use mslnk::ShellLink;

async fn install(location: String, create_desktop_shortcut: bool, create_start_menu_shortcut: bool, start_with_windows: bool)
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

pub fn start_application(exe: String)
{
    std::process::Command::new(exe).spawn().unwrap();
}

async fn download_exe(directory: PathBuf) -> PathBuf
{
    // create directory
    std::fs::create_dir_all(&directory).unwrap();
    // download file
    let url = "https://pricing-new.mardens.com/clients/latest";
    let destination = directory.join("pricing-app.exe");
    if let Ok(response) = reqwest::get(&url).await {
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
        let shortcut = user_dirs.desktop_dir()
            .join("Pricing App.lnk");
        let shell = ShellLink::new(destination_file).unwrap();
        shell.create_lnk(shortcut).unwrap()
    }
}