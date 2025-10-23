# RPX - Reverse ProXy

This is of project for system and implemented reverse proxy than nginx
This is of idea create Master, it create Workers for params and its Workers 
sharing request between Servers added

```mermaid
sequenceDiagram
    actor C as Server Protected
    participant M as Master
    participant W as Worker
    
rect rgba(100,100,100,.3)
    Note over C,W: Create Conection   
    M->>M: "Init Master Process on Thread."
    M->>W: Create Process
    W->>C: Create Socket to Client
end

rect rgba(100,100,100,.3)
    Note over C,W: Request HTTP  
    M->>W: Request
    W->>C: Send request to socket
    C->>M: Request
end
```

Development
-----------
- Cargo

```bash
# make project
$ cargo run --release
$ binlocal -d ../rpx

# run project
$ rpx
```

Commits
-----------
For commits add structured for easy correction and detect issues

```bash
[ADD] Added method of function for correct operation App
[IMP] or [FEAD] Implementation of new part of App

[BUG] Detection and correction of Bugs in code
[FIX] Detection and correction of fixes and future issues
[HOTFIX] Correction issue IMPORTANT!
```

**That was all, thank!** 
- **Authors: Andres Cuello**
