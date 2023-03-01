Hi Kerry,

It's a rust file so you just need the binary - for an Intel mac its here: `target/release/x86_64-apple-darwin/release/to_unix`.

I don't have a mac developer licence so:

- You'll need to set the permissions for the binary to executable with `chmod 755`.
- You'll need to allow your mac to run the file.

I keep the script in `~/tools/` and have this in my .zshrc `'alias to_unix='fn() { ~/tools/to_unix $1};f'`.

Then in my terminal I can run: `to_unix file.csv` to correct the file.

Just a heads up - It overwrites the files contents.
