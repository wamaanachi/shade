# Shade

[![Shade](https://img.shields.io/badge/GitHub-Shade-green?style=plastic)](https://www.github.com/wamaanachi/Shade)

> Fun with chemistry and statistics

Fun chem app ("Fun" in very, very deep quotes, and very subjective)

---

## Features

*   **List:** Lists all shaders
*   **Modify:** User can modify shader params
*   **Run:** Run specific shaders

## Stack

*   **Frontend:**   Terminal, Hackatui
*   **Backend:**    Rust
*   **Styling:**    Vibez

---

## Getting Started

Follow these simple steps to get a local copy up and running.

### Prerequisites

None (Unless you want to build it from source)

### How to get this for yourselves:

**Download it**

Get the latest release from [GitHub](https://github.com/wamaanachi/Shade/releases)

>**Note:** The name of the executable will be "shade-*", where * is the version number, and build version,
> remember to type the full name when executing like ```./shade-* -V``` , or rename it from "shade-*" to "shade"

> If you downloaded, most likely it is in the downloads directory,
> so either move it to the home directory (/home/user/) or run ```cd ~/Downloads``` before
> doing ```./shade```


Or download from command line, like this:


#### Linux

```shell
curl -L https://github.com/wamaanachi/Shade/releases/download/Release/shade-1.0.0-x86_64-Linux -o shade
chmod +x shade
```

> Always check what you are running, don't run random commands you find on the internet.

Done!, add to path to run anywhere or run from home like:
```shell
./shade -V
```

## Building from Source

1. **Pre-requisites:**  
   Need to install Rust


2. **Clone the repository:**
   ```shell
   git clone https://github.com/wamaanachi/Shade.git
   cd Shade
   ```


3. **Build**:  

   ```shell
   cargo build --release
   ```


4. **Done!:**  
   Now test the binary with:
   ```shell
   ./shade -V
   ```

## Contributors
*   **[![Amaan](https://img.shields.io/badge/GitHub-MiniGun1239-orange?style=plastic)](https://www.github.com/MiniGun1239)**
*   **[![Ruster](https://img.shields.io/badge/GitHub-Ruster-blue?style=plastic)](https://github.com/Tylerr-D)**
*   **[![Chish](https://img.shields.io/badge/GitHub-Chish-green?style=plastic)](https://www.github.com/Chish)**

> Coded and tested in Ubuntu, Fedora and Arch Linux, should work in any distro.


### [Examples](https://github.com/wamaanachi/Shade/tree/main/Examples)

```shell
$ shade --help
Usage: shade [OPTIONS] [COMMAND]

Commands:
  run   
  list  
  help  Print this message or the help of the given subcommand(s)

Options:
      --demo     
  -h, --help     Print help
  -V, --version  Print version
```
