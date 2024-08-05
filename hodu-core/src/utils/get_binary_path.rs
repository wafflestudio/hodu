use tokio::process::Command;

pub async fn get_binary_path(binary: &str) -> String {
    let which = Command::new("which")
        .arg(binary)
        .output()
        .await
        .expect("failed to execute process");

    if !which.status.success() {
        panic!("failed to execute which");
    }

    let output = std::fs::canonicalize(String::from_utf8(which.stdout).unwrap().trim().to_string())
        .expect("failed");

    output.to_str().unwrap().to_string()
}
