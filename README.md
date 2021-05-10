# A simple command line task manager

Built with Rust

## A super simple tool

1. Just a text file
    - `task` uses a simple text file `todo.txt` to store your tasks
    - you can sync the file with version control or any cloud drive of your choice
    - bulk edit the `todo.txt` file directly with any text editor

2. No projects, no tags, no priorities built in
    - it is a super simple tool
    - but if you need them it is not hard either with a combination of `!status`, `#topic`, `@person` and `grep`

## Command line interface
This is a very simple command line task manager
Main set of features include
1. add a new task
    ```
    $ task write project readme
    $ task --add write project readme
    1 - mail tax docs
    2 - write project readme
    ```
2. view all tasks
    ```
    $ task
    $ task --list/-l
    ```
3. edit an existing task
    ```
    $ task --edit/-e 2 write tasks project readme
    1 - mail tax docs
    2 - write tasks project readme
    ```
4. make a task complete
    ```
    $ task --done/-d 1
    1 - write tasks project readme
    ```

## Usage

1. Installing
2. Setting up an alias
3. Customization
