#!/bin/sh

set -eu

usage() {
    echo "usage: $0 DOMAIN {A IP | ANAME FQDN}" >&2
    exit 2
}

valid_domain() {
    printf '%s\n' "$1" | awk '
        length($0) > 253 { exit 1 }
        !/^[A-Za-z0-9][A-Za-z0-9.-]*[A-Za-z0-9]$/ { exit 1 }
        /\.\./ { exit 1 }
        {
            count = split($0, labels, ".")
            if (count < 2) exit 1
            for (i = 1; i <= count; i++) {
                if (length(labels[i]) > 63 || labels[i] ~ /^-/ || labels[i] ~ /-$/)
                    exit 1
            }
        }
    '
}

[ "$#" -eq 3 ] || usage

domain=${1%.}
record_type=$(printf '%s' "$2" | tr '[:lower:]' '[:upper:]')
target=${3%.}
script_dir=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
data_file=$script_dir/../rgbdns.data

if ! valid_domain "$domain"; then
    echo "invalid domain: $1" >&2
    exit 2
fi

domain=$(printf '%s' "$domain" | tr '[:upper:]' '[:lower:]')

case $record_type in
    A)
        if ! printf '%s\n' "$target" | awk -F. '
            NF != 4 { exit 1 }
            {
                for (i = 1; i <= 4; i++) {
                    if ($i !~ /^[0-9]+$/ || $i < 0 || $i > 255)
                        exit 1
                }
            }
        '; then
            echo "invalid IPv4 address: $target" >&2
            exit 2
        fi
        record="+$domain:$target:300"
        ;;
    ANAME)
        if ! valid_domain "$target"; then
            echo "invalid ANAME target: $3" >&2
            exit 2
        fi
        target=$(printf '%s' "$target" | tr '[:upper:]' '[:lower:]')
        record="A$domain:$target:300"
        ;;
    *)
        echo "unsupported record type: $2" >&2
        usage
        ;;
esac

if grep -Fq "Z${domain}:a.ns.cron.sh:hostmaster.cron.sh:" "$data_file"; then
    echo "domain already exists in rgbdns.data: $domain" >&2
    exit 1
fi

serial=$(date +%Y%m%d01)

if [ -s "$data_file" ] && [ "$(tail -c 1 "$data_file" | wc -l | tr -d ' ')" -eq 0 ]; then
    printf '\n' >>"$data_file"
fi

cat >>"$data_file" <<EOF

# $domain
Z$domain:a.ns.cron.sh:hostmaster.cron.sh:$serial:16384:2048:1048576:2560:3600
&$domain::a.ns.cron.sh:3600
&$domain::b.ns.cron.sh:3600
$record
EOF

echo "added $domain to $data_file"
