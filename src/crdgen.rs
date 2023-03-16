use controller::spec::Zeppelin;
use kube::CustomResourceExt;

fn main() {
    print!("{}", serde_yaml::to_string(&Zeppelin::crd()).unwrap())
}
