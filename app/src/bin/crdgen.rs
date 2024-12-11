use cloudsecrets::{CloudSecret, CloudSecretProvider};
use kube::CustomResourceExt;
use serde_yaml;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("No CRD specified. Usage: crdgen <CloudSecret|CloudSecretProvider>");
    }

    let crd_type = &args[1];
    let yaml = match crd_type.to_lowercase().as_str() {
        "cloudsecret" => {
            let crd = CloudSecret::crd();
            serde_yaml::to_string(&crd).expect("Failed to serialize CloudSecret CRD to YAML")
        }
        "cloudsecretprovider" => {
            let crd = CloudSecretProvider::crd();
            serde_yaml::to_string(&crd)
                .expect("Failed to serialize CloudSecretProvider CRD to YAML")
        }
        _ => panic!(
            "Invalid CRD type: {}. Use 'CloudSecret' or 'CloudSecretProvider'.",
            crd_type
        ),
    };

    println!("{}", yaml);
}