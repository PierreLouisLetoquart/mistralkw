# Ollama keywords

This repo aims to demonstrate how to create a super simple custom model using Ollama for keywords generation.

## Create the model

The simplest way to "finetune" a LLM and use it locally is Ollama. We will create a [Modelfile](https://github.com/ollama/ollama/blob/main/docs/modelfile.md) and create our custom LLM with the `ollama create` command.

The `Modelfile` is super simple and can be modify depending on your use case.

Then run the following command :

```bash
ollama create <model-name> -f </path/to/Modelfile>
```

Great ! To test the result, enter the command `ollama run <model-name>` and play with it.

## Usage with Rust

This document is a part of a Rust project, so we'll do the programmed logic in Rust. We will use [ollama-rs](https://github.com/pepperoni21/ollama-rs) to play with the model generated previously.



