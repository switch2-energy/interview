# Switch2 Pair Programming Interview

We want you to present the best version of yourself possible when interviewing with us and we don't want the interview pressure to get to you. However, we receive a lot of applications and can only hire a limited number of candidates, therefore we have to find a way to find the best candidate for each role. To do this we go will go through the following pair programming exercise with you.

## The Exercise

We have prepared a small library beginning a new billing system. The library has some unit tests with it that are either failing or not finished. Your task is simply to pass the tests. You will work through the tests with another software engineer from Switch2.

## What we are looking for

- Pairing: (1) solo hero → (4) collaborative, concise, kind.
- TDD: (1) code then tests → (4) crisp red/green/refactor, naming sings.
- Types: (1) strings/nums only → (4) illegal states unrepresentable.
- Code: (1) tangled → (4) small, intention-revealing functions.

We will score each of these criteria on a scale of 1 to 4.

## Tech Stack - Rust & Nix

We use Rust at Switch2 and don't expect you to know any Rust. We don't expect you to have Rust and its dependencies installed on your machine, so we are using Nix in this repo to manage the Rust dependencies (Rust, openssl, pkg-config). This Nix subshell will contain all you need to run the code and tests.

### How to Build/Run

#### Install Nix

https://nixos.org/download/

#### Run Nix Subshell

```bash
nix develop
```

#### Run Tests

Once inside the subshell run:

```bash
cargo test
```
