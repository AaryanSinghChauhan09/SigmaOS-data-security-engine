//! SigmaOS-data-security-engine — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_data_security_engine_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-data-security-engine: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-data-security-engine: ok");
}
