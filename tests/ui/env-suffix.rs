use paste2::paste;

paste! {
    fn [<env!("VAR"suffix)>]() {}
}

fn main() {}
