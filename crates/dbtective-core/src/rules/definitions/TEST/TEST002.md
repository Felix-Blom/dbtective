# TEST002: model-tests

**Category:** Testing  
**Short Description:** Models must have model-level tests

Derived from dbt testing best practices.

## What it does

Checks for dbt models that lack model-level tests (tests defined at the model level, not column level).

## Why is this bad?

Model-level tests validate the entire dataset and can catch issues that column-level tests might miss, such as row count validation, referential integrity, or business logic validation across multiple columns.

## Example

**Bad:**
```yaml
# models/schema.yml
models:
  - name: users
    description: "User data"
    columns:
      - name: user_id
        tests:
          - unique
          - not_null
    # No model-level tests
```

**Good:**
```yaml
# models/schema.yml
models:
  - name: users
    description: "User data"
    tests:
      - dbt_utils.row_count:
          above: 0
      - dbt_expectations.expect_table_row_count_to_be_between:
          min_value: 1000
          max_value: 100000
    columns:
      - name: user_id
        tests:
          - unique
          - not_null
```

## Options

- `min_model_tests`: Minimum number of model-level tests required (default: 1)
- `required_model_test_types`: List of required model test types (default: [])