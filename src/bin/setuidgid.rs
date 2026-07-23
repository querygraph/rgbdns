#[cfg(unix)]
fn main() {
    if let Err(error) = run() {
        eprintln!("setuidgid: fatal: {error}");
        std::process::exit(111);
    }
}

#[cfg(unix)]
fn run() -> Result<(), String> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    let account = arguments
        .first()
        .ok_or_else(|| "usage: setuidgid account program [arg ...]".to_owned())?;
    let identity = rgbdns::setuidgid::resolve(account)?;
    let (program, arguments) = rgbdns::setuidgid::command(&arguments[1..])?;
    rgbdns::setuidgid::drop_privileges(&identity)?;
    rgbdns::setuidgid::exec(&program, &arguments)
}

#[cfg(not(unix))]
fn main() {
    eprintln!("setuidgid: fatal: this platform does not support Unix identities");
    std::process::exit(111);
}
