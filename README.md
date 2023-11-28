# vst-2

Remaking the same project from here: https://github.com/esteveslima/vst

But this time in Rust!

### Notes:

- Coding Environment: VSCode

- Running Environment:
  - Host machine
    - Must install all rust packages manually
  - Docker
    - Setup files in `/assets/environment`
    - Use the helper `Makefile` to setup the environment
      - open: `$ make up`
      - close: `$ make down`
    - Attach VSCode into the docker container using a plugin(recommended in the plugins.json file)
  
- Run Project: 
  - build/install dependencies: `cargo build` (optional flag `--release`)
  - simple run: `cargo run`
  - run development(hot reload): `cargo watch --watch src --exec run`

- Debug Project:
    - Use the preferrable attach config to attach into the running application, look for `/target/debug/{PROJECT_NAME}` process to attach
    - Otherwise launch the application in debug mode directly, it'll refer to `/target/debug/{PROJECT_NAME}`
      - P.S.: might be necessary to build the project manually to update the debug file before launching it in debug mode