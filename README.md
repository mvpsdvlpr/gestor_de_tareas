# Task Manager in Rust

This is a simple command-line Task Manager application written in Rust. It allows users to add tasks, list tasks, and mark tasks as completed.

## Features

- Add new tasks with a description.
- List all tasks with their current status (Pending or Completed).
- Mark tasks as completed.

## Getting Started

### Prerequisites

- Rust (Install from [rust-lang.org](https://www.rust-lang.org/))

### Installation

1. Clone the repository:
```sh
   console $ git clone https://github.com/yourusername/task_manager_rust.git
   console $ cd task_manager_rust
```


Build the project:

```sh
cargo build
```

Run the application:
```sh
cargo run
```

## Code Structure
- main.rs: The main entry point of the application containing the logic for task management.
- Task: A struct representing a task with a description and status.
- TaskStatus: An enum representing the status of a task (Pending or Completed).

## Contributing
- Fork the repository.
- Create your feature branch (git checkout -b feature/your-feature).
- Commit your changes (git commit -m 'Add some feature').
- Push to the branch (git push origin feature/your-feature).
- Open a Pull Request.

## License
This project is licensed under the MIT License - see the LICENSE file for details.