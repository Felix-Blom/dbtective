# TEST001: tests

**Category:** Testing  
**Short Description:** Models must have at least one test

Derived from dbt testing best practices.

## What it does

Checks for dbt models that lack data tests to ensure data quality.

## Why is this bad?

Models without tests can introduce data quality issues that go undetected. Tests help ensure data integrity, catch issues early in the development cycle, and provide confidence when making changes to your data models.

## Example

**Bad:**
```yaml
models:
  - name: users
    description: "User data from our application"
    columns:
      - name: user_id
      - name: email
```

```sql
{{ config(
    materialized='table'
) }}

select 
    user_id,
    email
from {{ ref('raw_users') }}
```

**Good:**
```yaml
models:
  - name: users
    description: "User data from our application"
    tests:
      - dbt_utils.row_count:
          above: 0
    columns:
      - name: user_id
        description: "Unique identifier for users"
        tests:
          - unique
          - not_null
      - name: email
        description: "User email address"
        tests:
          - unique
          - not_null
```

```sql
{{ config(
    materialized='table'
) }}

select 
    user_id,
    email
from {{ ref('raw_users') }}
where email is not null
```

## Options

- `require_model_tests`: Require tests at the model level (default: false)
- `require_column_tests`: Require tests on specific columns (default: ["primary_key", "foreign_key"])
- `min_test_count`: Minimum number of tests required per model (default: 1)
- `required_test_types`: List of required test types (default: ["not_null"])