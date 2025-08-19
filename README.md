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

**Word Problems**

```sh
mathline "if bob has three apples, jane has two, and carl has eleven, how many apples do they have together?" --model gpt-oss:20b
LLM: 3 + 2 + 11
Parse: 3 + 2 + 11
Step: 3 + 2 => 5
Step: 3 + 2 + 11 => 16
Answer: 16
```

## How it works

Mathline works in three steps:

- it calls an LLM to translate your question into a mathematical expression
- it parses that mathematical expression
- it evaluates the parsed expression

## Ollama support

Mathline calls out to your device's local [Ollama](https://ollama.com/) server.
The default model is `gemma3:4b`. You can override the model with the `--model` CLI option. For example:

```sh
mathline "what is two cubed?" --model gpt-oss:20b
```
