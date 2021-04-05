<p align="center">
    <img width="200" alt="Alacritty Logo" src="https://raw.githubusercontent.com/alacritty/alacritty/master/extra/logo/compat/alacritty-term%2Bscanlines.png">
</p>

<p align="center">
      <a href="https://scott-hamilton.mit-license.org/"><img alt="MIT License" src="https://img.shields.io/badge/License-MIT-525252.svg?labelColor=292929&logo=creative%20commons&style=for-the-badge" /></a>
	  <a href="https://github.com/SCOTT-HAMILTON/alacritty/actions"><img alt="Build Status" src="https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2FSCOTT-HAMILTON%2Falacritty%2Fbadge&style=for-the-badge" /></a>
</p>

<h1 align="center">Fork of Alacritty - A fast, cross-platform, OpenGL terminal emulator</h1>

<p align="center">
  <img width="600"
       alt="Alacritty - A fast, cross-platform, OpenGL terminal emulator"
       src="https://user-images.githubusercontent.com/8886672/103264352-5ab0d500-49a2-11eb-8961-02f7da66c855.png">
</p>

<h3 align="center">Checkout the tabbed fork needed to make this work <a href="https://github.com/SCOTT-HAMILTON/tabbed">here</a></h3>

## About

This project is a fork of alacritty. (A name needs to be found tho).
Alacritty is a very good terminal emulator but there are two things that it lacks before being the perfect terminal emulator in my opinion :
1. It doesn't support tabs
2. Tabs don't open in the last working directory of the previous tab

I come from KDE Konsole and these are features that I absolutly need.

The first is easily solved with tabbed :
```shell_session
 $ tabbed -cr 2 alacritty --embed ""
```

And the second is solved with this patch that implements the working dir following protocol of tabbed.
Obviously, upstream tabbed doesn't have such a protocol, you need to use my tabbed fork [tabbed fork]


## How to test it right know ?

A nix shell is configured so that you can get this setup running in a few commands.
This shell builds the tabbed fork and this alacritty fork.

1. First install nix see [https://nixos.org/guides/install-nix.html](https://nixos.org/guides/install-nix.html)
I higly recommand you to check out the above link but normally this command should be enough :
```shell_session
 $ sh <(curl -L https://nixos.org/nix/install) --daemon
```
2. Navigate to this repo
```shell_session
 $ cd ~/path/to/where/you/cloned/my/alacritty/fork
```
3. enter the nix shell :
```shell_session
 $ nix-shell
```
4. (in the nix shell) build this alacritty fork :
```shell_session
 $ cargo build
```
5. (still in the nix shell) run the tabbed alacritty :
```shell_session
 $ tabbed -cr 2 -w "--xembed-tcp-port" ./target/debug/alacritty --embed ""
```
**Bonus for hackers**
6. (still in the nix shell) run the tabbed alacritty and put debug logs in a separate file :
```shell_session
 $ tabbed -cr 2 -w "--xembed-tcp-port" ./target/debug/alacritty --embed "" 2>&1 | ./filter_output.pl 'debug-' debug_logs /dev/stdout 2>&1
```


## How does it work ?

[![Flowchart Diagram](https://mermaid.ink/img/eyJjb2RlIjoiZ3JhcGggVERcbiAgICBBW1RhYmJlZCBwYXJlbnQgcHJvY2Vzc11cbiAgICBcbiAgICBBIC0tPnxYSUQgMXwgVEMxW1RhYmJlZCBDbGllbnQgMV1cbiAgICBBIC0tPnxYSUQgMnwgVEMyW1RhYmJlZCBDbGllbnQgMl1cbiAgICBBIC0tPnxYSUQgM3wgVEMzW1RhYmJlZCBDbGllbnQgM11cbiAgICBUQzEgLS0-IEExW0FsYWNyaXR0eSB0YWIgMV1cbiAgICBUQzIgLS0-IEEyW0FsYWNyaXR0eSB0YWIgMl1cbiAgICBUQzMgLS0-IEEzW0FsYWNyaXR0eSB0YWIgM11cbiAgICAiLCJtZXJtYWlkIjp7InRoZW1lIjoiZGFyayJ9LCJ1cGRhdGVFZGl0b3IiOmZhbHNlfQ)](https://mermaid-js.github.io/mermaid-live-editor/#/edit/eyJjb2RlIjoiZ3JhcGggVERcbiAgICBBW1RhYmJlZCBwYXJlbnQgcHJvY2Vzc11cbiAgICBcbiAgICBBIC0tPnxYSUQgMXwgVEMxW1RhYmJlZCBDbGllbnQgMV1cbiAgICBBIC0tPnxYSUQgMnwgVEMyW1RhYmJlZCBDbGllbnQgMl1cbiAgICBBIC0tPnxYSUQgM3wgVEMzW1RhYmJlZCBDbGllbnQgM11cbiAgICBUQzEgLS0-IEExW0FsYWNyaXR0eSB0YWIgMV1cbiAgICBUQzIgLS0-IEEyW0FsYWNyaXR0eSB0YWIgMl1cbiAgICBUQzMgLS0-IEEzW0FsYWNyaXR0eSB0YWIgM11cbiAgICAiLCJtZXJtYWlkIjp7InRoZW1lIjoiZGFyayJ9LCJ1cGRhdGVFZGl0b3IiOmZhbHNlfQ)

When spawning a new alacritty window, tabbed also forks a child process that will communicate with this alacritty window threw Unix Domain Sockets (cf [wiki](https://systemprogrammingatntu.github.io/mp2/unix_socket.html).
This allows non-blocking bidirectionnal communications between the child process and the alacritty window.
**A child is referred to as a client in tabbed**

Each tabbed client is identified by an XID, which is the X11 Identifier of the alacritty window it's responsible of, (cf [https://metacpan.org/pod/X11::Xlib#DESCRIPTION](https://metacpan.org/pod/X11::Xlib#DESCRIPTION))


### Authentification Step

The tabbed client doesn't know its window's XID when spawned, it needs to ask for it.
[![Tabbed client asking XID from alacritty window](https://mermaid.ink/img/eyJjb2RlIjoic2VxdWVuY2VEaWFncmFtXG4gICAgVGFiYmVkIENsaWVudCAxLT4-K0FsYWNyaXR0eSBUYWIgMTogSGVsbG8gQWxhY3JpdHR5LCB3aGF0J3MgeW91ciBYMTEgV2luZG93IElkZW50aWZpZXIgP1xuICAgIEFsYWNyaXR0eSBUYWIgMS0tPj4tVGFiYmVkIENsaWVudCAxOiBIaSwgbXkgWElEIGlzIDEzODQxMjAzNC5cbiAgICAgICAgICAgICIsIm1lcm1haWQiOnsidGhlbWUiOiJkZWZhdWx0In0sInVwZGF0ZUVkaXRvciI6ZmFsc2V9)](https://mermaid-js.github.io/mermaid-live-editor/#/edit/eyJjb2RlIjoic2VxdWVuY2VEaWFncmFtXG4gICAgVGFiYmVkIENsaWVudCAxLT4-K0FsYWNyaXR0eSBUYWIgMTogSGVsbG8gQWxhY3JpdHR5LCB3aGF0J3MgeW91ciBYMTEgV2luZG93IElkZW50aWZpZXIgP1xuICAgIEFsYWNyaXR0eSBUYWIgMS0tPj4tVGFiYmVkIENsaWVudCAxOiBIaSwgbXkgWElEIGlzIDEzODQxMjAzNC5cbiAgICAgICAgICAgICIsIm1lcm1haWQiOnsidGhlbWUiOiJkZWZhdWx0In0sInVwZGF0ZUVkaXRvciI6ZmFsc2V9)

The messages involved are :
`XID?` and `XID:138412034`


### Loop

Now that the client is authentified, in other words, now that it knows its associated window's XID, it can enter the following two state loops :

**The communication loop**

[![](https://mermaid.ink/img/eyJjb2RlIjoic3RhdGVEaWFncmFtLXYyXG4gICAgc3RhdGUgXCJEaWQgSSByZWNlaXZlIGEgbWVzc2FnZSA_XCIgYXMgVDFcbiAgICBzdGF0ZSBcIklmIGl0J3MgYSBQV0QgQW5zd2VyLCBzYXZlIGl0IGZvciBsYXRlclwiIGFzIFNcblxuICAgIFsqXSAtLT4gVDFcbiAgICBUMSAtLT4gTm9cbiAgICBObyAtLT4gVDFcbiAgICBUMSAtLT4gWWVzXG4gICAgWWVzIC0tPiBTXG4gICAgUyAtLT4gVDFcbiIsIm1lcm1haWQiOnsidGhlbWUiOiJkZWZhdWx0In0sInVwZGF0ZUVkaXRvciI6ZmFsc2V9)](https://mermaid-js.github.io/mermaid-live-editor/#/edit/eyJjb2RlIjoic3RhdGVEaWFncmFtLXYyXG4gICAgc3RhdGUgXCJEaWQgSSByZWNlaXZlIGEgbWVzc2FnZSA_XCIgYXMgVDFcbiAgICBzdGF0ZSBcIklmIGl0J3MgYSBQV0QgQW5zd2VyLCBzYXZlIGl0IGZvciBsYXRlclwiIGFzIFNcblxuICAgIFsqXSAtLT4gVDFcbiAgICBUMSAtLT4gTm9cbiAgICBObyAtLT4gVDFcbiAgICBUMSAtLT4gWWVzXG4gICAgWWVzIC0tPiBTXG4gICAgUyAtLT4gVDFcbiIsIm1lcm1haWQiOnsidGhlbWUiOiJkZWZhdWx0In0sInVwZGF0ZUVkaXRvciI6ZmFsc2V9)

The **PWD** is the shell's working directory of the current focused window.
It changes each time the user executes a `cd /somewhere/` command.
The message involved is : `PWD:/home/user`



**The logic loop :**

[![](https://mermaid.ink/img/eyJjb2RlIjoic3RhdGVEaWFncmFtLXYyXG4gICAgc3RhdGUgXCJBbSBJIHRoZSBmb2N1c2VkIHRhYiA_XCIgYXMgVDFcbiAgICBzdGF0ZSBcIlNsZWVwIE1vZGVcIiBhcyBTXG4gICAgc3RhdGUgXCJUdXJibyBNb2RlXCIgYXMgVFxuICAgIHN0YXRlIFwiVHJhbnNtaXQgdGhlIHNhdmVkIHNoZWxsIFBXRCB0byB0YWJiZWQgcGFyZW50IHByb2Nlc3MgaWYgYW55IHdhcyByZWNlaXZlZFwiIGFzIFBcbiAgICBzdGF0ZSBcIkFzayB3aW5kb3cgZm9yIHNoZWxsIFBXRFwiIGFzIEExXG5cblxuICAgIFsqXSAtLT4gVDFcbiAgICBUMSAtLT4gTm9cbiAgICBObyAtLT4gU1xuICAgIFMgLS0-IFQxXG4gICAgVDEgLS0-IFllc1xuICAgIFllcyAtLT4gVFxuICAgIFQgLS0-IFBcbiAgICBQIC0tPiBBMVxuICAgIEExIC0tPiBUMSAiLCJtZXJtYWlkIjp7InRoZW1lIjoiZGVmYXVsdCJ9LCJ1cGRhdGVFZGl0b3IiOmZhbHNlfQ)](https://mermaid-js.github.io/mermaid-live-editor/#/edit/eyJjb2RlIjoic3RhdGVEaWFncmFtLXYyXG4gICAgc3RhdGUgXCJBbSBJIHRoZSBmb2N1c2VkIHRhYiA_XCIgYXMgVDFcbiAgICBzdGF0ZSBcIlNsZWVwIE1vZGVcIiBhcyBTXG4gICAgc3RhdGUgXCJUdXJibyBNb2RlXCIgYXMgVFxuICAgIHN0YXRlIFwiVHJhbnNtaXQgdGhlIHNhdmVkIHNoZWxsIFBXRCB0byB0YWJiZWQgcGFyZW50IHByb2Nlc3MgaWYgYW55IHdhcyByZWNlaXZlZFwiIGFzIFBcbiAgICBzdGF0ZSBcIkFzayB3aW5kb3cgZm9yIHNoZWxsIFBXRFwiIGFzIEExXG5cblxuICAgIFsqXSAtLT4gVDFcbiAgICBUMSAtLT4gTm9cbiAgICBObyAtLT4gU1xuICAgIFMgLS0-IFQxXG4gICAgVDEgLS0-IFllc1xuICAgIFllcyAtLT4gVFxuICAgIFQgLS0-IFBcbiAgICBQIC0tPiBBMVxuICAgIEExIC0tPiBUMSAiLCJtZXJtYWlkIjp7InRoZW1lIjoiZGVmYXVsdCJ9LCJ1cGRhdGVFZGl0b3IiOmZhbHNlfQ)

When entering sleep mode, the tabbed client also sends a message informing the window that it should also enter the sleep mode, same goes to the turbo mode.
These modes are critical for limiting CPU usage.

Since the user can change the working directory at anytime, the client has to ask for it constantly.
And so it needs to transmit the flow of answers to the tabbed parent process.
This is the **key system** that allows it to follow the working directories.
Because the tabbed parent process always knows the PWD of the currently focused window's shell, it can spawn a new one at the good location.


The messages involved are : `sleep`, `turbo` and `PWD?`


## License

Alacritty is released under the [Apache License, Version 2.0].
This few patches are released under the [MIT License].





**References that helped**
 - [qubes-os markdown conventions] : <https://www.qubes-os.org/doc/doc-guidelines/#markdown-conventions>
 - [Linux man pages] : <https://linux.die.net/man/>
 - [TcpStream rust doc] : <https://docs.rs/mio/0.5.1/mio/tcp/struct.TcpStream.html>
 - [mermaid-js documentation] : <https://mermaid-js.github.io/mermaid/#/stateDiagram>

[//]: # (These are reference links used in the body of this note and get stripped out when the markdown processor does its job. There is no need to format nicely because it shouldn't be seen. Thanks SO - http://stackoverflow.com/questions/4823468/store-comments-in-markdown-syntax)



   [qubes-os markdown conventions]: <https://www.qubes-os.org/doc/doc-guidelines/#markdown-conventions/>
   [Linux man pages]: <https://linux.die.net/man/>
   [TcpStream rust doc]: <https://docs.rs/mio/0.5.1/mio/tcp/struct.TcpStream.html>
   [mermaid-js documentation]: <https://mermaid-js.github.io/mermaid/#/stateDiagram>


[MIT License]: https://scott-hamilton.mit-license.org/
[tabbed fork]: https://github.com/SCOTT-HAMILTON/tabbed
