# Gemini Code Assist Configuration

This directory contains configuration files for Gemini Code Assist, Google's AI-powered code review assistant.

## Files

### `config.yaml`

Main configuration file for Gemini Code Assist behavior on Pull Requests.

**Key settings:**

- `code_review.comment_severity_threshold`: Controls minimum severity level for review comments (LOW, MEDIUM, HIGH)
- `code_review.max_review_comments`: Maximum number of comments per PR (-1 = unlimited)
- `ignore_patterns`: Glob patterns for files to exclude from review

### `styleguide.md`

Custom coding style guide and best practices that Gemini should follow during code reviews.

## File Exclusion Behavior

**Important:** By default, Gemini Code Assist GitHub app **respects `.gitignore` files**. This means:

- Files listed in `.gitignore` are excluded from PR reviews
- Hidden configuration files (`.github/`, `.prettierrc.yml`, etc.) in `.gitignore` won't be reviewed

**Solutions:**

**`ignore_patterns` in config.yaml**: Add additional patterns to exclude

- Uses glob patterns
- Can use `!pattern` for explicit inclusions

## References

- [Gemini Code Assist Documentation](https://cloud.google.com/gemini/docs/discover/set-up-gemini)
- [GitHub App Configuration](https://cloud.google.com/gemini/docs/code-assist/configure-reviews)
