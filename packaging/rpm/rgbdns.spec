Name:           rgbdns
Version:        0.6.0
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
Suggests:       postfix
Provides:       group(rgbdns)
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
%if 0%{?rgbdns_skip_rust_build}
test -x target/release/rgbdns
%else
cargo build --release --locked --bins
cargo test --release --locked --all-targets
%endif

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
install -D -m 0755 packaging/scripts/import-data \
    %{buildroot}%{_prefix}/lib/rgbdns/import-data
install -D -m 0755 packaging/scripts/migrate-zones \
    %{buildroot}%{_prefix}/lib/rgbdns/migrate-zones
install -D -m 0755 packaging/scripts/migrate-zone-drop \
    %{buildroot}%{_prefix}/lib/rgbdns/migrate-zone-drop
install -D -m 0755 packaging/scripts/migrate-zone-state \
    %{buildroot}%{_prefix}/lib/rgbdns/migrate-zone-state
install -D -m 0755 packaging/scripts/restore-role-units \
    %{buildroot}%{_prefix}/lib/rgbdns/restore-role-units
install -D -m 0755 packaging/scripts/daily-query-report \
    %{buildroot}%{_prefix}/lib/rgbdns/daily-query-report
install -D -m 0755 packaging/scripts/publish-dnssec \
    %{buildroot}%{_prefix}/lib/rgbdns/publish-dnssec
install -D -m 0755 packaging/scripts/rgbdns-setup \
    %{buildroot}%{_sbindir}/rgbdns-setup
install -D -m 0640 packaging/default/tinydns.env \
    %{buildroot}%{_sysconfdir}/rgbdns/tinydns.env
install -D -m 0640 packaging/default/acme-update.conf \
    %{buildroot}%{_sysconfdir}/rgbdns/acme-update.conf
install -D -m 0640 packaging/default/query-report.env \
    %{buildroot}%{_sysconfdir}/rgbdns/query-report.env
install -D -m 0644 packaging/default/data \
    %{buildroot}%{_docdir}/%{name}/examples/data
for unit in packaging/systemd/*; do
    install -D -m 0644 "$unit" \
        "%{buildroot}%{_unitdir}/$(basename "$unit")"
done
install -D -m 0644 debian/rgbdns.7 \
    %{buildroot}%{_mandir}/man7/rgbdns.7
install -D -m 0644 man/rgbdns-acme.1 \
    %{buildroot}%{_mandir}/man1/rgbdns-acme.1
install -D -m 0644 man/rgbdns-log-report.1 \
    %{buildroot}%{_mandir}/man1/rgbdns-log-report.1
for manual in man/acme-materialize.1 man/aname-materialize.1 \
    man/dnssec-check.1 man/dnssec-data.1 man/dnssec-ds.1 \
    man/dnssec-keygen.1 man/dnssec-sign.1; do
    install -D -m 0644 "$manual" \
        "%{buildroot}%{_mandir}/man1/$(basename "$manual")"
done

%pre
getent group rgbdns >/dev/null 2>&1 || groupadd --system rgbdns
getent passwd rgbdns >/dev/null 2>&1 || \
    useradd --system --gid rgbdns --home-dir /var/lib/rgbdns \
        --shell /usr/sbin/nologin --comment "rgbdns service" rgbdns
exit 0

%post
install -d -o root -g rgbdns -m 0750 %{_sysconfdir}/rgbdns
install -d -o root -g root -m 0700 %{_sysconfdir}/rgbdns/keys
install -d -o rgbdns -g rgbdns -m 0750 /var/lib/rgbdns/tinydns
chown root:rgbdns %{_sysconfdir}/rgbdns/tinydns.env
chmod 0640 %{_sysconfdir}/rgbdns/tinydns.env
chown root:rgbdns %{_sysconfdir}/rgbdns/acme-update.conf
chmod 0640 %{_sysconfdir}/rgbdns/acme-update.conf
chown root:rgbdns %{_sysconfdir}/rgbdns/query-report.env
chmod 0640 %{_sysconfdir}/rgbdns/query-report.env
%{_prefix}/lib/rgbdns/migrate-zones
%{_prefix}/lib/rgbdns/migrate-zone-state
%{_prefix}/lib/rgbdns/migrate-zone-drop
%service_add_post rgbdns-tinydns.service rgbdns-secondary-sync.service rgbdns-secondary-sync.timer rgbdns-zones-import.service rgbdns-zones.path rgbdns-data-import.service rgbdns-data.path rgbdns-query-report.service rgbdns-query-report.timer rgbdns-dnssec-publish.service rgbdns-dnssec-publish.timer rgbdns-dnssec-check.service rgbdns-dnssec-check.timer
if [ "$1" -gt 1 ]; then
    %{_prefix}/lib/rgbdns/restore-role-units
fi

%preun
%service_del_preun rgbdns-tinydns.service rgbdns-secondary-sync.service rgbdns-secondary-sync.timer rgbdns-zones-import.service rgbdns-zones.path rgbdns-data-import.service rgbdns-data.path rgbdns-query-report.service rgbdns-query-report.timer rgbdns-dnssec-publish.service rgbdns-dnssec-publish.timer rgbdns-dnssec-check.service rgbdns-dnssec-check.timer

%postun
%service_del_postun rgbdns-tinydns.service rgbdns-secondary-sync.service rgbdns-secondary-sync.timer rgbdns-zones-import.service rgbdns-zones.path rgbdns-data-import.service rgbdns-data.path rgbdns-query-report.service rgbdns-query-report.timer rgbdns-dnssec-publish.service rgbdns-dnssec-publish.timer rgbdns-dnssec-check.service rgbdns-dnssec-check.timer

%files
%doc README.md docs/OPENSUSE.md docs/DNSSEC.md docs/DNSSEC-DESIGN.md
%{_bindir}/*
%{_sbindir}/rgbdns-setup
%dir %{_prefix}/lib/rgbdns
%{_prefix}/lib/rgbdns/compile-zone
%{_prefix}/lib/rgbdns/secondary-sync
%{_prefix}/lib/rgbdns/import-zones
%{_prefix}/lib/rgbdns/import-data
%{_prefix}/lib/rgbdns/migrate-zones
%{_prefix}/lib/rgbdns/migrate-zone-drop
%{_prefix}/lib/rgbdns/migrate-zone-state
%{_prefix}/lib/rgbdns/restore-role-units
%{_prefix}/lib/rgbdns/daily-query-report
%{_prefix}/lib/rgbdns/publish-dnssec
%attr(0750,root,rgbdns) %dir %{_sysconfdir}/rgbdns
%attr(0640,root,rgbdns) %config(noreplace) %{_sysconfdir}/rgbdns/tinydns.env
%attr(0640,root,rgbdns) %config(noreplace) %{_sysconfdir}/rgbdns/acme-update.conf
%attr(0640,root,rgbdns) %config(noreplace) %{_sysconfdir}/rgbdns/query-report.env
%{_unitdir}/rgbdns-tinydns.service
%{_unitdir}/rgbdns-secondary-sync.service
%{_unitdir}/rgbdns-secondary-sync.timer
%{_unitdir}/rgbdns-zones-import.service
%{_unitdir}/rgbdns-zones.path
%{_unitdir}/rgbdns-data-import.service
%{_unitdir}/rgbdns-data.path
%{_unitdir}/rgbdns-query-report.service
%{_unitdir}/rgbdns-query-report.timer
%{_unitdir}/rgbdns-dnssec-publish.service
%{_unitdir}/rgbdns-dnssec-publish.timer
%{_unitdir}/rgbdns-dnssec-check.service
%{_unitdir}/rgbdns-dnssec-check.timer
%{_docdir}/%{name}/examples/data
%{_mandir}/man7/rgbdns.7%{?ext_man}
%{_mandir}/man1/rgbdns-acme.1%{?ext_man}
%{_mandir}/man1/rgbdns-log-report.1%{?ext_man}
%{_mandir}/man1/acme-materialize.1%{?ext_man}
%{_mandir}/man1/aname-materialize.1%{?ext_man}
%{_mandir}/man1/dnssec-check.1%{?ext_man}
%{_mandir}/man1/dnssec-data.1%{?ext_man}
%{_mandir}/man1/dnssec-ds.1%{?ext_man}
%{_mandir}/man1/dnssec-keygen.1%{?ext_man}
%{_mandir}/man1/dnssec-sign.1%{?ext_man}

%changelog
* Tue Aug 18 2026 Alexy Khrabrov <deliverable@gmail.com> - 0.6.0-1
- Add opt-in offline authoritative DNSSEC signing and denial proofs
- Compose ACME and ANAME inputs into verified atomic signed snapshots
- Keep authority and secondaries keyless; transfer DNSSEC over standard AXFR
- Support explicit mixed signed and unsigned zones in one CDB

* Thu Aug 07 2026 Alexy Khrabrov <deliverable@gmail.com> - 0.5.3-1
- Add a monospace HTML view to daily query report email

* Thu Aug 06 2026 Alexy Khrabrov <deliverable@gmail.com> - 0.5.2-1
- Render daily query report email bodies with a monospace HTML view and a
  fixed-format plain-text fallback
- Add fast Debian and RPM repackaging without rebuilding Rust binaries

* Wed Aug 05 2026 Alexy Khrabrov <deliverable@gmail.com> - 0.5.1-1
- Avoid requiring the optional hostname utility for report delivery

* Wed Aug 05 2026 Alexy Khrabrov <deliverable@gmail.com> - 0.5.0-1
- Add daily per-zone total and unique-client query reports
- Add configurable sendmail delivery through a hardened systemd timer

* Sun Aug 02 2026 Alexy Khrabrov <deliverable@gmail.com> - 0.4.0-1
- Add scoped TSIG-authenticated RFC 2136 updates for ACME DNS-01
- Persist challenge TXT state and transfer it with monotonic SOA serials
- Add the rgbdns-acme administrative and manual-hook client

* Sat Aug 01 2026 Alexy Khrabrov <deliverable@gmail.com> - 0.3.6-1
- Move the activated secondary zone list from /etc into writable managed state

* Sat Aug 01 2026 Alexy Khrabrov <deliverable@gmail.com> - 0.3.5-1
- Keep the zone-list staging descriptor open under SELinux enforcement

* Sat Aug 01 2026 Alexy Khrabrov <deliverable@gmail.com> - 0.3.4-1
- Move secondary zone-list pickup out of SELinux-protected home directories
- Migrate existing home-directory drop configuration during package upgrades

* Fri Jul 31 2026 Alexy Khrabrov <deliverable@gmail.com> - 0.3.3-1
- Restart authority for an existing configured role during package upgrades

* Thu Jul 30 2026 Alexy Khrabrov <deliverable@gmail.com> - 0.3.2-2
- Provide the rgbdns group capability required by packaged file ownership

* Thu Jul 30 2026 Alexy Khrabrov <deliverable@gmail.com> - 0.3.2-1
- Preserve rgbdns group access to configuration during RPM upgrades
- Avoid reopening the already-created zone-list staging file

* Thu Jul 30 2026 Alexy Khrabrov <deliverable@gmail.com> - 0.3.1-1
- Restore configured primary or secondary picker units on package upgrade

* Thu Jul 30 2026 Alexy Khrabrov <deliverable@gmail.com> - 0.3.0-1
- Preserve ANAME directives between upgraded rgbdns AXFR peers
- Coalesce ANAME lookups and suppress immediate retries after failures
- Publish the ANAME and zone-transfer Internet-Draft

* Thu Jul 30 2026 Alexy Khrabrov <deliverable@gmail.com> - 0.2.3-1
- Negotiate and preserve private ANAME directives between rgbdns AXFR peers
- Keep standard AXFR output free of rgbdns private transfer metadata

* Thu Jul 30 2026 Alexy Khrabrov <deliverable@gmail.com> - 0.2.2-1
- Validate, compile, and atomically activate primary rgbdns.data drops

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
