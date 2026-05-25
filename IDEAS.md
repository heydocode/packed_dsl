# Ideas

This document presents ideas that randomly come into my head

## ZST DSL Contents Composition

Once a DSL is established at compile-time, the user can use the derived
Instruction enum either to deserialize DSL contents, either to serialize
DSL contents. For serialization, a struct will be used - one field for a 
mutable reference an array of instructions and other fields - ZSTs for
keeping track of instructions, and extremely ergonomically permit to create
a sorts of execution pipeline of high-level instructions.