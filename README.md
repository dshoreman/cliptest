## Cliptest

A very minimal base for testing target issues in x11-clipboard.

### Installation / Testing

Clone this repo with [my x11-clipboard fork](https://github.com/dshoreman/x11-clipboard) inside it, making sure you're on the right branch:

```sh
git clone https://github.com/dshoreman/cliptest.git
cd cliptest && git clone -b paste-text-targets https://github.com/dshoreman/x11-clipboard.git
cargo run
```

Once it's running, open another terminal to play around copying text to clipboard with various targets:
```
echo "foo" | xclip
echo "bar" | xclip -t TEXT
echo "baz" | xclip -t UTF8_STRING
echo "bat" | xclip -t text/plain
```
If it's not ignored, it'll show in stdout.
