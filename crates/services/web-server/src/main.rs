use lib_utils::b64::b64u_encode;

fn main() {
    let encoded_data = b64u_encode([0, 1, 2, 3]);
    println!("{}", encoded_data);
}
