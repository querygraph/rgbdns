Name:           rgbdns
Version:        0.2.1
Release:        1%{?dist}
Summary:        Memory-safe DNS server and djbdns-compatible tool suite
License:        Unlicense
URL:            https://github.com/querygraph/rgbdns
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  cargo
BuildRequires:  python3
BuildRequires:  rust
BuildRequires:  systemd-rpm-macros
Requires(pre):  shadow
Requires(post): systemd
Requires(preun): systemd
Requires(postun): systemd
Requires:       systemd
Requires:       util-linux
Conflicts:      daemontools
Conflicts:      djbdns
Obsoletes:      daemontools
Obsoletes:      djbdns
%{?systemd_requires}

%description
rgbdns provides authoritative DNS, recursive resolution, zone compilation,
AXFR transfers, diagnostic clients, and service tools implemented in Rust.
This package includes hardened systemd units and primary/secondary setup
automation.

%prep
%autosetup

%build
cargo build --release --locked --bins
cargo test --release --locked --all-targets

%install
install -d %{buildroot}%{_bindir}
cargo metadata --format-version 1 --no-deps | \
    python3 packaging/cargo-binaries.py | while read binary; do
        install -m 0755 "target/release/$binary" \
            "%{buildroot}%{_bindir}/$binary"
    done
install -D -m 0755 packaging/scripts/compile-zone \
    %{buildroot}%{_prefix}/lib/rgbdns/compile-zone
install -D -m 0755 packaging/scripts/secondary-sync \
    %{buildroot}%{_prefix}/lib/rgbdns/secondary-sync
install -D -m 0755 packaging/scripts/import-zones \
    %{buildroot}%{_prefix}/lib/rgbdns/import-zones
install -D -m 0755 packaging/scripts/migrate-zones \
    %{buildroot}%{_prefix}/lib/rgbdns/migrate-zones
install -D -m 0755 packaging/scripts/rgbdns-setup \
    %{buildroot}%{_sbindir}/rgbdns-setup
install -D -m 0640 packaging/default/tinydns.env \
    %{buildroot}%{_sysconfdir}/rgbdns/tinydns.env
install -D -m 0644 packaging/default/data \
    %{buildroot}%{_docdir}/%{name}/examples/data
for unit in packaging/systemd/*; do
    install -D -m 0644 "$unit" \
        "%{buildroot}%{_unitdir}/$(basename "$unit")"
done
install -D -m 0644 debian/rgbdns.7 \
    %{buildroot}%{_mandir}/man7/rgbdns.7

%pre
getent group rgbdns >/dev/null 2>&1 || groupadd --system rgbdns
getent passwd rgbdns >/dev/null 2>&1 || \
    useradd --system --gid rgbdns --home-dir /var/lib/rgbdns \
        --shell /usr/sbin/nologin --comment "rgbdns service" rgbdns
exit 0

%post
install -d -o rgbdns -g rgbdns -m 0750 /var/lib/rgbdns/tinydns
chmod 0640 %{_sysconfdir}/rgbdns/tinydns.env
%{_prefix}/lib/rgbdns/migrate-zones
%service_add_post rgbdns-tinydns.service rgbdns-secondary-sync.service rgbdns-secondary-sync.timer rgbdns-zones-import.service rgbdns-zones.path

%preun
%service_del_preun rgbdns-tinydns.service rgbdns-secondary-sync.service rgbdns-secondary-sync.timer rgbdns-zones-import.service rgbdns-zones.path

%postun
%service_del_postun rgbdns-tinydns.service rgbdns-secondary-sync.service rgbdns-secondary-sync.timer rgbdns-zones-import.service rgbdns-zones.path

%files
%doc README.md docs/OPENSUSE.md
%{_bindir}/*
%{_sbindir}/rgbdns-setup
%dir %{_prefix}/lib/rgbdns
%{_prefix}/lib/rgbdns/compile-zone
%{_prefix}/lib/rgbdns/secondary-sync
%{_prefix}/lib/rgbdns/import-zones
%{_prefix}/lib/rgbdns/migrate-zones
%attr(0750,root,root) %dir %{_sysconfdir}/rgbdns
%attr(0640,root,root) %config(noreplace) %{_sysconfdir}/rgbdns/tinydns.env
%{_unitdir}/rgbdns-tinydns.service
%{_unitdir}/rgbdns-secondary-sync.service
%{_unitdir}/rgbdns-secondary-sync.timer
%{_unitdir}/rgbdns-zones-import.service
%{_unitdir}/rgbdns-zones.path
%{_docdir}/%{name}/examples/data
%{_mandir}/man7/rgbdns.7%{?ext_man}

%changelog
* Thu Jul 30 2026 Alexy Khrabrov <deliverable@gmail.com> - 0.2.1-1
- Import validated secondary zone lists from an atomic scp drop
- Watch the configured drop file and trigger isolated AXFR synchronization

* Wed Jul 29 2026 Alexy Khrabrov <deliverable@gmail.com> - 0.2.0-1
- Restore original-compatible per-request tinydns logging
- Add QUERY_LOG opt-out for journald and multilog deployments

* Wed Jul 29 2026 Alexy Khrabrov <deliverable@gmail.com> - 0.1.1-5
- Isolate refresh failures with per-zone last-known-good snapshots

* Wed Jul 29 2026 Alexy Khrabrov <deliverable@gmail.com> - 0.1.1-4
- Synchronize and atomically activate multiple secondary zones

* Wed Jul 29 2026 Alexy Khrabrov <deliverable@gmail.com> - 0.1.1-3
- Create a writable runtime directory for secondary synchronization

* Wed Jul 29 2026 Alexy Khrabrov <deliverable@gmail.com> - 0.1.1-2
- Replace conflicting djbdns and daemontools packages cleanly

* Sun Jul 26 2026 Alexy Khrabrov <deliverable@gmail.com> - 0.1.1-1
- Add native openSUSE Leap package and hardened systemd deployment
