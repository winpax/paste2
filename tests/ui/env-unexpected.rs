use paste2::paste;

paste! {
    fn [<env!("VAR" "VAR")>]() {}
}

fn main() {}
