#![no_std]

use core::fmt::Write;
use core::cmp::{max, min};
use libtock::result::TockResult;
use libtock::syscalls;

fn measure_latency() -> TockResult<usize> {
    let tic = syscalls::command(0xEFEFEFEF, 0, 0, 0)?;
    let toc = syscalls::command(0xEFEFEFEF, 0, 0, 0)?;
    Ok(toc - tic)
}

#[libtock::main]
async fn main() -> TockResult<()> {
    let drivers = libtock::retrieve_drivers()?;
    let mut console = drivers.console.create_console();
    writeln!(console, "latency: running\r")?;

    loop {
        // Measure latency
        let mut min_latency: usize = usize::max_value();
        let mut max_latency: usize = 0;
        for _i in 0..100 {
            let latency = measure_latency()?;
            min_latency = min(min_latency, latency);
            max_latency = max(max_latency, latency);
        }
        writeln!(console, "latency: System call latency: min: {} cycles, max: {} cycles.\r", min_latency, max_latency)?;
    }
}
