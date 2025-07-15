# psar
## Daemon:
### First Fork:
 - We need to fork a parent process to have it run in the background.
 - When the parent exits, the child is no longer a session leader
 - Session leaders can reacquire a controlling terminal
 - Forking removes session leader status
 - Automatically inherrits the open file descriptors stdout and stderr from parent
### SID:
 - A session is a collection of related processes usually associated with a user terminal
 - The session leader is the process that creates the session and its PID is SID
 - We need to detach from the controlling terminal 
 - setSID detach from the old terminal
 - Does not close stdout and stderr
### Second Fork:
 - After we detach from the old terminal we want to remove the session leader status so that this process cannot create a new terminal
### Systemd
 - We run the daemon as a service which means it starts automatically at boot or on demand with defined lifecycle. 
 - We do not need a PID file since systemd manages under cgroups

## Rust Compiler:
### Parsing:
 - Turns .rs code into Abstract Syntax Tree
 - AST is how the compiler or interpreter internally understands your code after parsing it
 - Logical structure not exact syntax
### Name Resolution and Type Checking:
 - Resolves type and what each variable refers to
 - Uses ownership, lifetimes, and borrow checking to ensure memory safety
#### Borrow Checking:
 - Ensures memory safety without a garbage collector by enforcing who can access data at what time.
 - `let s = String::from("hello");` Means that s owns the string and when ownership is removed e.g. `let b = a;`, the original cannot be used
 - `let r = &s;` allows you to immutable borrow a reference to a value. You can mutable borrow with `&mut T` but only once
 - A borrow cannot outlive the data it points to
### Multiple intermediate stages to move to Machine Code:
### Linking: 
 - One of the last compile time steps which connects Rust object code with the  C-compiled object code or a dynamic library
 - Final binary contains C and Rust machine code stitched together
 - Calling conventions must match e.g. extern "C"
## Rust Programming:
### Running:
 - `cargo build` to compile using rustc into a runnable binary
 - `cargo run` to build and run your program
 - When running package directly, main is the entry point otherwise, if library, uses src/lib.rs
 - main.rs is run once by a single process, the original process
 - Placing packages in dependency of cargo.toml automatically installs on build
 - Installed packages go into a Global Cache Directory in `~/.cargo/registry/` which is shared among all rust projects
### Variables:
 - By default, variables are immutable. You can change that with the `mut` keyword
 - Constants are immutable variables but are evaluated at compile time whereas varaibles are evaluated at runtime with global scope. Variables also create a memory binding for reference even though they are mutable.
 - We can safely shadow and the compiler will use the second instantiation as the proper variable and creates a new binding
 - A stack allocated primitive variables which are stored directly on stack. Heap is different since it uses heap allocation. In that case, Rust automatically frees that pointer
### Fork:
 - Child process returns 0 from the fork wheras parent gets the actual pid of the child
### Match:
 - Pattern matching function