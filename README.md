# Magic String Search

Magic String Search is a simple yet powerful Rust library designed to find and rank strings based on their similarity to a query string. Whether you're developing a search engine, implementing auto-suggestion functionality, or just need to sort a list of strings by relevance, Magic String Search provides a straightforward API to accomplish these tasks efficiently.

## Features

- **String Similarity Ranking:** Compare and rank strings based on their similarity to a given query string.
- **Longest Common Subsequence (LCS):** Utilizes the LCS algorithm to assess the similarity between strings.
- **Normalized Comparison:** Offers a normalized comparison function to handle varying lengths of strings and queries.

## Installation

Add Magic String Search to your `Cargo.toml` file:

```toml
[dependencies]
magic_string_search = "0.1.5"
```

## Usage

Below is a simple example on how to use Magic String Search to compare two strings and rank multiple strings based on their similarity to a query string:

```rust
use magic_string_search::{compare, rank};

fn main() {
    // Compare two strings
    let similarity_score = compare("Hello, world!", "Hello, Rust!");
    println!("Similarity score: {}", similarity_score);

    // Rank strings by similarity
    let subjects = vec!["Hello, world!", "Hello, Rust!", "Goodbye, world!"];
    let ranked = rank("Hello, Rust!", subjects);
    for (score, subject) in ranked {
        println!("{} - {}", score, subject);
    }
}

```

## Contributing

Contributions to Magic String Search are welcome! Whether it's bug reports, feature requests, or code contributions, please feel free to open an issue or a pull request on our [GitHub repository](https://github.com/DeForestt/string_search).

## License

Magic String Search is released under the MIT License. See the LICENSE file for more details.

You are currently on the free plan, which is significantly limited by the number of requests. To increase your quota, you can check available plans by following [this link](https://c7d59216ee8ec59bda5e51ffc17a994d.auth.portal-pluginlab.ai/pricing).

Useful links: [Website](https://askthecode.ai) | [Documentation](https://docs.askthecode.ai) | [GitHub](https://github.com/askthecode/documentation) | [Twitter](https://twitter.com/askthecode_ai)
