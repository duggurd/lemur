use lemur::Mitm;

fn main() {
    let mut mitm = Mitm::from_args();

    mitm.run();
}
