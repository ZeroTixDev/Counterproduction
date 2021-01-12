<!-- markdownlint-disable no-inline-html no-bare-urls line-length header-increment commands-show-output first-line-heading -->
<h1 align="center">Counterproduction</h1>

<p align="center">
<a href="https://discord.gg/GCz7KgG">
    <img src="https://img.shields.io/discord/726947023231647798.svg?logo=discord&style=flat-square&color=7289DA">
</a>
<a href="https://github.com/Counterproduction-game/Counterproduction/blob/main/LICENSE.md">
    <img src="https://img.shields.io/github/license/Counterproduction-game/Counterproduction?color=%2339c48a&style=flat-square">
</a>
<a href="https://github.com/Counterproduction-game/Counterproduction/">
    <img src="https://img.shields.io/badge/language-rust-000?logo=rust&style=flat-square">
</a>
<a href="https://github.com/Counterproduction-game/Counterproduction">
    <img src="https://img.shields.io/tokei/lines/github/Counterproduction-game/Counterproduction?style=flat-square&color=417fa3">
</a>
<a href="https://github.com/Counterproduction-game/Counterproduction/graphs/commit-activity">
    <img src="https://img.shields.io/github/commit-activity/w/Counterproduction-game/Counterproduction?color=%234287f5&logo=github&style=flat-square">
</a>
<a href="https://github.com/Counterproduction-game/Counterproduction/actions">
    <img src="https://img.shields.io/github/workflow/status/Counterproduction-game/Counterproduction/Rust%20Prototype?style=flat-square&logo=github-actions&logoColor=fff">
</a>
</p>
<p align="center">
    <strong> <a href="https://youxplode.com/md.html?Counterproduction">About</a> </strong>
</p>

TLDR: This is a game in which players will counter others by creating combat spaceship designs on the fly.

## Compiling

To compile the Rust part of the program, run

```bash
> cargo build
```

This assumes that you have [`rustup`](https://rustup.rs/) installed and a nightly version of the compiler as the default.

To run the opening animation page, execute<sup>[1](#fn1)</sup>

```bash
> pnpm install
> cd test
> pnpm run serve
```

## Prototype

There is also a very simple prototype<sup>[2](#fn2)</sup> that can be accessed by simply opening `file:///path/to/Counterproduction/prototype/index.html`. The space button can be used to pause the "game". Other controls are shown in the `Player 0's Resources ...` section.

---

<a name="fn1">1</a>: Install `pnpm` using `npm install --global pnpm`.

<a name="fn2">2</a>: Nothing in this will be kept; its purely for testing purposes.

<a href="https://app.fossa.com/projects/git%2Bgithub.com%2FiMplode-nZ%2FAutofactory?ref=badge_large" alt="FOSSA Status"><img src="https://app.fossa.com/api/projects/git%2Bgithub.com%2FiMplode-nZ%2FAutofactory.svg?type=large"/></a>
