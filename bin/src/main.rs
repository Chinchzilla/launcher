use pop_launcher_plugins as plugins;
use pop_launcher_service as service;
use smol::block_on;
use std::io;

fn main() {
    tracing_subscriber::fmt()
        .with_writer(io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    std::env::args();

    if let Some(plugin) = std::env::args().next() {
        let start = plugin.rfind('/').map(|v| v + 1).unwrap_or(0);
        let cmd = &plugin.as_str()[start..];
        match cmd {
            "desktop-entries" => block_on(plugins::desktop_entries::main()),
            "find" => block_on(plugins::find::main()),
            "pop-launcher" => block_on(service::main()),
            "pop-shell" => block_on(plugins::pop_shell::main()),
            "scripts" => block_on(plugins::scripts::main()),
            "web" => block_on(plugins::web::main()),
            unknown => {
                eprintln!("unknown cmd: {}", unknown);
            }
        }
    }
}
