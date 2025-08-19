# Mathline

Natural language solver for mathematical expressions

## Examples

**Arithmetic**

```sh
mathline "what is the cosine of three times eleven?"
LLM: cos(3 * 11)
Parse: cos(3 * 11)
Step: 3 * 11 => 33
Step: cos(3 * 11) => -0.01328
Answer: -0.01328
```

**Logic**

```sh
mathline "is twenty squared equal to 400?"
LLM: 20^2 = 400
Parse: 20 ** 2 == 400
Step: 20 ** 2 => 400
Step: 20 ** 2 == 400 => true
Answer: true
```

## How it works

Mathline works in three steps:

- it calls an LLM to translate your question into a mathematical expression
- it parses that mathematical expression
- it evaluates the parsed expression

## Ollama support

Mathline calls out to [Ollama](https://ollama.com/). The default model is `gemma3:4b`.
You can override the model with the `--model` CLI option. For example:

```sh
mathline "what is two cubed?" --model gpt-oss:20b
```
