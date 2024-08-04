use rand::Rng;
use std::path::PathBuf;
use std::process::Command;

pub fn execute_isolate(
    temp_dir: &PathBuf,
    command: &PathBuf,
    args: &[PathBuf],
) -> std::process::Output {
    let box_id = rand::thread_rng().gen_range(0..1000);
    let box_id_arg = format!("--box-id={}", box_id);

    Command::new("isolate")
        .arg(&box_id_arg)
        .arg("--init")
        .output()
        .expect("Failed to init box");

    let mut command_args = vec![
        format!("--dir={}", temp_dir.display()),
        box_id_arg.clone(),
        "--run".to_string(),
        "--".to_string(),
        command.display().to_string(),
    ];

    command_args.extend(args.iter().map(|arg| arg.display().to_string()));

    let output = Command::new("isolate")
        .args(&command_args)
        .output()
        .expect("Failed to execute");

    Command::new("isolate")
        .arg(&box_id_arg)
        .arg("--cleanup")
        .output()
        .expect("Failed to cleanup box");

    output
}
