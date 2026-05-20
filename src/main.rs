//###################################################################################################################################################
// Introduction:
// 1.this is the main function of the program
// 2. this project using tokio for async and reqwest for http requests and serde_json for json parsing and chrono for  real time
// 3. this project is a simple quote app in TUI with discord webhook using hack Club AI APIs using for getting quotes. This is a funny app.
// 4. this project not production ready. level project. this only a simple learning project.
// 5. this project is not perfect. it is a learning project. so dont expect too much. but somthing fun and learning.
// End of Introduction
// you can edit or modify this code as you want.
// enjoy my Code! :)
//###################################################################################################################################################


mod time; // adding time module to this
mod quots; // adding ai quotes module to this


#[tokio::main]// adding tokio main macro to this project

async fn main() { // main function to run other functions, and async means it can wait for things to finish
    println!("Starting quote app...");

    if let Err(e) = quots::setup_config() { // setup config, if error happened put it in e or and calling quots::setup_config
        eprintln!("Setup error: {}", e); // eprintln is like println but for errors. this is a macro in rust. using for printing errors, e is error containtind variable
        return;
    }

    println!("Sending today quote...");

    if let Err(e) = quots::send_saved_webhook().await { // this same as above but for sending webhook
        eprintln!("Webhook error: {}", e);// this also same as above but for sending webhook
        return;
    }

    println!("Done. app still open and checking day.");
    time::time_checker(); // this is for checking time using time module.
}
