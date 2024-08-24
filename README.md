# Modular Assembler

## Features
- Compile time statements
    Ops:
    - `+` add
    - `-` subtract
    - `*` multiply
    - `/` divide
    - `&` bitwise and
    - `|` bitwise or
    - `^` bitwise xor
    - `<<` bitshift left
    - `>>` bitshift right
    ```
    lim R2, ((5 + 5) << 2)
    ```
  NOTE: you can manipulate label addresses
- Sub-labels (simmilar to any other assembler)
    ```
    main:
      lim R1, 5
    .loop:
      limb R2, .loop

    func:
      add R1, .loop
    .loop:
      limb R1, main.loop
    ```
- flexible instruction set and registers
  #### just look at config.rs inside src folder and see for yourself
