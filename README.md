## tiny-compiler in Rust :)

#### TODO
- [X] Instruction datatype is implemented
- [ ] Restructure basic block, function, and program
- [ ] Finish parser for SSA generation

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
