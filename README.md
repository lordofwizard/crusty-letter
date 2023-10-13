# Crusty Letter

Crusty Letter is a daily communication curated with an assortment of jokes, news, a profound quote, and a selection of recommended songs. It's designed to provide a lighthearted start to your day.

## Motivation

The motivation behind the Crusty Letter project was to reimagine and rewrite an existing application utilizing Rust, a powerful and memory-safe programming language. This reimplementation served as an opportunity to gain insights into ETL (Extract, Transform, Load) pipelines while utilizing Rust's robust ecosystem. Heavily Inspired by Akash's Project called [Cereal-Letter]([https://github.com/akashkham
](https://github.com/akashjkhamkar/cereal-letter) 🤝.
## Stack

- [Rust](https://www.rust-lang.org/): A) systems programming language known for its speed, memory safety, and concurrency support.
- [reqwest](https://docs.rs/reqwest): An ergonomic HTTP client for Rust, making requests easy and efficient.
- [serde](https://serde.rs/): A fast and efficient library for serializing and deserializing data in Rust.
- [genpdf](https://docs.rs/genpdf): A simple and convenient crate for generating PDF documents in Rust.

## Working

The Crusty Letter workflow is orchestrated through Rust's robust concurrency model, ensuring efficient execution of tasks.

1. **Data Collection**:
   - Information is gathered from various open APIs using the `reqwest` crate.
   - The collected data is then merged together to create a unified dataset.

2. **Data Storage**:
   - The combined data is securely stored in a MongoDB database.

3. **PDF Generation**:
   - The stored data is leveraged to generate a PDF document, incorporating jokes, news, quotes, and song recommendations.

4. **Scheduled Delivery**:
   - The PDF document is sent to subscribers at the designated time, providing a daily dose of light-hearted content.

## How to Run

1. Launch the services with:

    ```bash
    docker-compose up
    ```

2. Once all services are up, visit [http://localhost:8080](http://localhost:8080) to access the Crusty Letter console.

3. Trigger the `crusty-letter` operation within the console.

## PDF Storage

Generated PDFs are stored in the `/temp` directory.

## Environment Setup

To enable email functionality, you'll need to configure your [sendgrid.com](http://sendgrid.com) account. After setup, add your email and API key in the `.env` file. This will ensure successful email operations.

## Future Scope

In the future, Crusty Letter aims to implement multi-user subscription functionality. This enhancement will allow users to receive tailored letters based on their individual preferences.

---

### Dependencies

- [reqwest](https://docs.rs/reqwest): An ergonomic HTTP client for Rust.
- [serde](https://serde.rs/): A serialization and deserialization library for Rust.
- [genpdf](https://docs.rs/genpdf): A Rust crate for generating PDF documents.

### Tech Stack

- Rust
- Linux

This project showcases Rust's prowess in creating performant and safe applications, ensuring a seamless and delightful user experience.
