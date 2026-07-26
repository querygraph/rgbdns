Name:           rgbdns
Version:        0.1.1
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
install -d -o root -g rgbdns -m 0750 %{_sysconfdir}/rgbdns
install -d -o rgbdns -g rgbdns -m 0750 /var/lib/rgbdns/tinydns
chown root:rgbdns %{_sysconfdir}/rgbdns/tinydns.env
chmod 0640 %{_sysconfdir}/rgbdns/tinydns.env
%service_add_post rgbdns-tinydns.service rgbdns-secondary-sync.service rgbdns-secondary-sync.timer

%preun
%service_del_preun rgbdns-tinydns.service rgbdns-secondary-sync.service rgbdns-secondary-sync.timer

%postun
%service_del_postun rgbdns-tinydns.service rgbdns-secondary-sync.service rgbdns-secondary-sync.timer

%files
%doc README.md docs/OPENSUSE.md
%{_bindir}/*
%{_sbindir}/rgbdns-setup
%dir %{_prefix}/lib/rgbdns
%{_prefix}/lib/rgbdns/compile-zone
%{_prefix}/lib/rgbdns/secondary-sync
%dir %{_sysconfdir}/rgbdns
%attr(0640,root,rgbdns) %config(noreplace) %{_sysconfdir}/rgbdns/tinydns.env
%{_unitdir}/rgbdns-tinydns.service
%{_unitdir}/rgbdns-secondary-sync.service
%{_unitdir}/rgbdns-secondary-sync.timer
%{_docdir}/%{name}/examples/data
%{_mandir}/man7/rgbdns.7%{?ext_man}

%changelog
* Sun Jul 26 2026 Alexy Khrabrov <deliverable@gmail.com> - 0.1.1-1
- Add native openSUSE Leap package and hardened systemd deployment
