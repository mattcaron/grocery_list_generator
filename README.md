# grocery_list_generator

An application to generate a formatted grocery lists from a text input file.

## Introduction

This is a custom application which used to perform a very specific task, for a very picky customer set - my sons Max and Miles. We used to go grocery shopping and split up the list between the two of them, and they read their list and collect their goods. The master list (mine) served as a check on this.

However, as time progressed, we learned that:
1. Max only wanted to come some days.
1. Miles wanted the full list himself.

As such, this has collapsed into a single list, printed as many times as needed.

The output from this application's companion application, [emeals_getter](https://github.com/mattcaron/emeals_getter), can be used as input for this.

## Usage

1. Make a list of groceries, one entry per line.
1. Run `grocery_list_generator <listname>`.
1. The `.tex` will be created with the same stem as `<listname>` above (so, `groceries.txt` becomes `groceries.tex`).
1. Generate the PDF from that `.tex` file. (See [my LaTeX scripts](https://github.com/mattcaron/latex_scripts) for help with this.)

## TODO

- Add test code.
- There is much copypasta tech debt that needs refactoring.
- Add a license file.