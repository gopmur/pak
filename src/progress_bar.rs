use std::{
    sync::{Arc, Mutex},
    thread, time::Duration,
};

use indicatif::{ProgressBar, ProgressStyle};

pub fn start_progress_bar(progress: Arc<Mutex<u64>>, total: u64) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        println!();
        let bar = ProgressBar::new(total);
        bar.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] {spinner} {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
            )
            .unwrap(),
        );
        bar.enable_steady_tick(Duration::from_millis(100));
        loop {
            let progress = *progress.lock().unwrap();
            bar.set_position(progress);
            thread::sleep(Duration::from_millis(50));
            if total <= progress {
                break;
            }
        }
    })
}
