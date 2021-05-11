# `todo` : minimal command line to-do manager

Built with Rust by a Rust n00b

`todo` is a very minimal tool to manage your to-do list

1. Just a text file
    - `todo` uses a simple text file `todo.txt` to store your todos
    - you can sync the file with version control or any cloud drive of your choice for cross-platform access
    - you can also bulk edit the `todo.txt` file directly with any text editor

2. No projects, no tags, no priorities built in
    - who wants to waste another
    - it is a minimal tool
    - but if you need them you can use these [tricks](#tricks) and `grep`

## Command line interface
You can do 4 basic actions
1. `add` a new todo
    ```
    $ todo add "mail tax forms"
    0 - write project readme
    1 - mail tax forms
    ```
2. `list` all todos
    ```
    $ todo list
    0 - write project readme
    1 - mail tax forms
    ```
3. `edit` an existing todo
    ```
    $ todo edit 0 "write todo project readme"
    0 - write todo project readme
    1 - mail tax forms
    ```
4. make a todo `done`
    ```
    $ todo done 0
    0 - mail tax forms
    ```
### Install

```
cargo install --git https://github.com/akshithg/todo --branch main
```

### Aliasing
add these to your .bashrc, .zshrc
```
alias ll='todo list'
alias lla='todo add'
alias lle='todo edit'
alias lld='todo done'
```

### tricks

to add features such as **assignee**, **topic** etc. you can use a combination of special characters and `grep`

```
$ todo add "update project readme @me #work"
$ todo add "review pull request @jhon #work"
$ todo add "buy grocerries for dinner @me #home"
```

get `#work` todos
```
$ todo list | grep "#work"

update project readme @me #work
todo add "review pull request @jhon #work
```

get todos assigned to `@me`
```
$ todo list | grep "@me"

update project readme @me #work
buy grocerries for dinner @me #home
```
