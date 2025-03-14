use sysinfo::{System, SystemExt, ProcessExt};
use std::{thread, time::Duration};

fn main() {
    let mut sys = System::new_all();

    loop {
        sys.refresh_processes();
        let mut processes: Vec<_> = sys.processes()
            .into_iter()
            .map(|(_pid, process)| process)
            .collect();

        processes.sort_by(|a, b| b.memory().cmp(&a.memory()));

        println!("En çok RAM kullanan süreçler:");

        for process in processes.iter().take(10) {

            let mem = process.memory() as f64 / 1024.0 / 1024.0;

            println!(
                "Process: {}, PID: {}, RAM: {:.2} MB",
                process.name(),
                process.pid(),
                mem
            );

        }

        thread::sleep(Duration::from_secs(3));
        println!("\n");
    }
}
