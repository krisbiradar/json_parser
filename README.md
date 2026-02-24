# parse_light

[![Crates.io](https://img.shields.io/crates/v/parse_light.svg)](https://crates.io/crates/parse_light)

A lightweight and extensible **JSON parser** available at [https://crates.io/crates/parse_light](https://crates.io/crates/parse_light). Built as a hobby project, with the goal of evolving it into a **production-ready, well-tested, and performant** library.

This project focuses on understanding JSON parsing internals, building clean abstractions, and gradually improving reliability, performance, and developer experience.

---

## ✨ Features

- ✅ Parses valid JSON structures
- ✅ Supports:
  - Objects
  - Arrays
  - Strings
  - Numbers
  - Booleans
  - `null`
- ✅ Clear error reporting for invalid JSON
- ✅ Minimal dependencies
- 🚧 Designed to be extended (streaming, validation, performance optimizations)

---

## 📦 Project Status

> **Work in Progress**

This project started as a learning exercise and is actively evolving toward:
- Production-grade code quality
- Better performance
- Robust error handling
- Comprehensive test coverage
- Clear public APIs

Expect breaking changes while the API stabilizes.

---

## 🧠 Motivation

Most developers *use* JSON parsers, but few ever *build* one.

This project exists to:
- Deepen understanding of parsing techniques
- Explore tokenization and grammar handling
- Practice writing clean, maintainable parsing code
- Gradually apply real-world engineering practices

---

## 🚀 Getting Started

### Clone the repository

```bash
git clone https://github.com/krisbiradar/json_parser.git
cd json_parser
