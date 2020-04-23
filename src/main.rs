// TODO convert to REST API

mod command;
mod sandbox;

use command::Language;
use sandbox::Sandbox;

const TEST_C_CODE: &'static str = r#"
#include <stdio.h>
int main() {
    for (int i = 0; i < 10; i++) {
        printf("%d\n", i);
    }
    return 0;
}
"#;

fn main() {
    let output = Sandbox::new(TEST_C_CODE, Language::C)
        .expect("Failed to construct sandbox")
        .output()
        .expect("Failed to execute");

    println!("STDOUT:\n{}", output.execute_output.stdout);
}
