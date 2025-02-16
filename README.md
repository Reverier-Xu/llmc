# LLM in Console

Maybe a simple chat tool for terminal users.

## Usage (WIP)

```bash
# oneshot question
llmc <your question>

# ... answers from LLM
```

```bash
# change profile (configurable)
llmc -p r1 <your question>

# ... answers from LLM
```

```bash
# continuous chat
llmc -s
>> <your question 1>
# ... answers from LLM
>> <your question 2>
# ... answers from LLM
```

```bash
# translate, the default source / destination language is configurable.
llmt -s zh_CN -d en_US <word or paragraph>

# ... translation from LLM
```
