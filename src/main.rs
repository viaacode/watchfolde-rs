use std::env;
use std::path::PathBuf;
use clap::Parser;

use inotify::{
    EventMask,
    WatchMask,
    Inotify,
};

// CLI parsing
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// path: filepath to watch
    #[clap(short, long)]
    filepath: String,
}


fn main() {
    //
    env_logger::init();
    // Args
    let args = Args::parse();

    let mut inotify = Inotify::init()
        .expect("Failed to initialize inotify");

    //~ let current_dir = env::current_dir()
        //~ .expect("Failed to determine current directory");

    let directory_to_watch = PathBuf::from(&args.filepath);

    inotify
        .add_watch(
            directory_to_watch,
            WatchMask::MODIFY | WatchMask::CREATE | WatchMask::DELETE | WatchMask::MOVED_TO,
        )
        .expect("Failed to add inotify watch");

    log::info!("Starting watchfolder for '{}'", &args.filepath);

    let mut buffer = [0u8; 4096];
    loop {
        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("Failed to read inotify events");

        for event in events {
            if event.mask.contains(EventMask::CREATE) {
                if event.mask.contains(EventMask::ISDIR) {
                    log::info!("Directory created: {:?}", event.name);
                } else {
                    log::info!("File created: {:?}", event.name);
                }
            } else if event.mask.contains(EventMask::DELETE) {
                if event.mask.contains(EventMask::ISDIR) {
                    log::info!("Directory deleted: {:?}", event.name);
                } else {
                    log::info!("File deleted: {:?}", event.name);
                }
            } else if event.mask.contains(EventMask::MODIFY) {
                if event.mask.contains(EventMask::ISDIR) {
                    log::info!("Directory modified: {:?}", event.name);
                } else {
                    log::info!("File modified: {:?}", event.name);
                }
            } else if event.mask.contains(EventMask::MOVED_TO) {
                if event.mask.contains(EventMask::ISDIR) {
                    log::info!("Directory moved: {:?}", event.name);
                } else {
                    log::info!("File moved: {:?}", event.name);
                }
            }
        }
    }
}
