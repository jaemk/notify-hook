# notify-hook

[![Build Status](https://travis-ci.org/jaemk/notify-hook.svg?branch=master)](https://travis-ci.org/jaemk/notify-hook)

> Git post-receive hook to send GitHub PushEvent-formatted requests

Note, some payload fields are missing because I'm lazy. The exact payload format can be found in [`payload.rs`](https://github.com/jaemk/notify-hook/blob/master/src/payload.rs).


**notifyhook.reponame**

Optional: Repository name to use. Defaults to: ``` basename -s .git `git config --get remote.origin.url` ```


**notifyhook.hookurls**

Comma separated list of urls to post content to


**notifyhook.secrettoken**

Optional: Hex encoded secret token used to generate a [GitHub X-Hub-Signature](https://developer.github.com/webhooks/securing/) header.


**notifyhook.contenttype**

Content type to send payload as. Defaults to `urlencoded`. Options:

- `urlencoded`: (Default) Post as `application/x-www-form-urlencoded`
- `json`: Post as `application/json`

