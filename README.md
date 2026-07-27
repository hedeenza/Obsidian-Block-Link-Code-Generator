# Obsidian-Block-Link-Code-Generator
Manually generate block-linking codes for Obsidian 

## Use Case:
- Obsidian allows users to create internal links to not only entire notes, but specific headers or blocks of text within that note.[^1](https://obsidian.md/help/links#Link+to+a+block+in+a+note) While linking to blocks, Obsidian will automatically generate a code that serves as an anchor in the target file. However, in very long notes, scrolling down to the desired block can take considerable time away from more high-impact tasks.
- This tool generates block linking codes formatted in the same manner as the ones Obsidian creates automatically. They may be pasted below the linked block in the place they would appear automatically, and then used again in the link, formatted as `[[Note#Code]]`. For example, `[[Note of Interest#^qa902h]]`.

## Get the Tool
- Source code for major releases is available in "Releases".
- You may also run `git clone https://github.com/hedeenza/Obsidian-Block-Link-Code-Generator.git` in the command line to clone the repository, or download the repository as a zipped directory by clicking on the green `<> Code` button and selecting `Download ZIP`.

## Running the CLI
`$ ./obsidian_block_link_code_generator <NUMBER-OF-CODES> <CODE-LENGTH>`

- `<NUMBER-OF-CODES>`: The number of random codes to output. Default 1.
- `<CODE-LENGTH>`: The number of characters in the output code, not including the initial `^`. Default 6.
- Ensure the program has executable permissions.

## Building from Source
Navigate to the project root directory.
- If using cargo: `$ cargo build --release`
- If not using cargo: `$ rustc -0 src/main.rs`

The executable binary should then be available in `./target/release/`

## Running the CLI from anywhere in your file system
Add the following lines to your `.bashrc` file:
```
~/.bashrc
# Obsidian-Block-Link-Code-Generator
export PATH="$PATH:/home/path/to/directory/where/this/program/lives"

alias obc="obsidian_block_link_code_generator"
```

