ONE ENV
=======

All local env vars in a single file.

```
cargo build --release
cp ./target/release/oneenv ~/.cargo/bin/
```

```
$ vim ~/.zshrc
## https://superuser.com/a/735969
precmd() {
  oneenv | while read -r line; do eval "export $line"; done
}
```

```
$ cat ~/.oneenv 
/Users/john.doe/this:
API_KEY=qweqweqwe

/Users/john.doe/that:
ANSWER=42
```

```
$ cd /Users/john.doe/this
$ echo "https://cool.api/${API_KEY}"
https://cool.api/qweqweqwe

$ cd /Users/john.doe/that
$ echo $ANSWER
42
```
