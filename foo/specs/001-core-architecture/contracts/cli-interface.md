# CLI Contract

## Interface Protocol

All CLI interaction follows the text in/out protocol from the constitution
(Principle II):

- **Input**: stdin or CLI arguments
- **Output**: stdout for structured/success output
- **Errors**: stderr for error messages, exit code non-zero on failure

## Command Format

```
foo <command> [<args>]
```

## Output Formats

All commands MUST support at minimum:
- Human-readable text (default)
- JSON output (via `--format json` flag)

## Exit Codes

| Code | Meaning |
|------|---------|
| 0    | Success |
| 1    | General error |
| 2    | Invalid input / usage error |

## Error Output Format

```
Error: <user-friendly message>
```

Errors MUST NOT include:
- Raw debug output (`Debug` trait output)
- Panic traces
- Internal file paths or line numbers
