# Introduction to Apache Arrow with Rust

This article gives an overview of Apache Arrow. The focus is introducing the library and exploring its primary use cases. Important features are explored at a high level, and code samples are used to provide examples.

## Publication

This article has been publised in [Level Up Coding](https://bit.ly/apache-arrow-with-rust)

## Outline

### What is Apache Arrow?

- Description of the Arrow Project
- In-memory columnar format
- Language-agnostic specification

### Rust Implmentation

#### Low Level Arrays

- What are arrays?
- Initializing arrays
- Array builders

#### High Level Constructs

- Field
- Schema
- RecordBatch

#### Data Readers

- CSV Reader

#### Compute Kernels

- Filter
- Sort
- Group By
