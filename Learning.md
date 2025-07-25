# Learning
## Sar:
 - Collected using a daemon called sadc
 - Defaults to collecting one snapshot of usage every 10 minutes via cron. Schedule is in `/etc/cron.d/sysstat`
 - Connect `proc` files with the sar snapshots
### IO:
 - rchar is the number of bytes that the process has read from storage or a page cache aka the number of bytes requested from the process
 - read_bytes is the number of bytes the kernel actually fetches from disk
 - similar logic for wchar and write_bytes
 - cancelled_write_bytes are the number of bytes written by user requested but never hit the disk
 - rchar(from cache) does not contribute to iowait
 - iowait is the percentage of time the CPU spent idle waiting for an I/O operation to complete
 - Programmed IO is the simplest possible I/O method where microprocessors have a explicit sequence of instructions and a single input and output
 - Interrupt driven IO is when the CPU instructs an IO device to start the task and generate an interrupt when done. This however leads to many unneccessary interrupts after something like a single character transmitted
 - DMA controller/chip allows a device to transfer data from memory directly without the CPU manually copying byte by byte by setting up a few registers for a source address, a destination address, a size of transfer, and a direction(read or write). DMA controller takes over and works without any further CPU instructions and raises interrupt when done
 - Usually reads cause iowait since it is blocking until the data arrives, writes are usually buffered.
 - However, if writes are happening after explicitely flushing the disk, the kernel's writeback thread is overwhelmed and write buffers fill up aka explicitely flushed writing
 - Buffered writing will rarely cause iowait unless the dirty page threshold is exceeded where kernel blocks until it is cleared

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
### Dependency Injection:
 - Recommended not to use stdout and stderr since it is global and not flexible
 - Instead dependency inject implementations of a writer or reader so that we can select a different writer for logging tests or other things
 - Does not lose performance!
### Writing:
 - I/O traits require a mutable reference to write since writing changes the internal state of the writer
 - `.unwrap()` will try to extract a value from a function and if it cannot then it panics and crashes
 
### OOP in Rust:
#### Traits:
 - These are similar to interfaces - they are methods that are implemented seperately for different types
 - When called on **trait objects**, it is invoked based on runtime information which is called dynamic dispatch
 - Generics are any type that implements the requisite trait
#### Monomorphization:
 - The compiler generates a seperate version of the function for each concrete type used in a generic
 - This allows for static dispatch because the type is known at runtime
#### vtable:
 - Virtual Method Table is a runtime structure that rust creates per concrete type that implements a trait and has pointers to that type's method implementation
 - Each type-trait combination has a vtable which has pointers to all methods in the trait which are implemented by that type
 - Allows us to group objects by their shared traits rather than their concrete types exactly
 - For dynamic dispatch, the compiler still guarantees that the type implements the trait that it is being used for but does not pull the exact implementation until runtime
#### Classes:
 - Implementation inheritance is one of the biggest regrets in creating Java. Rust does not have standard classes and therefore forbids classic inheritance of classes
### Error Handling:
 - Rust breaks down errors into two types, recoverable errors and unrecoverable errors
#### Unrecoverable Errors:
 - Unrecoverable errors usually want the program to stop entirely and use panic!
 - Two ways of inducing an unrecoverable, either by doing some bad behavior like accessing an array past the end or directly calling `panic!` macro
 - By default, panic will print a message, unwind, clean up the stack, and quit
 - Unwinding means that Rust walks back up the stack and cleans up data from each function it encounters. However, you can choose to immedietely abort as well without cleaning up. Add `panic = 'abort'` to the appropriate [profile] sections in your Cargo.toml file
 - Recoverable errors are usually when the program wants to tell the user something went wrong and retry the operation
 - Rust does not have exceptions and instead uses the type `Result<T,E>`
### Lifetimes:
 - The compiler checks the lifetime length of each variable that references and if a variable has a lifetime longer than the variable it is referencing
### Vectors:
 - Re-sizeable arrays. Their size is not known at compile time. It tracks a pointer to data, length, and capacity of the vector
 - Length is an integer stored on the stack 
### String:
 - A vector of bytes `Vec<u8>` which is heap allocated, growable and not null terminated
 - `&str` is a slice that always points to a valid UTF-8 sequence
 - `len` has the number of bytes used to encode the the string in UTF-8 which can be more than 1 byte per char instead for chars use `s.chars().count()`
 - `.chars().rev()` creates a reversed character iterator
 - `.take(x)` yields up to x characters
### HashMap:
 - implemented using quadratic probing and SIMD lookup - single instruction multiple data.
### Threads:
 - No guarantee on order in which parts of multithreaded code will run
 - Race conditions are when threads access data in an inconsistent order
 - Deadlock is when multiple threads are waiting on each other to finish
 - Rust uses a 1:1 model of thread implementation in which they use one OS thread per programming language thread
 - `thread::spawn` will create a new thread and pass a closure containing the code we want to run
 - When the main thread completes, all spawned threads are shut down which you can avoid with `join` handle which blocks completion of current thread until the thread represented by handle is completed
 - We use `move` keyword to pass ownership of a value from one thread to another. Rust wants to borrow a reference from another thread but does not know how long that reference will exist so it does not know if the reference is going to be valid for the entire existence of the thread
#### Message Passing:
 - “Do not communicate by sharing memory; instead, share memory by communicating.”
 - Rust standard library includes a channel implementation which is a programming concept allowing data to be sent from one thread to another
 - Includes a transmitter which is called to send data and a reciever which is checked for arriving messages
 - `mpsc::channel` where mpsc stands for multiple producer, single consumer which returns a tuple (transmitter, receiver)
 - Once we send a value, we transfer ownership of that value to the send function
 - We can treat rx as an iterator which will close once the channel is closed
 - We can call `.clone` on a transmitter
### Closures:
 - anonymous functions you can save in a variable