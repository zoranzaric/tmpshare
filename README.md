# tmpshare

[![Crates.io](https://img.shields.io/crates/v/tmpshare.svg)](https://crates.io/crates/tmpshare)
[![Documentation](https://docs.rs/tmpshare/badge.svg)](https://docs.rs/tmpshare/)
![License](https://img.shields.io/crates/l/tmpshare.svg)
[![Build Status](https://travis-ci.org/zoranzaric/tmpshare-rs.svg?branch=master)](https://travis-ci.org/zoranzaric/tmpshare)

`tmpshare` is a tool to share files.

### Usage

```
$ echo "Hello World" > hello-world

$ tmpshare add hello-world
D2A84F4B8B650937EC8F73CD8BE2C74ADD5A911BA64DF27458ED8229DA804A26

$ tmpshare list
D2A84F4B8B650937EC8F73CD8BE2C74ADD5A911BA64DF27458ED8229DA804A26: hello-world

$ tmpshare serve
Serving from http://127.0.0.1:8080

$ curl http://127.0.0.1:8080/get/D2A84F4B8B650937EC8F73CD8BE2C74ADD5A911BA64DF27458ED8229DA804A26
Hello World
```

## Development

`tmpshare` is mostly developed during live coding sessions by
[Zoran Zaric on Twitch](http://twitch.tv/zoranstreams). It is archived on
[YouTube](https://www.youtube.com/playlist?list=PLzZiioPR-W-ZbMAdbvvsTPkFGz_uLwbjB)

  * [2018-04-01: First session](https://youtu.be/kl-w8TQzMv4) (setup, hashing, metadata)
  * [2018-04-02: Second session](https://www.youtube.com/watch?v=F3sG3aDQT_4) (http server, more setup)
  * [2018-04-06: Third session](https://youtu.be/2XOUEEjDSns) (error handling, refactoring, http file handling (`Content-Dispositon`-header))
  * [2018-04-07: Fourth session](https://youtu.be/ZUhlIU2yywc) (serde_json improvement, structopt, refactoring, list command)
