use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

use tatam::solve::SmtBridge;

fn main() {
    let mut smt_bridge = SmtBridge::new("z3", vec!["-in"], Some("log.smt")).unwrap();
    smt_bridge.declare_const("x", "Int").unwrap();
    smt_bridge.assert("(= x 10)").unwrap();

    // let result = smt_bridge.check_sat().unwrap();
    let result = smt_bridge.check_sat_using("(then simplify smt)").unwrap();
    println!("result: {:?}", result);

    let x = smt_bridge.eval("x").unwrap();
    println!("x = {}", x);

    smt_bridge.exit().unwrap();

    // smt_bridge.write_line("(declare-fun x () Int)").unwrap();
    // smt_bridge.write_line("(assert (= x 10))").unwrap();
    // smt_bridge.write_line("(check-sat)").unwrap();
    // smt_bridge.flush().unwrap();

    // let result = smt_bridge.read_line().unwrap();
    // println!("result: {}", result);

    // smt_bridge.write_line("(exit)").unwrap();
    // smt_bridge.flush().unwrap();

    let status = smt_bridge.wait().unwrap();
    println!("{}", status);
}

fn old() {
    let mut command = Command::new("z3");
    command
        .arg("-in")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = command.spawn().unwrap();

    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let mut stdout_reader = BufReader::new(stdout);

    let file = std::fs::File::open("future.smt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap() + "\n";
        stdin.write_all(line.as_bytes()).unwrap();
    }

    // stdin.write_all(b"(declare-fun x () Int)\n").unwrap();
    // stdin.write_all(b"(assert (= x 10))\n").unwrap();
    // stdin.write_all(b"(check-sat)\n").unwrap();
    stdin
        .write_all(
            b"(check-sat-using (then (repeat (then propagate-values simplify solve-eqs)) smt))\n",
        )
        .unwrap();
    stdin.flush().unwrap();

    {
        let mut line = String::new();
        stdout_reader.read_line(&mut line).unwrap();
        println!("result: {}", line.trim());
    }

    stdin.write_all(b"(exit)\n").unwrap();
    stdin.flush().unwrap();

    {
        let mut line = String::new();
        stdout_reader.read_line(&mut line).unwrap();
        println!("result: {}", line.trim());
    }

    // stdin.write_all(b"(assert (= x 20))\n").unwrap();
    // stdin.write_all(b"(check-sat)\n").unwrap();
    // // stdin.write_all(b"(exit)\n").unwrap();
    // stdin.flush().unwrap();

    // line.clear();
    // stdout_reader.read_line(&mut line).unwrap();
    // println!("echo output: {}", line.trim());

    let mut stderr_reader = std::io::BufReader::new(stderr).lines();
    while let Some(line) = stderr_reader.next() {
        println!("stderr: {}", line.unwrap());
    }

    let exit_status = child.wait().unwrap();
    println!("exited with status: {}", exit_status);

    // println!("stdout: {}", String::from_utf8_lossy(&output.stdout));

    // println!("command exited with status: {}", output.status);
}
