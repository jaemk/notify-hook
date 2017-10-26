# notify-hook

[![Build Status](https://travis-ci.org/jaemk/notify-hook.svg?branch=master)](https://travis-ci.org/jaemk/notify-hook)

> Git post-receive hook to send GitHub PushEvent-formatted requests

Note, some payload fields are missing. The exact payload format can be found in [`payload.rs`](https://github.com/jaemk/notify-hook/blob/master/src/payload.rs).

See [`releases`](https://github.com/jaemk/notify-hook/releases) for binaries, or build from source (see `Releases` section below).
If you've installed a pre-compiled binary, you can update to the latest release via `notify-hook self update`.

----

**notifyhook.repo-name**

Optional: Repository name to use. Defaults to: ` basename -s .git `git config --get remote.origin.url` `


**notifyhook.repo-description**

Optional: Repository description to use. Defaults to: blank


**notifyhook.repo-owner-name**

Optional: Repository owner name to use. Defaults to: blank


**notifyhook.repo-owner-email**

Optional: Repository owner email to use. Defaults to: blank


**notifyhook.hook-urls**

Comma separated list of urls to post content to.


**notifyhook.secret-token**

Optional: Hex encoded secret token used to generate a [GitHub X-Hub-Signature](https://developer.github.com/webhooks/securing/) header.


**notifyhook.content-type**

Content type to send payload as. Defaults to `urlencoded`.

Options:

- `urlencoded`: (Default) Post as `application/x-www-form-urlencoded`
- `json`: Post as `application/json`


## Development

- Install [`rust`](https://rustup.rs)
- `cargo build`


## Releases

Statically compiled releases are built by travis-ci with `--features update` to allow `notify-hook self update`.
If you want to fork this project and continue building/self-updating with github releases, you'll need
to enable the `notify-hook` repository in your travis-ci account and then update the `update` function
to point to your forked github repository.

If you prefer to build locally, there are two options:

- If your build and target architectures are identical, you can get away with a simple `cargo build --release`. Note, this requires
  `libssl-dev` to be installed in your build environment, and `openssl` installed on your target.
- If you want to build static releases, like those built by travis-ci, you can use the `build-release.py` script.
  This script requires `docker` and `cross` to be installed to produce statically compiled binaries. By default,
  artifacts will be produced for `i686` and `x86_64`. This can be tweaked in the `TARGETS` list of `build-release.py`.

- Install [`docker`](https://www.digitalocean.com/community/tutorials/how-to-install-and-use-docker-on-ubuntu-16-04)
    - Add yourself to the `docker` group: `sudo usermod -a -G docker <user>`
    - Restart to pick up changes (logging in & out may suffice)
    - You should be able to run `docker version` without any errors
    - May need to start the Docker daemon if it's not already running: `sudo systemctl start docker`
- Install [`cross`](https://github.com/japaric/cross): `cargo install cross`
- `./build-release.py`
    - Release artifacts (`i686` & `x86_64`) will be copied into `bin/32` & `bin/64`

