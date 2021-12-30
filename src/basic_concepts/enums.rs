#[derive(Debug)]
enum ApiVersion {
    V1,
    V2,
}

#[derive(Debug)]
struct ApiInfo {
    version: ApiVersion,
    address: String,
}

impl ApiInfo {
    fn name() {
        let info = ApiInfo {
            version: ApiVersion::V1,
            address: String::from("http://localhost:5000/api"),
        };
        println!("Version: {:?}", &info);
    }
}

pub fn call() {
    ApiInfo::name();
}