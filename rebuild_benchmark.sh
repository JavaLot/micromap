#!/bin/bash
# SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
# SPDX-License-Identifier: MIT

set -x
set -e

rm -rf src/bin
mkdir -p src/bin
cp tests/benchmark.rs src/bin/benchmark.rs

sed -E -i 's/\[dev-dependencies\]//g' Cargo.toml

micromap="micromap::Map"
capacities="2 4 8 16 32 64 128"
cycles=1000000
first=$(echo "${capacities}" | cut -f1 -d' ')

rm -rf target/benchmark
mkdir -p target/benchmark
SECONDS=0
for capacity in ${capacities}; do
    sed -E -i "s/CAPACITY: usize = [0-9]+/CAPACITY: usize = ${capacity}/g" \
        src/bin/benchmark.rs
    cargo build --release
    ./target/release/benchmark "${cycles}" > "target/benchmark/${capacity}.out"
done

{
    echo -n '| |'
    for capacity in ${capacities}; do
        echo -n " ${capacity} |"
    done
    echo ''
    echo -n '| --- |'
    for capacity in ${capacities}; do
        echo -n " --: |"
    done
    echo ''
    maps=$(cut -f 1 "target/benchmark/${first}.out")
    for map in ${maps}; do
        echo -n "| \`${map}\`"
        if [ "${map}" == "${micromap}" ]; then
            echo -n ' 👍'
        fi
        echo -n ' |'
        for capacity in ${capacities}; do
            our=$(grep "${micromap}" "target/benchmark/${capacity}.out" \
                | cut -f 2)
            if [ "${our}" -eq "0" ]; then
                our=1
            fi
            their=$(grep "${map}" "target/benchmark/${capacity}.out" \
                | cut -f 2)
            if [ "${their}" -eq "0" ]; then
                their=1
            fi
            echo -n ' '
            if [ $(( "${their}" / "${our}" / 1000 / 1000 )) -gt 0 ]; then
                perl -e "printf(\"%dM\", ${their} / ${our} / 1000 / 1000);"
            elif [ $(( "${their}" / "${our}" / 1000 )) -gt 0 ]; then
                perl -e "printf(\"%dK\", ${their} / ${our} / 1000);"
            else
                perl -e "printf(\"%.02f\", ${their} / ${our});"
            fi
            echo -n ' |'
        done
        echo ''
    done
    echo ''
    echo "The experiment [was performed][action] on $(date +%d-%m-%Y)."
    echo "There were ${cycles} repetition cycles."
    echo "The entire benchmark took ${SECONDS}s."
    echo "Uname: '$(uname)'."
} > target/benchmark/table.md

perl -e '
    my $readme;
    my $file = "README.md";
    open(my $r, "<", $file);
    { local $/; $readme = <$r>; }
    close($r);
    my $sep = "<!-- benchmark -->";
    my @p = split(/\Q$sep\E/, $readme);
    my $table = "target/benchmark/table.md";
    open(my $t, "<", $table);
    { local $/; $table = <$t>; }
    close($t);
    $p[1] = "\n" . $table . "\n";
    my $new = join($sep, @p);
    open(my $w, ">", $file);
    print $w join($sep, @p);
    close($w);
'

git restore Cargo.toml
rm -rf src/bin
