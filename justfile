
default:
    @just --list --list-heading $'Available commands:\n'

alias t := test
alias e := example
alias i := install

test test_name='': 
    if [ {{test_name}} = '' ]; then \
    cargo test ; \
    else \
    cargo watch -q -c -x "test {{test_name}}"; \
    fi

example ex_file_name:
    cargo watch -q -c -x "run -q --example {{ex_file_name}}"

run_once ex_file_name:
    cargo run --example {{ex_file_name}}

install example_name='': 
    if [ {{example_name}} = '' ]; then \
    cargo watch -x "install --path ."; \
    else \
    cargo watch -x "install --example {{example_name}} --path ."; \
    fi