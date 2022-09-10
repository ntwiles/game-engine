use pollster;
use fooheppy::run;

fn main() {
    pollster::block_on(run());
}
