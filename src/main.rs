use nix::sys::resource::{getrusage, Usage, UsageWho};
use nix::sys::signal::Signal;
use nix::sys::time::TimeValLike;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::ffi::OsString;
use std::os::unix::process::ExitStatusExt;
use std::process::{Command, ExitCode};
use std::time::{Duration, Instant};

fn main() -> ExitCode {
    let args: Vec<OsString> = std::env::args_os().collect();
    let mut cmd = Command::new(&args[1]);
    cmd.args(&args[2..]);

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
