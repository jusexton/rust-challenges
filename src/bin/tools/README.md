# tools

A CLI for managing challenge files and keeping the README in sync.

## Commands

### `scaffold`

Creates a new challenge file and inserts a corresponding entry into the README.

```sh
cargo run --bin tools -- scaffold --url <URL>
```

**Example**

```sh
cargo run --bin tools -- scaffold --url https://leetcode.com/problems/two-sum/
```

This command will:

1. Parse the problem slug from the URL (`two-sum`)
2. Create `src/leetcode/two_sum.rs` pre-populated with a function stub and empty test module
3. Fetch the problem difficulty (`Easy` / `Medium` / `Hard`) from the LeetCode GraphQL API
4. Insert a linked entry into the correct difficulty section of the README

> **Note:** `scaffold` makes a network request to `leetcode.com` to look up the problem difficulty. If the difficulty can not be retrieved, the README update is skipped.

---

### `verify-readme`

Checks that every challenge file in `src/leetcode/` has a corresponding entry in the README. Exits with a non-zero status and prints the offending slugs if any are missing.

```sh
cargo run --bin tools -- verify-readme
```

This is useful as a sanity check after manually adding challenge files or editing the README. It is also suitable for use in CI.

**Example output when out of sync**

```
Error: The README file is missing the corresponding challenge entries:
  two_sum
  binary_search
```

**Example output when everything is in sync**

```
No README.md issues detected!
```
