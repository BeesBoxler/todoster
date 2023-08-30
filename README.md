# TODOSTER
## Usage
```terminal
$ todoster
```
Will print out all of the `TODO` items in the current and sub directories.

You can also use this to output directly to a markdown file with 
```terminal 
$ todoster --format md > todo.md
```
which will output the following:
```markdown
- [ ] __TODO:__ tidy this up _(./src/main.rs:13)_
- [ ] __TODO:__ This will be removed once more options are added _(./src/main.rs:34)_
```

- [ ] __TODO:__ tidy this up _(./src/main.rs:13)_
- [ ] __TODO:__ This will be removed once more options are added _(./src/main.rs:34)_

