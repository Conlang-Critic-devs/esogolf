# Stalch
A stack-based esolang

## Interpreter

The interpreter goes through each command, separated by whitespace, and runs the
command. The command can be a value or an internal command. If it's a value, it
will be interpreted and pushed to the stack. Otherwise the command will run.

## Variables

Stalch is dynamically typed, although a value still has an internal type:

When the interpreter encounters a literal, it is parsed and pushed to the stack.

 * String

    Anything inside quotation marks ("") will be treated as a string.
    There is no escaping yet, everything between quotation marks will
    be considered part of the string.
 * Number

    All numbers are 64-bit floating point values.
 * Block

    Basically a lambda (anonymous function). They are created with the by
    putting code between the `{` and `}` operators. These are also pushed to
    the stack. It can be run using the `apply` command (alias: `()`).
 * Null

    This type is rarely used. It usually represents an error.
 * Variable name

    Used to assign a value to a name. When called they either push what their
    name is assigned to, or push a variable handle. This handle can be used to
    assign a value to a name with the `assign` command (alias: `:=`)
    ##### Example
        foo 47 :=

## Basic commands

Almost every command works with what's currently at the top of the stack.
The syntax is postfix and may therefore be unfamiliar.

### Mathematical operators

The basic mathematical operators are `add`, `sub`, `mul`, `div`, `rem` which
each have the following respective symbolic aliases: `+`, `-`, `*`, `/`, `%`.

#### Example

`4 4 +`. This will end up with an `8` in the stack.
Let's go through this step by step:

First the interpreter will read the first `4` and then add that to the stack.
The stack now looks like this:

`4`

The next `4` will then also be added to the top of the stack:

`4 4`

Lastly, the add command is run, this pops the two numbers off the stack,
adds them together and pushes the result. The resulting stack thus looks like:

`8`

**NOTE**: If you try and run a command without enough values on the stack.
The program will fail and exit. E.g. if you tried to write `4 + 4`, the interpreter
would try to add the two numbers on the stack together, but since it has only
gotten the first `4` at that point, it will fail.
Remember to put the operation last.

### IO

Stalch has three commands to pass stuff through STDIN and STDOUT:

#### `prnt` (`_`)

This prints pops the stack and prints the value to STDOUT along with a newline.

#### `wrte` (`>`)

Does the same as `prnt`, but doesn't add a newline.

#### `read` (`<`)

This reads a line from `STDIN` and pushes it to the stack as a string
(NOTE: The string will be right trimmed).

### Stack manipulation

#### `swap`, `exch`, `$`

This command switches the two top values in the stack. E.g imagine a stack as follows:

`A B C D`

Running `swap` would make it look as this:

`A B D C`

#### `drop`

Pops the stack and simply throws away the value.

#### `dup`, `;`

Pushes a copy of the current top value to the stack.

#### `grab`, `£`

Pops a value, `n`, from the stack, this value is assumed to be an integer.
It then goes `n` elements back in the stack and moves that value to the top.

E.g. doing `1 grab` would be equivalent to doing a `swap`.

#### `dupgrab`, `:`

Does the same as `grab` except instead of moving the value, it makes a copy of
and puts that on top. The original value will stay in its place.
