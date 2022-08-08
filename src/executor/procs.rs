use procfs::process::Process;
use procfs::{process};
use cli_table::{print_stdout, Cell, Style, Table};
use nix;
use nix::sys::signal;
use nix::unistd::Pid;
use nix::sys::signal::Signal;

pub struct WineProc(Vec<Process>);
impl WineProc {
    pub fn new() -> Self {
        let procs = process::all_processes().unwrap();
        WineProc {
            0: procs
                .filter(|p| match p {
                    Ok(proc) => match proc.exe() {
                        Ok(pp) => {
                            pp.ends_with("wine-preloader") || pp.ends_with("wine64-preloader")
                        }
                        Err(_) => false,
                    },
                    Err(_) => false,
                })
                .filter(|p| match p {
                    Ok(_) => true,
                    Err(_) => false,
                })
                .map(|p| p.unwrap())
                .collect::<Vec<Process>>(),
        }
    }
    pub fn kill_all(&self){
        if self.0.is_empty() {
            println!("There are not Wine process found!");
            return;
        }
        for p in &self.0{
            signal::kill(Pid::from_raw(p.pid), Some(Signal::SIGKILL)).unwrap();
        }
    }
    pub fn kill_by_name(&self, name: &str){
        if self.0.is_empty() {
            println!("There are not Wine process found!");
            return;
        }
        for p in &self.0{
            match p.stat() {
                Ok(s) => {
                    if s.comm == name{
                        signal::kill(Pid::from_raw(p.pid), Some(Signal::SIGKILL)).unwrap();
                        return;
                    }
                }
                Err(_) => {}
            }
        }
        println!("Wine process: \"{}\" not found!", name);
    }

    pub fn print_table(&self){
        let mut table = vec![];
        for p in &self.0{
            let stat = p.stat().unwrap();
            table.push(
                vec![stat.comm.cell(),
                     stat.pid.cell(),
                     stat.ppid.cell(),
                     p.uid().unwrap().cell(),
                     stat.state.cell()])
        }
        let table = table
            .table()
            .title(vec!["comm".cell().bold(true),
                        "pid".cell().bold(true),
                        "ppid".cell().bold(true),
                        "uid".cell().bold(true),
                        "status".cell().bold(true)
            ])
            .bold(true);
        print_stdout(table).unwrap();
    }
}
