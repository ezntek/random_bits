# random_bits
random things (wtfpl)

# notes on committing python scripts

always include a few newlines at the start of the file or else you cant chain the scripts.

# running the python utility scripts
|name|script|
|----|------|
|basic calculator utility functions|`python -i -c "$(curl -fsSL 'https://raw.githubusercontent.com/ezntek/random_bits/main/py/prelude.py') $(curl -fsSL 'https://raw.githubusercontent.com/ezntek/random_bits/main/py/calculator_functions.py')"`|

## running multiple scripts + starting a repl at once

`python -i -c "$(curl -fsSL 'https://raw.githubusercontent.com/ezntek/random_bits/main/py/your_script.py') $(curl -fsSL 'https://raw.githubusercontent.com/ezntek/random_bits/main/another_script.py')"`
