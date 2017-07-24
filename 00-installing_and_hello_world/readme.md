### Installing Rust in my Linux Mint
To install Rust, I need first running this in terminal:
```
curl https://sh.rustup.rs -sSf | sh
```

This will download and install some packages to my computer:
- `rustc`
- `rust-std`
- `cargo`
- `rust-docs`

After installing those, then the terminal will guide me to run this command to configure my current shell:
```
source ~/.cargo/env
```

After this command, let's check if we get Rust running already. Run this command in the terminal:
```
rustc --version
```

I got `rustc 1.19.0 (0ade33941 2017-07-17)`. It means Rust is successfully installed in my Linux Mint. Yay!

Is that all? No. I have to configure my Visual Studio Code to support my Rust Programming.

### Configuring Visual Studio Code
Configuring VS Code is not very hard, we just have to install some extension. I got the recommendation from [This guide from rustnish](https://klausi.github.io/rustnish/2017/05/28/using-visual-studio-code-for-rust-on-ubuntu.html). Thanks, bro!

Then I proceeded to install these extensions:

- Rust code completion and auto formatting: [Rust](https://marketplace.visualstudio.com/items?itemName=kalitaalexey.vscode-rust)
- TOML configuration files syntax highlighting: Better [TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml)
- LLDB debugging for Rust programs: [LLDB Debugger](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)
- nice file icons: [vscode-icons](https://marketplace.visualstudio.com/items?itemName=robertohuertasm.vscode-icons)

Yea maybe the last item is not mandatory but let's just do that. :D

Now, let's get dirty! I will get my [book](https://doc.rust-lang.org/book/second-edition/) ready and get into the code!

### Hello World in Rust
First, create a file in the project directory with `rs` extension (it's Rust!). And then, add this code to it:
```
fn main() {
    println!("Hello, Darkness my old friend!");
}
```

Afer saving the `.rs` file, Run `rustc` to compile:
```
rustc hello_world.rs
```

Okay I got this weird error:
```
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/home/umanium/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "hello_world.0.o" "-o" "hello_world" "-Wl,--gc-sections" "-pie" "-nodefaultlibs" "-L" "/home/umanium/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/home/umanium/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-35ad9950c7e5074b.rlib" "/home/umanium/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-20a50a22d4c2b1e9.rlib" "/home/umanium/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcollections-b479831207997444.rlib" "/home/umanium/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_unicode-f4f0ae88f5ad8ad4.rlib" "/home/umanium/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-fb44afc024bbc636.rlib" "/home/umanium/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-14b8f3202acdad6a.rlib" "/home/umanium/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-10b591f1a68dd370.rlib" "/home/umanium/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-28913dc5a1e63cd7.rlib" "/home/umanium/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-6ecacccb5bdc4911.rlib" "/home/umanium/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-bfaa82017ca17cb2.rlib" "/home/umanium/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-863b57a66ba6c3e1.rlib" "-Wl,-Bdynamic" "-l" "dl" "-l" "rt" "-l" "pthread" "-l" "gcc_s" "-l" "pthread" "-l" "c" "-l" "m" "-l" "rt" "-l" "pthread" "-l" "util"
  = note: /usr/bin/ld: cannot find Scrt1.o: No such file or directory
          /usr/bin/ld: cannot find crti.o: No such file or directory
          /usr/bin/ld: cannot find -ldl
          /usr/bin/ld: cannot find -lrt
          /usr/bin/ld: cannot find -lpthread
          /usr/bin/ld: cannot find -lpthread
          /usr/bin/ld: cannot find -lc
          /usr/bin/ld: cannot find -lm
          /usr/bin/ld: cannot find -lrt
          /usr/bin/ld: cannot find -lpthread
          /usr/bin/ld: cannot find -lutil
          /usr/bin/ld: cannot find crtn.o: No such file or directory
          collect2: error: ld returned 1 exit status
```

After searching the internet, I found out that the package `build-essential` is missing, so the compilation cannot be completed. I installed the package then:
```
sudo apt-get install build-essential
```

Then voila! The program is compiled successfully! Yay :D. Let's run it!
```
./hello_world
Hello darkness, my old friend!
```

That's one million steps for humankind! :D