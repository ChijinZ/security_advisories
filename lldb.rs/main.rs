extern crate lldb;

use lldb::*;
use std::fs;

fn main() {

    SBDebugger::initialize();

    let debugger = SBDebugger::create(false);
    debugger.set_async(false);
    println!("{:?}", debugger);

    if let Some(target) = debugger.create_target_simple("testcase/app") {
        println!("{:?}", target);
    

    if let Ok(entries) = fs::read_dir("testcase/coredump/") {
        for entry in entries {
            if let Ok(entry) = entry {
//                println!("{:?}", entry.file_name());
//                println!("{:?}", entry.path());
               
                let path = entry.path();
//                println!("{:?}", path);

                let str_path = path.to_str().unwrap();
                println!("{:?}", str_path);


                let process = target.loadcore(str_path);
                println!("{:?}", process);

                for thread in process.threads(){
                    println!("{:?}", thread);
                    for frame in thread.frames(){
                        println!("{:?}", frame);
//                        println!("{:?}", frame.module());
                        println!("{:?}", frame.display_function_name());
//                        println!("{:?}", frame.pc_address());
                        // println!("{:?}", frame);
                    }
                }
            }
        }
    }

        

       
        // let _ = process.continue_execution();
        // println!("{:?}", process);
        // let _ = process.kill();
        // println!("{:?}", process);

        // let launchinfo = SBLaunchInfo::new();
        // launchinfo.set_launch_flags(LaunchFlags::STOP_AT_ENTRY);
        // match target.launch(launchinfo) {
        //     Ok(process) => {
        //         println!("{:?}", process);
        //         let _ = process.continue_execution();
        //         println!("{:?}", process);
        //     }
        //     Err(e) => println!("Uhoh: {:?}", e),
        // }
    }
    SBDebugger::terminate();
}


