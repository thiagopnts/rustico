# Rustico

A tiny 32 bit kernel written in Rust.

It prints 'olar mundo' then hangs. That's it:

![](http://f.cl.ly/items/2U2g0i111J3O0E0C1G25/Screen%20Shot%202014-04-07%20at%209.22.52%20PM.png)

## Setup

With [docker](https://www.docker.com/) installed run:
```bash
$ docker build -t rustico .
```

## Running it

```bash
$ docker run -it --privileged -v $(pwd):/kernel rustico make run
```
