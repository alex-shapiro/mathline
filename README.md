# Mathline

Natural language solver for mathematical expressions

## Examples

**Arithmetic**

```
mathline "what is the cosine of three times eleven?"
LLM: cos(3 * 11)
Parse: cos(3 * 11)
Step: 3 * 11 => 33
Answer: -0.013276747223059479
```

**Logic**

```
mathline "is twenty squared equal to 400?"
LLM: 20^2 = 400
Parse: 20 ** 2 == 400
Step: 20 ** 2 => 400
Step: 400 == 400 => true
true
```

## How it works

Mathline works in three steps:

- it calls an LLM to translate your question into a mathematical expression
- it parses that mathematical expression
- it evaluates the parsed expression

## Supported LLMs

Mathline supports three LLMs:

- Default: [Ollama](https://ollama.com/) with the Gemma 3 4B model. Once installed, it runs locally on your device.
- Claude API
- OpenAI API
