use tokio::process::Command;

pub async fn realpath(binary: &str) -> String {
    let which = Command::new("which")
        .arg(binary)
        .output()
        .await
        .expect("failed to execute process");

    if !which.status.success() {
        panic!("failed to execute which");
    }

    let output = Command::new("realpath")
        .arg(String::from_utf8(which.stdout).unwrap().trim().to_string())
        .output()
        .await
        .expect("failed to execute process");

    if !output.status.success() {
        panic!("failed to execute realpath");
    }

    String::from_utf8(output.stdout).unwrap().trim().to_string()
}
