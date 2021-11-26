# Displacement - A Simple Command-Line FTP Client in Rust

[WIP]

Displacement is a simple implementation of a command-line FTP client in Rust. At present, the functionalities supported are - 

* FTP Shell Access
* Authentication with the `USER` and `PASS` commands (will be automated soon enough)
* status commands (`STAT`)
* retrieving a single file (`RETR`)

## Prerequisites
These are the things you need to have to run displacment.
1. `vsftpd` or any other FTP server you can configure
2. `tcpflow` to monitor the packets being sent back and forth from `vsftpd` - do this if you want to see the raw TCP packets being sent to and from the FTP server on the standard port `21`.
```
$ tcpflow -i any -C -J port 21
```
3. The Rust Programming Language and its toolchain

## Building the Program
1. Clone the program
2. `cd` into the folder (i.e `cd displacement`)
3. Use `cargo`
```
$ cargo build
```

## Running the program
Same as ![](#Building the Program), but replace the last step with - 
```
$ cargo run
```
You can ignore the warnings for now.

Once you complete running the above, feel free to execute the program by exploring as you would a regular FTP shell.

## Basic FTP Shell Usage

Every FTP session begins with an authentication, which proceeds with the following commands, one after the other - 

```ftp
USER <your username>
PASS <your password>
```

You should now be logged in, and you're free to try out some of the more well-known commands. Try the following - 
```ftp
HELP
```

To be able to transfer a file, start off with the following - (at the moment, you'll need to do this before every transfer, regardless of session)
```ftp
PORT 127,0,0,1,10,1
```

Upon success, you can test the transfer facilities by running the command here, which should dump the entire file recieved to the terminal - 

```ftp
RETR <filename>
```


