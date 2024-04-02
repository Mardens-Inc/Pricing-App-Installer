use std::mem::size_of;
use std::process::exit;

use windows::core::{Error, PCWSTR, w};
use windows::core::imp::WaitForSingleObject;
use windows::Win32::Foundation::{CloseHandle, WAIT_OBJECT_0};
use windows::Win32::System::Threading::{GetExitCodeProcess, INFINITE};
use windows::Win32::UI::Shell::{SEE_MASK_NOCLOSEPROCESS, ShellExecuteExW, SHELLEXECUTEINFOW};
use windows::Win32::UI::WindowsAndMessaging::SW_NORMAL;

pub fn run_cmd_as_admin(cmd: &str, args: &[&str])
{
    // join args by space
    let args = args.join(" ");
    let cmd = PCWSTR::from_raw(cmd.encode_utf16().collect::<Vec<u16>>().as_ptr());
    let args = PCWSTR::from_raw(args.encode_utf16().collect::<Vec<u16>>().as_ptr());

    let mut sh_exec_info = SHELLEXECUTEINFOW {
        cbSize: size_of::<SHELLEXECUTEINFOW>() as u32,
        fMask: SEE_MASK_NOCLOSEPROCESS,
        lpVerb: w!("runas"),
        lpFile: cmd,
        lpParameters: args,
        nShow: SW_NORMAL.0 as i32,
        ..SHELLEXECUTEINFOW::default()
    };
    if let Err(e) = unsafe { ShellExecuteExW(&mut sh_exec_info) } {
        eprintln!("ShellExecuteExW: {e:?}");
        exit(1);
    }
    let r = unsafe { WaitForSingleObject(sh_exec_info.hProcess.0, INFINITE) };
    if r != WAIT_OBJECT_0.0 {
        eprintln!("WaitForSingleObject: {:?}", Error::from_win32());
        exit(1);
    }
    let mut status = 0u32;
    if let Err(_) = unsafe { GetExitCodeProcess(sh_exec_info.hProcess, &mut status) } {
        exit(1);
    }
    unsafe { CloseHandle(sh_exec_info.hProcess).unwrap(); }
}