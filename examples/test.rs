extern crate immi;

fn main() {
    let layout = .draw();

    //let layout = layout.margin(1.0, 1.0, 1.0, 1.0);

    for layout in layout.equal_vertical_split(5) {

    }
}
