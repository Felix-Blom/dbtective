# TEST003: column-tests

**Category:** Testing  
**Short Description:** Specific columns must have tests

Derived from dbt testing best practices.

## What it does

Checks for specific column types (like primary keys, foreign keys) that lack tests.

## Why is this bad?

Critical columns like primary keys and foreign keys should always be tested to ensure data integrity. Missing tests on these columns can lead to downstream data quality issues.

## Example

**Bad:**
```yaml
# models/schema.yml
models:
  - name: users
    description: "User data"
    columns:
      - name: user_id  # Primary key without tests
        description: "Primary key"
      - name: company_id  # Foreign key without tests
        description: "Foreign key to companies"
```

**Good:**
```yaml
# models/schema.yml
models:
  - name: users
    description: "User data"
    columns:
      - name: user_id
        description: "Primary key"
        tests:
          - unique
          - not_null
      - name: company_id
        description: "Foreign key to companies"
        tests:
          - not_null
          - relationships:
              to: ref('companies')
              field: id
```

## Options

- `required_column_patterns`: Patterns that identify columns requiring tests (default: ["*_id", "id"])
- `required_tests_per_column`: Required tests for matched columns (default: ["not_null"])