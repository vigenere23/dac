<div align="center">

# Disma

[![Build](https://github.com/vigenere23/disma/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/vigenere23/disma/actions/workflows/build.yml)
[![Deploy](https://github.com/vigenere23/disma/actions/workflows/deploy.yml/badge.svg)](https://github.com/vigenere23/disma/actions/workflows/deploy.yml)

**👨🏼‍🔧 Discord server management has never been easier!**

</div>

## Why ?

In the new context of the pandemic, many educational institutions have shifted their courses online, with the use of communication platforms like Discord. However, managing multiple roles and channels across a Discord server is challenging : there are no way to centrally visualize the information nor to apply synchronized permissions updates. This tool allows you to define a single configuration file to be applied to a server, and it will automatically find the changes that needs to be made, ensuring that your Discord will always be in sync with your config.

## Features

- 📜 **Diff current Discord server config with your desired one**
- 🏗️ **Apply large scale changes to your Discord server**
- ⚡ **Fast, secure and reliable**
- 📋 **Advanced templating techniques**

## Prerequesites

Your will need to setup a [Discord bot](./docs/bot.md) in order to use the CLI tool.

## Modules

- [disma](./disma) : Core Rust library for defining configuration as code and controlling your own orchestrations and implementations.
- [disma-cli](./disma-cli) : A stable and easy to use command line interface for defining configuration as simple YAML files. Still allows for templating to simplify the configuration.

## Documentation

- [Configuration file format](./docs/config.md)
- [Roadmap](./docs/roadmap.md)
