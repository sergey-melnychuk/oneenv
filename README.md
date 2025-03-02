ONE ENV
=======

All local env vars in a single file.

```
cargo build --release
cp ./target/release/oneenv ~/.cargo/bin/
```

```
## MacOS
$ vim ~/.zshrc
...
precmd() {
  local X=$(oneenv); eval "$X"
}

## Linux
$ vim ~/.bashrc
...
PROMPT_COMMAND='X=$(oneenv); eval "$X"'
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
