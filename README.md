Successfully serves locally.

I tried to  deploy two ways on to cloudflare pages, without success.

I tried to push the compiled release wasm file to github by first calling
`trunk build --release`
then `git push origin`

With the cloudflare build settings set to:

build command: `none`
entry source: `target/wasm32-unknown-unknown/release/yew-app.wasm`


No luck, so I tried 

build command `trunk build --release`
and entry source: `target/wasm32-unknown-unknown/release/yew-app.wasm`

No luck, here we failed with the following output

```
14:30:43.551	v12.18.0 is already installed.
14:30:44.076	Now using node v12.18.0 (npm v6.14.4)
14:30:44.122	Started restoring cached build plugins
14:30:44.126	Finished restoring cached build plugins
14:30:44.260	Attempting ruby version 2.7.1, read from environment
14:30:45.322	Using ruby version 2.7.1
14:30:45.606	Using PHP version 5.6
14:30:45.639	5.2 is already installed.
14:30:45.645	Using Swift version 5.2
14:30:45.645	Installing Hugo 0.54.0
14:30:46.232	Hugo Static Site Generator v0.54.0-B1A82C61A/extended linux/amd64 BuildDate: 2019-02-01T10:04:38Z
14:30:46.233	Started restoring cached go cache
14:30:46.236	Finished restoring cached go cache
14:30:46.262	go version go1.14.4 linux/amd64
14:30:46.266	go version go1.14.4 linux/amd64
14:30:46.267	Installing missing commands
14:30:46.267	Verify run directory
14:30:46.267	Executing user command: trunk build --release
14:30:46.268	/opt/build/bin/build: line 39: trunk: command not found
14:30:46.270	Failed: build command exited with code: 127
```