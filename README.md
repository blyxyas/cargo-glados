![Glados](https://camo.githubusercontent.com/dea61397c98076fe2bb1ff0a805333f236b8d08e9028bc2b4f7b54f04d98dd96/68747470733a2f2f75706c6f61642e77696b696d656469612e6f72672f77696b6970656469612f656e2f622f62662f476c61646f732e706e67)

# Cargo GLaDOS

GLaDOS, the character from the award-winning video game series _Portal_ will criticize your code. Note that the code is very janky and it was originally for just some laughs in https://github.com/rust-lang/rust-clippy/pull/14510. Sometimes the voice line will wait a few seconds until it starts (allocating the necessary memory for GLaDOS) and it will sometimes wait until it exits compilation. In fact, it may even crash your computer.

It may be the worst cargo wrapper to date, with no intent to be maintained apart from adding new voicelines and maybe some functionality if they seem funny enough.
For the recommended use, alias `cargo` to `~/.cargo/bin/cargo-glados`

