#[allow(unused)]
#[napi_derive::napi]
fn run(args: Vec<String>, bin_name: Option<String>) {
    create_node_api::run(args, bin_name);
}
