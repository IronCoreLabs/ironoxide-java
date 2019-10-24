#!/usr/bin/env bash

set -e
set -x

# Set JAVA_HOME inside our CentOS Docker container.
if [ -z "${JAVA_HOME}" -a -e /usr/lib/jvm/java-openjdk ] ; then
    JAVA_HOME=/usr/lib/jvm/java-openjdk
fi

# JAVA_HOME isn't set on OSX for some reason, so manually set it
if [ -z "$JAVA_HOME" -a -e /usr/libexec/java_home ]; then
    JAVA_HOME=$(/usr/libexec/java_home)
fi

# Looks like we don't have Java; install it.
if [ -z "$JAVA_HOME" -a -x /usr/bin/apt-get ] ; then
    apt-get update && apt-get install -y openjdk-11-jdk
    JAVA_HOME=$(readlink -f /usr/bin/java | sed 's,/bin/java,,')
fi

export JAVA_HOME

"${JAVA_HOME}/bin/java" -version

cargo fmt -- --check
cargo b
pushd tests && ~/bin/sbt test && popd
