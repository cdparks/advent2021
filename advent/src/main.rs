use nix::sys::wait::waitpid;
use nix::unistd::{fork, ForkResult, Pid};
use std::process::{exit, Command};

fn main() {
    let forked = unsafe { fork() };
    let code = match forked.expect("failed to fork process") {
        ForkResult::Parent { child } => wait_on(child),
        ForkResult::Child => run_tests(),
    };
    exit(code.unwrap_or(1));
}

fn wait_on(pid: Pid) -> Option<i32> {
    waitpid(Some(pid), None).ok().map(|_| 0)
}

fn run_tests() -> Option<i32> {
    let mut child = Command::new("cargo").arg("test").spawn().ok()?;
    let status = child.wait().ok()?;
    status.code()
}
