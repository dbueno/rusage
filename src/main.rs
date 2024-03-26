use nix::sys::resource::{getrusage, Usage, UsageWho};
use nix::sys::signal::Signal;
use nix::sys::time::TimeValLike;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::ffi::OsString;
use std::os::unix::process::ExitStatusExt;
use std::process::{Command, ExitCode, Stdio};
use std::time::{Duration, Instant};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() -> ExitCode {
    let args: Vec<OsString> = std::env::args_os().collect();

    if args.len() < 2 {
        eprintln!("error: no command to run\n");
        return usage();
    }
    let mut quiet = false;
    let mut args_start = 1;
    if args[1] == "-q" {
        quiet = true;
        args_start = 2;
    }
    let mut cmd = Command::new(&args[args_start]);
    cmd.args(&args[args_start+1..]);
    let cmd = if quiet { cmd.stdout(Stdio::null()).stderr(Stdio::null()) } else { &mut cmd };

    let start_instant = Instant::now();
    let child_status = cmd.status().expect("Could not start child");
    let end_instant = Instant::now();

    let wall_time = end_instant - start_instant;

    let child_usage = getrusage(UsageWho::RUSAGE_CHILDREN).expect("Could not get resource usage");

    let (exit_code, message) = if let Some(child_sig) = child_status.signal() {
        let msg = if let Ok(child_sig) = Signal::try_from(child_sig) {
            format!("Program terminated with signal: {}", child_sig)
        } else {
            format!("Program terminated with signal: {}", child_sig)
        };
        (127 - child_sig, Some(msg))
    } else if let Some(child_code) = child_status.code() {
        let msg = if child_code != 0 {
            Some(format!(
                "Program terminated with non-zero status: {}",
                child_code
            ))
        } else {
            None
        };
        (child_code, msg)
    } else {
        panic!("Unknown kind of termination: {}", child_status);
    };

    eprintln!();
    print_resources(wall_time, &child_usage);
    if let Some(message) = message {
        eprintln!("{}", message);
    }

    ExitCode::from(exit_code as u8)
}

fn usage() -> ExitCode {
    eprintln!("rusage {VERSION}");
    eprintln!("Usage: rusage [-q] command [args ...]");
    eprintln!("{DESCRIPTION}");
    eprintln!("-q: quiet, no stdout or stderr for command");
    return ExitCode::from(0)
}

fn print_resources(wall_time: Duration, ru: &Usage) {
    let user_time = Duration::from_micros(ru.user_time().num_microseconds().try_into().unwrap());
    let system_time =
        Duration::from_micros(ru.system_time().num_microseconds().try_into().unwrap());
    eprintln!("Wall time (secs):        {:.3}", wall_time.as_secs_f32());
    eprintln!(
        "CPU time (secs):         user={:.3}; system={:.3}",
        user_time.as_secs_f32(),
        system_time.as_secs_f32()
    );
    eprintln!("Max resident set size:   {}", ru.max_rss());
    eprintln!("Integral shared memory:  {}", ru.shared_integral());
    eprintln!("Integral unshared data:  {}", ru.unshared_data_integral());
    eprintln!("Integral unshared stack: {}", ru.unshared_stack_integral());
    eprintln!("Page reclaims:           {}", ru.minor_page_faults());
    eprintln!("Page faults:             {}", ru.major_page_faults());
    eprintln!("Swaps:                   {}", ru.full_swaps());
    eprintln!(
        "Block I/Os:              input={}; output={}",
        ru.block_reads(),
        ru.block_writes()
    );
    eprintln!("Signals received:        {}", ru.signals());
    eprintln!(
        "IPC messages:            sent={}; received={}",
        ru.ipc_sends(),
        ru.ipc_receives()
    );
    eprintln!(
        "Context switches:        voluntary={}; involuntary={}",
        ru.voluntary_context_switches(),
        ru.involuntary_context_switches()
    );
}
