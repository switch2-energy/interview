# Switch2 Pair Programming Interview

We want you to present the best version of yourself possible when interviewing with us and we don't want the interview pressure to get to you. However, we receive a lot of applications and can only hire a limited number of candidates, therefore we have to find a way to find the best candidate for each role. To do this we go will go through the following pair programming exercise with you.

## The Exercise

We have prepared a very small library beginning a new billing system. The core of the package is a function which tries to work out how much a customer should be billed given an input csv containing fake meter consumption data. The csv data contains things like the starting meter read and the end meter read, along with some simplified tariff data, and the `calculate_charges` function attempts to calculate the total cost the customer has incurred. For example, if someone used 10 units of energy at a cost of £1/unit, then the total cost should be £10.

The Rust code has some unit tests with it that are failing and the code is written intentionally to encourage refactoring. Your task is simply to pass the tests and refactor the code to your liking. You will work through the tests with another software engineer from Switch2. Treat them as a peer in a pair programming setting; they will be able to answer any Rust questions you have.

## What we are looking for

- Pairing: How do you work and collaborate with others?
- Type-Driven Development: How do you think about types and correctness?
- Testing: How do you test code?
- Code: How do you write code?

We will score each of these criteria on a scale of 1 to 4.

## Tech Stack - Rust & Nix

We use Rust at Switch2 and don't expect you to know any Rust or have Rust and its dependencies installed on your machine, so we are using Nix in this repo to manage the Rust dependencies (Rust, openssl, pkg-config). This Nix subshell will contain all you need to run the code and tests.

### Help: I don't know Rust!

Since we don't expect you to know any Rust, you might wonder how you can progress through this exercise. The intention of the code here is not to write idiomatic Rust, but to write simple enough Rust that we feel anyone coming from different languages could understand and in the pair-programming setting will be able to ask the right questions to help not get stuck. If you join Switch2 you will be writing Rust so this is a good opportunity for you to see if you like it and us to see if you want to learn it and can pair productively with it.

## How to Build/Run

### Install Nix

https://nixos.org/download/

### Run Nix Subshell

If it's the first time running nix, run: 

```bash
nix develop --extra-experimental-features nix-command --extra-experimental-features flakes
```

If you have already enabled experimental-features:

```bash
nix develop
```

### Run Tests

Once inside the subshell run:

```bash
cargo test
```
