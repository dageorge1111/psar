# psar
## Daemon:
### First Fork:
 - We need to fork a parent process to have it run in the background.
 - When the parent exits, the child is no longer a session leader
 - Session leaders can reacquire a controlling terminal
 - Forking removes session leader status
### SID:
 - A session is a collection of related processes usually associated with a
 user terminal
 - The session leader is the process that creates the session and its PID is SID
 - We need to detach from the controlling terminal 
 - setSID detach from the old terminal
### Second Fork:
 - After we detach from the old terminal we want to remove the session leader
 status so that this process cannot create a new terminal
### Handle Logging:
 - 
### Systemd
 - We run the daemon as a service which means it starts automatically at boot or
 on demand with defined lifecycle. 