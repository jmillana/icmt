# Copilot, for your commits

`icmt`, spelled "I Commit", automatically generates commit messages based on the current changes in your repository. It's a CLI tool that's inspired by [plz-cli](https://github.com/m1guelpf/plz-cli).

## Installation

You can install `icmt` by running the following command in your terminal:

```
curl -fsSL https://raw.githubusercontent.com/jmillana/icmt/main/install.sh | sh -
```

### Optional
You want to use emojis on commit messages? Install gitmoji, a tool that provides a preset of emojis to get an easy way of identifying the purpose of a commit. Learn more at [gitmoji](https://github.com/carloscuesta/gitmoji).

```bash
npm i -g gitmoji-cli
```

## Usage

`icmt` uses [GPT-3](https://platform.openai.com/docs/guides/gpt). To use it, you'll need to grab an API key from [your dashboard](https://platform.openai.com/account/api-keys), and save it to `OPENAI_API_KEY` as follows (you can also save it in your bash/zsh profile for persistance between sessions).

```bash
export OPENAI_API_KEY='sk-XXXXXXXX'
```

To see a full list of available options, run icmt --help:

```sh
$ icmt --help
Generate commit messages with AI, using chat-gpt-3.5-turbo

Usage: icmt [OPTIONS] <COMMAND>

Commands:
  commit, -c  Autogenerate a commit message
  squash, -s  Autogenerate a squash message
  help        Print this message or the help of the given subcommand(s)

Options:
  -y                               Auto accept the generated commit
  -d, --dryrun                     Show the generated command without executing them
  -t, --token-limit <TOKEN_LIMIT>  Limit the ammout of tokens to be used
  -H, --hint [<HINT>...]           Drive the AI to de-genenerate commit messages that fulfill your desires
  -h, --help                       Print help
  -V, --version                    Print version
```

## Example

```sh
$ icmt commit -e
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
If you want to manually update the generated commit, you can always run the following command and then amend it before pushing the changes to the remote:
```sh
git commit --amend
```
## Develop

Make sure you have the latest version of rust installed (use [rustup](https://rustup.rs/)). Then, you can build the project by running `cargo build`, and run it with `cargo run`.

## License

This project is open-sourced under the MIT license. See [the License file](LICENSE) for more information.
