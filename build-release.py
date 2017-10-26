#!/usr/bin/env python

import os
import sys
import time
import subprocess

ARTIFACT = "notify-hook"
PROJDIR = os.path.dirname(os.path.realpath(__file__))
BINDIR = os.path.join(PROJDIR, 'bin')
BINDIR_32 = os.path.join(BINDIR, '32')
BINDIR_64 = os.path.join(BINDIR, '64')

# Tweak targets as needed
TARGETS = [
    #(architecture, environment, artifact-output-dir)
    ("i686", 'gnu', BINDIR_32),
    ("x86_64", 'musl', BINDIR_64),
]


class CmdError(Exception):
    pass


def cmd(args):
    proc = subprocess.Popen(args, stdout=subprocess.PIPE)
    for line in iter(proc.stdout.readline, ''):
        sys.stdout.write(line)
    while proc.poll() is None:
        time.sleep(0.5)
    if proc.returncode != 0:
        raise CmdError("Command: {} exited with status: {}".format(args, proc.returncode))


def mkdir(p):
    return cmd(['mkdir', '-p', p])


def main(args):
    print("** Building release artifacts for: {} **".format(ARTIFACT))
    mkdir(BINDIR_32)
    mkdir(BINDIR_64)

    print("\n** START BUILD OUTPUT **")
    print(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>")
    for arch, env, bin_dir in TARGETS:
        target_name = "{}-unknown-linux-{}".format(arch, env)
        print("** Building release artifact for {} **".format(target_name))
        cmd(["cross", "build", "--release", "--target", target_name])
        artifact = os.path.join(PROJDIR, "target", target_name, "release", ARTIFACT)
        cmd(["cp", artifact, bin_dir])
    print("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<")
    print("** END BUILD OUTPUT **\n")

    print("** Release artifacts copied to {}".format(BINDIR))


if __name__ == '__main__':
    main(sys.argv[1:])

