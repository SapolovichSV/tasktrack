# Task Manager CLI in Rust

Welcome to **Task Manager CLI**, a simple command-line application written in Rust. This is my first project in Rust, created to explore the language's features, idioms, and ecosystem. The project implements a basic task manager that allows users to add, update, delete, and query tasks stored locally on the file system.

---

## Features

- **Command Pattern Implementation**: The application uses the Command Pattern to encapsulate operations, making the code clean, modular, and easy to extend.
- **Task Management**: Perform common task management operations like:
  - Adding a new task
  - Updating an existing task
  - Deleting a task
  - Marking tasks as "done," "in progress," or "not done"
  - Clearing completed tasks
- **Query Capabilities**: List tasks by their statuses:
  - All tasks
  - Completed tasks
  - Tasks in progress
  - Pending tasks
- **File-Based Storage**: Tasks are stored as JSON files in a directory, providing persistence without requiring a database.
- **Explicit Argument Handling**: The CLI requires exactly three arguments for every command: 
  1. The command itself.
  2. The task ID (if not needed, use a  `_`).
  3. The task name (if not needed, use a  `_`).

---

## Architecture

This project is structured into multiple modules to promote separation of concerns and maintainability:

### 1. **Commands**
Defines various commands (e.g., `add`, `update`, `list-all`) and their behavior. It uses an enum to encapsulate the supported commands and categorizes them as either **Querying** or **Modifying**.

### 2. **Config**
Parses and validates command-line arguments to build a configuration object that the application can execute. It ensures that all three arguments are provided, even if they are placeholders (`_`).

### 3. **Entities**
Contains core entities like `Task`, `TaskStatus`, and enums to represent commands (`Commands`) and their kinds (`CommandKind`).

### 4. **Storage**
Implements persistence for tasks using the file system. Provides methods to add, update, delete, and query tasks. The `fsstorage` module handles JSON serialization and file management.


## How It Works

1. **Command-Line Interface**: The user interacts with the application by providing three arguments via the CLI:
   - `<command>`: The operation to perform (e.g., `add`, `list-all`).
   - `<task_id>`: The ID of the task (use `_` if not applicable).
   - `<task_name>`: The name of the task (use `_` if not applicable).
   - Build the project:
    cd /directory/with/project
    cargo build --release
   tacktrack <command> <task_id> <task_name>
   For example:
   ```bash
   tasktrack add - "Finish Rust project"
   tasktrack list-all _ _
   tasktrack mark-done 1 _
