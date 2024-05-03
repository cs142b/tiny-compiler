## tiny-compiler in Rust :)

#### TODO
- [ ] Generate or emit SSA instructions in the parser<br>-> idea is to store all instructions in a vector of pairs (basic_block_id, instruction_enum), using the indices as the line numbers
- [ ] Implement how basic blocks, functions, and program work as a whole together.<br>-> add implementation like graphs here...
- [ ] Add tasks as we go...

#### Usage of the tiny Language
```js
main 
var x, var y, var z; 

function addition (a,b); 
var c, var d; { 
    c <- a; 
    d <- b;
    return c + d;
};

function subtraction (a,b);
var c, var d; {
    c <- b;
    d <- a;
    return  c - d;
};

{
    let y <- call inputNum();
    let z <- call inputNum();
    let x <- call subtraction(y,z);
    if x < 2 then
        let x <- 100;
    else
        while x < 200 do
            let x <- call addition(x, 1);
        od
    fi
    call outputNum(x);
}.
```
