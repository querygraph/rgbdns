#[cfg(unix)]
mod unix {
    use std::{
        fs,
        path::Path,
        process::{Command, Output},
        time::{SystemTime, UNIX_EPOCH},
    };

    fn run(program: &str, arguments: &[&str], directory: &Path) -> Output {
        let output = Command::new(program)
            .args(arguments)
            .current_dir(directory)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{} failed: {}",
            program,
            String::from_utf8_lossy(&output.stderr)
        );
        output
    }

    #[test]
    fn small_dnssec_tools_form_a_verified_fail_closed_pipeline() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let directory = std::env::temp_dir().join(format!(
            "rgbdns-dnssec-tools-{}-{unique}",
            std::process::id()
        ));
        fs::create_dir(&directory).unwrap();
        fs::write(
            directory.join("data"),
            "Zexample:ns.example:hostmaster.example:7:3600:600:86400:300:300\n\
             &example:192.0.2.53:ns.example:300\n\
             +www.example:192.0.2.1:300\n\
             Zlegacy.example:ns.legacy.example:hostmaster.legacy.example:8:3600:600:86400:300:300\n\
             &legacy.example:192.0.2.54:ns.legacy.example:300\n\
             Alegacy.example:target.example:120\n",
        )
        .unwrap();
        let key = directory.join("example.pk8");
        let keygen = run(
            env!("CARGO_BIN_EXE_rgbsec-keygen"),
            &["example", key.to_str().unwrap()],
            &directory,
        );
        let mut policy = keygen.stdout;
        policy.extend_from_slice(b"Ulegacy.example.\n");
        fs::write(directory.join("dnssec"), policy).unwrap();

        run(
            env!("CARGO_BIN_EXE_rgbsec-sign"),
            &["data", "data.signed"],
            &directory,
        );
        let check = run(
            env!("CARGO_BIN_EXE_rgbsec-check"),
            &["data.signed", "dnssec"],
            &directory,
        );
        assert!(String::from_utf8(check.stdout).unwrap().ends_with("\tok\n"));

        let signed = fs::read_to_string(directory.join("data.signed")).unwrap();
        assert!(signed.contains("Alegacy.example:target.example:120"));
        fs::write(
            directory.join("data.tampered"),
            signed.replace("192.0.2.1", "192.0.2.2"),
        )
        .unwrap();
        let tampered = Command::new(env!("CARGO_BIN_EXE_rgbsec-check"))
            .args(["data.tampered", "dnssec"])
            .current_dir(&directory)
            .output()
            .unwrap();
        assert!(!tampered.status.success());

        run(
            env!("CARGO_BIN_EXE_rgbsec-data"),
            &["data", "data.cdb"],
            &directory,
        );
        run(
            env!("CARGO_BIN_EXE_rgbsec-check"),
            &["data.cdb", "dnssec"],
            &directory,
        );
        fs::remove_dir_all(directory).unwrap();
    }
}
