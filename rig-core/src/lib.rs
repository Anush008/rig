//! Rig is a Rust library for building LLM-powered applications that focuses on ergonomics and modularity.
//!
//! # Table of contents
//!
//! - [High-level features](#high-level-features)
//! - [Simple Example](#simple-example)
//! - [Core Concepts](#core-concepts)
//! - [Integrations](#integrations)
//!
//! # High-level features
//! - Full support for LLM completion and embedding workflows
//! - Simple but powerful common abstractions over LLM providers (e.g. OpenAI, Cohere) and vector stores (e.g. MongoDB, in-memory)
//! - Integrate LLMs in your app with minimal boilerplate
//!
//! # Simple example:
//! ```
//! use rig::{completion::Prompt, providers::openai};
//!
//! #[tokio::main]
//! async fn main() {
//!     // Create OpenAI client and model.
//!     // This requires the `OPENAI_API_KEY` environment variable to be set.
//!     let openai_client = openai::Client::from_env();
//!
//!     let gpt4 = openai_client.agent("gpt-4").build();
//!
//!     // Prompt the model and print its response
//!     let response = gpt4
//!         .prompt("Who are you?")
//!         .await
//!         .expect("Failed to prompt GPT-4");
//!
//!     println!("GPT-4: {response}");
//! }
//! ```
//! Note: using `#[tokio::main]` requires you enable tokio's `macros` and `rt-multi-thread` features
//! or just `full` to enable all features (`cargo add tokio --features macros,rt-multi-thread`).
//!
//! # Core concepts
//! ## Completion and embedding models
//! Rig provides a consistent API for working with LLMs and embeddings. Specifically,
//! each provider (e.g. OpenAI, Cohere) has a `Client` struct that can be used to create completion
//! and embedding models. These models implement the [CompletionModel](crate::completion::CompletionModel)
//! and [EmbeddingModel](crate::embeddings::EmbeddingModel) traits respectively, which provide a common,
//! low-level interface for creating completion and embedding requests and executing them.
//!
//! ## Agents
//! Rig provides high-level abstractions over LLMs in the form of the [Agent](crate::agent::Agent) type.
//!
//! The [Agent](crate::agent::Agent) type can be used to create anything from simple agents that use vanilla models to full blown
//! RAG systems that can be used to answer questions using a knowledge base.
//!
//! ## Vector stores and indexes
//! Rig provides a common interface for working with vector stores and indexes. Specifically, the library
//! provides the [VectorStore](crate::vector_store::VectorStore) and [VectorStoreIndex](crate::vector_store::VectorStoreIndex)
//! traits, which can be implemented to define vector stores and indices respectively.
//! Those can then be used as the knowledgebase for a RAG enabled [Agent](crate::agent::Agent), or
//! as a source of context documents in a custom architecture that use multiple LLMs or agents.
//!
//! # Integrations
//! Rig natively supports the following completion and embedding model providers:
//! - OpenAI
//! - Cohere
//! - Anthropic
//! - Perplexity
//!
//! Rig currently has the following integration companion crates:
//! - `rig-mongodb`: Vector store implementation for MongoDB
//! - `rig-lancedb`: Vector store implementation for LanceDB

pub mod agent;
pub mod cli_chatbot;
pub mod completion;
pub mod embeddings;
pub mod extractor;
pub(crate) mod json_utils;
pub mod loaders;
pub mod providers;
pub mod tool;
pub mod vector_store;
