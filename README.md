# csv2template

Convert csv files to custom templates with [Tera](https://github.com/Keats/tera).

(wip)

## Why?

While Tera is mostly designed for html templates, one use case is converting
[Firefly III](https://github.com/firefly-iii/firefly-iii) csv export files to
other formats like [beancount](https://github.com/beancount/beancount).

Example template is in the [`templates/`](./templates) directory.

## Usage

```bash
cat file.csv | cargo run
```

## Custom templates

All the csv data is available in the template via the `data` variable as an
array of maps.  The map keys are the column names.

### Example

Input csv file:

```csv
date,amount,currency_code,description,source_name,source_type,destination_name,destination_type,category
2020-08-08T00:00:00-07:00,-80.00,USD,Ramen,Discover credit card,Asset account,Hironori Craft Ramen,Expense account,Dining Out
2020-08-04T14:26:20-07:00,-250.00,USD,Buy VTSAX,Debit Account,Asset account,Vanguard,Asset account,Investments
```

Same csv file in tabular form (or view file [here](./example.csv)):

| date                      | amount  | currency_code | description | source_name          | source_type   | destination_name     | destination_type | category    |
| ------------------------- | ------- | ------------- | ----------- | -------------------- | ------------- | -------------------- | ---------------- | ----------- |
| 2020-08-08T00:00:00-07:00 | -80.00  | USD           | Ramen       | Discover credit card | Asset account | Hironori Craft Ramen | Expense account  | Dining Out  |
| 2020-08-04T14:26:20-07:00 | -250.00 | USD           | Buy VTSAX   | Debit Account        | Asset account | Vanguard             | Asset account    | Investments |

The `data` variable in the template will consist of the following:

```json
[
    {
        "amount": "-80.00",
        "category": "Dining Out",
        "currency_code": "USD",
        "date": "2020-08-08T00:00:00-07:00",
        "description": "Ramen",
        "destination_name": "Hironori Craft Ramen",
        "destination_type": "Expense account",
        "source_name": "Discover credit card",
        "source_type": "Asset account"
    },
    {
        "amount": "-250.00",
        "category": "Investments",
        "currency_code": "USD",
        "date": "2020-08-04T14:26:20-07:00",
        "description": "Buy VTSAX",
        "destination_name": "Vanguard",
        "destination_type": "Asset account",
        "source_name": "Debit Account",
        "source_type": "Asset account"
    }
]
```

## Planned features

Priority:

- [ ] Load custom template from input arguments
- [ ] Cleaner error messaging
- [ ] Easier replacements / renaming
- [ ] Better working beancount template

Nice to have:

- [ ] Live preview of template modifications
