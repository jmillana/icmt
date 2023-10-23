# Copilot, for your commits

icmt, spelled "I Commit", automatically generate commit messages for the current changes.
A CLI tool that generates commit messages from repository state.

This poject is highly ispired by https://github.com/m1guelpf/plz-cli

## Installation

You can install `icmt` by running the following command in your terminal.

```
curl -fsSL https://raw.githubusercontent.com/jmillana/icmt/main/install.sh | sh -
```

## Usage

`icmt` uses [GPT-3](https://beta.openai.com/). To use it, you'll need to grab an API key from [your dashboard](https://beta.openai.com/), and save it to `OPENAI_API_KEY` as follows (you can also save it in your bash/zsh profile for persistance between sessions).

```bash
export OPENAI_API_KEY='sk-XXXXXXXX'
```

Once you have configured your environment, run `icmt` followed by whatever it is that you want to do (`icmt show me all options for the icmt cli`).

To get a full overview of all available options, run `imct --help`

```sh
$ icmt --help
Generates commit messages from the command line.

With the `commit` mode the system will check if there is any file ready to be commited
In case --gitmoji option is set it will try to generate the commit using emojis.

Usage: icmt [OPTIONS] <PROMPT>

Arguments:
  <PROMPT>

Options:
  -m, --mode     [default: commit] Currently only commit mode is implemented
  -e, --gtmoji   Use gitmoji emojis for the commit message
  -y,            Auto accept the generated commit message
  -t, --token-limit Set the maximum amount of tokens that can be used per request
  -H, --hint     Set a hint to be used to give extra context to the generated responses
  -h, --help     Print help information
  -V, --version  Print version information
```

## Example

```sh
$ icmt -e
🤖 Welcome to commit AI!
✔ Got some results!
───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
:memo: Update README.md with build and run instructions

Update the README.md file with instructions on how to build and run the project. Provide information on installing the latest version of Rust using rustup, and the commands `cargo build` and `cargo run` to build and run the project.

:bug: fix: Correct typo in command example

Fix a typo in the command example by changing `-y` to `-t` to match the correct option for setting the maximum amount of tokens that can be used per request.
───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
>> Refine the prompt? [y/N] y
>> Enter your refinement:  Remove the bug section
✔ Refined result!
───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
:memo: Update README.md with build and run instructions

Update the README.md file with instructions on how to build and run the project. Provide information on installing the latest version of Rust using rustup, and the commands `cargo build` and `cargo run` to build and run the project.
───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
>> Refine the prompt? [y/N]
───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
📝 Update README.md with build and run instructions

Update the README.md file with instructions on how to build and run the project. Provide information on installing the latest version of Rust using rustup, and the commands `cargo build` and `cargo run` to build and run the project.
───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
>> Apply the generated commit? [Y/n]
───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
git commit -m '📝 Update README.md with build and run instructions

Update the README.md file with instructions on how to build and run the project. Provide information on installing the latest version of Rust using rustup, and the commands `cargo build` and `cargo run` to build and run the project.
───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
✔ Command ran successfully
[main 0b7bed5] 📝 Update README.md with build and run instructions
 1 file changed, 5 insertions(+), 1 deletion(-)
```

## Develop

Make sure you have the latest version of rust installed (use [rustup](https://rustup.rs/)). Then, you can build the project by running `cargo build`, and run it with `cargo run`.

## License

This project is open-sourced under the MIT license. See [the License file](LICENSE) for more information.
