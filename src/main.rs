extern crate helper;

fn main() {
    let args = helper::args_and_create_file::get_user_args();

    helper::args_and_create_file::create_file(&args).expect("Error while creating file");
}
