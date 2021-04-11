<p align="center">
<a href="https://discord.gg/dumxVnYe6n">
    <img alt="Discord" src="https://img.shields.io/discord/332629362094374913"></a>
    <img alt="Build Status" src="https://github.com/AchetaGames/Epic-Asset-Manager/actions/workflows/rust.yml/badge.svg">
</p>

# Epic-Asset-Manager
A frontend to Assets purchased on Epic Games Store

## Current Screenshot
![Screenshot from 2021-04-05 03-15-03](https://user-images.githubusercontent.com/252905/113527240-2bbb8800-95bd-11eb-8580-60711816fd21.png)

## Install
### Arch Linux
Use the [AUR package](https://aur.archlinux.org/packages/eam-git)
### Build from source
 - Install rust using [rustup](https://rustup.rs/)
 - Install the stable toolchain
```
rustup install stable
rustup default stable
```
 - Install dependencies: **gtk3 libsoup webkit2gtk**
 - Clone the repository
```
git clone git@github.com:AchetaGames/Epic-Asset-Manager.git
```
 - Move into the repository
```
cd Epic-Asset-Manager
```
 - Build the project (the resulting binary is in target/release/epic_asset_manager)
```
cargo build --release
```
 - Or run the project
```
cargo run --release
```

## Action video 
[![Youtube Video](https://img.youtube.com/vi/mF0RGK5LglE/maxresdefault.jpg)](https://youtu.be/mF0RGK5LglE)
