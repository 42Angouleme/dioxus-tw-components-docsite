# Docsite

A docsite to showcase, explore the [dioxus-tw-components library](https://github.com/42Angouleme/dioxus-components) and even export custom themes to embed in your own projects.

## Build

To build the docsite you first need to build the `tailwind.css` file using [TailwindCSS 3](https://v3.tailwindcss.com/).
[Here](https://v3.tailwindcss.com/docs/installation) you'll find how to install it.

Then run this command in the project's root directory:
```bash
npx tailwindcss -c ./tailwind/tailwind.config.js -i ./tailwind/input.css -o ./assets/tailwind.css
```

Install the dioxus-cli binary using this command:
```bash
cargo install dioxus-cli
```

And just run this from the `Cargo.toml` directory:
```bash
dx serve
```
Or this one if you want to bundle the docsite:
```bash
dx bundle
```

## License

This project is licensed under either the MIT license or the Apache-2 License.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Dioxus by you, shall be licensed as MIT or Apache-2, without any additional terms or conditions.
