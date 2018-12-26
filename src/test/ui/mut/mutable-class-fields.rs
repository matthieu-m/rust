// revisions: ast mir
//[mir]compile-flags: -Z borrowck=mir

struct Cat {
  meows : usize,
  how_hungry : isize,
}

fn cat(in_x : usize, in_y : isize) -> Cat {
    Cat {
        meows: in_x,
        how_hungry: in_y
    }
}

fn main() {
  let nyan : Cat = cat(52, 99);
  nyan.how_hungry = 0; //[ast]~ ERROR cannot assign
  //[mir]~^ ERROR cannot assign
}
