# RTS (Real-Time Script)
A reactive, free-form, high-level scripting language that runs in interpreter mode and ~~converts scripts directly into machine code~~.
> Now RTS is ~2 times faster than python. Ideally, it should run on GASS, not Rust, which will give a ~20x performance increase, reduce memory leaks and instruction consumption. But if GASS uses DAS instead of GAS, then it is difficult to talk about the amount of output power, most likely it will be incredibly fast.

The language uses the Latin alphabet as a technical one and any other language as explanatory-accompanying, more understandable to the developer.
> This is associated with the widespread use of the Latin alphabet in exact sciences and global distribution. In one English word, there can be many meanings, and the generalizing meaning is clearly visible. However, this does not exclude porting the syntax to other world languages. The original form should be taken as an international standard.

It is expected to write programs of any complexity and completely free structures, running them in interpretation mode.

### Why an interpreter?
> RTS is a continuation of GASS <- DAS. This is a chain from assembler, compiled language and ending with interpreted language.

How interpretation and RTS work:

1. The code is executed in real time:
   1. The program is processed directly on the processor in real time and its operation depends on the complexity of the code and the strength of the machine;
   2. The code is more difficult to intercept during execution, which is more secure for the confidentiality of the program;
   3. Opting out of error handling. In place of their processing, the emphasis is on maximum fault tolerance. Mistakes are simply not possible.
   
2. The language is stored separately from the program itself and does not interfere with:
   1. There is no need to compile the program several times;
   2. The interpreter can be configured before running the program;
   3. Allows you to have a package manager for flexible real-time configuration;
   4. Allows you to run multiple programs simultaneously.
   
3. Writing free structures:
   1. Structures can represent any type of data or structure of code blocks;
   2. Structures can be processed with individual flags, asynchronously, and in a fault-tolerant manner;
   3. Structures have no restrictions, inherit from primitive structures and automatically work with memory;
   4. Structures interact reactively with each other.

The code is read from top to bottom line by line and taking into account the level of nesting.
> RTS considers indentation as creating a new block from lines of code. If such an attachment has been given a name, then a data structure of the type Class, Enum, List, Method and others will be formed.
