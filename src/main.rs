use std::{
    env,
    process::{exit, Command},
};

use winapi::{
    shared::minwindef::{BOOL, DWORD, FALSE, TRUE},
    um::{consoleapi, wincon},
};

unsafe extern "system" fn routine_handler(evt: DWORD) -> BOOL {
    match evt {
        wincon::CTRL_C_EVENT => TRUE,        //eprintln!("ctrl_c handled!"),
        wincon::CTRL_BREAK_EVENT => TRUE,    //eprintln!("ctrl_break handled!"),
        wincon::CTRL_CLOSE_EVENT => TRUE,    //eprintln!("ctrl_close handled!"),
        wincon::CTRL_LOGOFF_EVENT => TRUE,   //eprintln!("ctrl_logoff handled!"),
        wincon::CTRL_SHUTDOWN_EVENT => TRUE, //eprintln!("ctrl_shutdown handled!"),
        other => {
            eprintln!("unknown event number: {}, unhandled!", other);
            return FALSE;
        }
    }
}

mod shims;
use shims::Shim;

const EXIT_FAILED_LOAD_SHIM: i32 = 1;
const EXIT_FAILED_SPAWN_PROG: i32 = 2;
const EXIT_FAILED_WAIT_PROG: i32 = 3;
const EXIT_PROG_TERMINATED: i32 = 4;

fn main() {
    let res: BOOL = unsafe { consoleapi::SetConsoleCtrlHandler(Some(routine_handler), TRUE) };
    if res == FALSE {
        eprintln!("shim: register Ctrl handler failed.");
    }

    let calling_args: Vec<_> = env::args().skip(1).collect();
    let shim = match Shim::init() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error while loading shim: {}", e);
            exit(EXIT_FAILED_LOAD_SHIM);
        }
    };
    let args = if let Some(mut shim_args) = shim.args {
        shim_args.extend_from_slice(calling_args.as_slice());
        shim_args
    } else {
        calling_args
    };
    let mut cmd = match Command::new(&shim.target_path).args(&args).spawn() {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "Error while spawning target program `{}`: {}",
                shim.target_path.to_string_lossy(),
                e
            );
            exit(EXIT_FAILED_SPAWN_PROG);
        }
    };
    let status = match cmd.wait() {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "Error while waiting target program `{}`: {}",
                shim.target_path.to_string_lossy(),
                e
            );
            exit(EXIT_FAILED_WAIT_PROG);
        }
    };
    exit(status.code().unwrap_or(EXIT_PROG_TERMINATED))
}
