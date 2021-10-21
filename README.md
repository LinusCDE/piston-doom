# piston-doom

This was primarily a test whether a doom port could be integrated into rust without much hassle (to later integrate with another platform).

The awesome doom port [doomgeneric](https://github.com/ozkl/doomgeneric) was used as a basis. The image is rendered and input read with [piston](https://github.com/PistonDevelopers/piston).

![piston-doom](https://transfer.cosmos-ink.net/12oclH/Unbenannt.png)

Since this project was merely a test, running it isn't that polished. I developed this on linux and it should run on most distributions. It might run on windows as well, but I haven't tested it.

---

## How to run this

### Install rust

- Install rust. [rustup](https://rustup.rs/) is a great installer. You should be able to run the `cargo` command afterwards.

### Clone this repo

Clone this repo with the doomgeneric submodule: `$ git clone https://github.com/LinusCDE/piston-doom.git`.  
Go into the clone directory afterwards.

### Get an IWAD file

This project expects two dependencies: The doomgeneric repo and a IWAD file named in a certain way. The easiest way is to get the `doom1.wad` (more details [here](https://doomwiki.org/wiki/DOOM1.WAD)). Pleace it into this repo and DON'T FORGET to make the name all uppercase (rename to `DOOM1.WAD`). Other files don't need this change:

- doom2.wad
- plutonia.wad
- tnt.wad
- doom.wad
- DOOM1.WAD
- chex.wad
- hacx.wad
- freedm.wad
- freedoom2.wad
- freedoom1.wad

### Compile

This should be as easy as running `$ cargo build --release`. I'll automatically compile this rust project and also compile doomgeneric and link it statically into the rust binary. I used rust nightly. But stable (the default) should work as well.

### Done

The finished binary should now be in `target/release` named `piston-doom`. Just run that or use `$ cargo run` to do it for you.

Please note that I'm not passing the cli arguments. So you can't specify `-iwad /path/to/file.wad`.

---

## Missing features

Todos if you will. I'll probably never do these features for this test port. Most of these shouldn't be too much trouble to add though. Feel free to create a PR.

- Inputs other than keyboard
- Setting `myargv` and `myargc` to support the `-iwad`-flag: You need to the file named in a default way for now. The console will show you all the names
- Stopping the game by closing the window. You need to properly exit out of the game or kill the process. Otherwise it'll hang forever.
- Sound. The port doesn't seem to care and I don't care, since my target platform doesn't have a speaker.
- Better integration / more seamless build (already explained before `How to run this`)
