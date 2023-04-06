
a: src/a.rs
	cat input/a | cargo run --bin a

b: src/b.rs
	cat input/b | cargo run --bin b

c: src/c.rs
	cat input/c | cargo run --bin c

d: src/d.rs
	cat input/d | cargo run --bin d

d_other: src/d_other.rs
	cat input/d | cargo run --bin d_other

d_test:
	cat input/d | cargo run --bin d > d_res
	cat input/d | cargo run --bin d_other > d_other_res
	   
	diff d_res d_other_res
    

e: src/e.rs
	cat input/e | cargo run --bin e

f: src/f.rs
	cat input/f | cargo run --bin f

g: src/g.rs
	cat input/g | cargo run --bin g
