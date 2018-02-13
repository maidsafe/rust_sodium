# Small script to run tests for a target (or all targets) inside all the
# respective docker images.

set -ex

run() {
    case "$TARGET" in
        armv7-linux-androideabi)
            echo $1
            docker build -t libc ci/docker/$1
            mkdir -p target
            docker run \
                   --user `id -u`:`id -g` \
                   --rm \
                   --volume $HOME/.cargo:/cargo \
                   --env CARGO_HOME=/cargo \
                   --volume `rustc --print sysroot`:/rust:ro \
                   --volume `pwd`:/checkout:ro \
                   --volume `pwd`/target:/checkout/target \
                   --env CARGO_TARGET_DIR=/checkout/target \
                   --workdir /checkout \
                   --privileged \
                   --interactive \
                   --tty \
                   libc \
                   ci/run.sh $1
            ;;

        i686-linux-android)
            echo $1
            docker build -t libc ci/docker/$1
            mkdir -p target
            docker run \
                   --user `id -u`:`id -g` \
                   --rm \
                   --volume $HOME/.cargo:/cargo \
                   --env CARGO_HOME=/cargo \
                   --volume `rustc --print sysroot`:/rust:ro \
                   --volume `pwd`:/checkout:ro \
                   --volume `pwd`/target:/checkout/target \
                   --env CARGO_TARGET_DIR=/checkout/target \
                   --workdir /checkout \
                   --privileged \
                   --interactive \
                   --tty \
                   --volume `pwd`/ci:/ci \
                   toopher/centos-i386:centos6 \
                   /bin/bash -c "linux32 --32bit i386 /ci/run.sh $1"
            ;;

        *)
            exit 1;
            ;;
    esac
}

if [ -z "$1" ]; then
    for d in `ls ci/docker/`; do
        run $d
    done
else
    run $1
fi
