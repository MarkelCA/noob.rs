Stack allocations and lookups are extremely fast.

This is because inside each stack frame, all the variable's offsets of the stack are stored instead of the full memory address. For example, for variables a and b, if a is at offset 0, and it's an integer of size 4 bytes, then b might be at offset 4, and so on.

Also allocations just involves moving the pointer n bytes to the top and returning that pointer. This is because the stack is allocated in a continuous manner.

Deallocating data simply means to move the stack pointer back without resetting the memory to any particular address, leaving them as "thrash" or "stale" data.

So this is why the stack memory it's so fast, because it simply involves pointer arithmetics.
