# Rusty Password Manager

### A Rusty but Trusty Password Manager

## Requirements
- `rust 1.71.1` ( including `cargo` )
- `tree v2.0.2`
- `xclip 0.13`
- an Arch linux user ( ***I use arch btw*** )

## Installation
1. Clone the repo
2. `cd` into the repo
3.  ```bash
    cargo build --release
    cargo install --path .
    ```
- To Check if the installation was successful
    ```bash
    rusty --help
    ```
- Run `init` to save password save locations and other stuff
    ```bash
    rusty init
    ```
### And you should be good to go :) <br> Look at the usage section for more info or run ```rusty --help```


## Usage
- For Help
    ```bash
    rusty -h
    # or
    rusty --help
    ```
- To add a new password
    ```bash
    rusty add <service name> <password>
    ```
- To get a password
    ```bash
    rusty copy <service name>
    ```

### Upcoming Features
- [ ] Edit Saved Passwords (you can technically do this by overwriting the old password)
- [ ] Delete Saved Passwords
- [ ] Auto Complete
- [ ] A more secure way of encrypting the passwords 
- [ ] Colors (maybe)

### Why? A down to earth talk about this project

why am i doing this? casue i hate myself and would love to have a cli password manager ....
why is it called ~~`rusty-password-manager`~~ `rusty` ? ... idk 

why am i building this? i dont like the current password manager that im using casue its too complicated and filled with features ... 

im just building a pass manager with bare minimum features that i need and nothing more

have a great day .... dk how and why you ended up visiting this repo