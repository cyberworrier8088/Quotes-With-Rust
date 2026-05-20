// This is the ai quotes module for getting quotes from ai service
// It will get quotes from ai service and send it to webhook url

use reqwest::Client; // importing reqwest crate for http requests
use serde_json::{Value, json}; // importing serde_json crate for json parsing
use std::fs; // importing fs crate for file operations
use std::io::{self, Write}; // importing io crate for input/output operations

use crate::time; // importing time module

const CONFIG_FILE: &str = "Quotes.env"; // constant variable for config file name, env means environment variable file

struct AppConfig { // struct for config file
    api_key: String, // api key for ai service
    webhook_url: String, // webhook url for sending quote
    user_name: String, // user name
    user_hobby: String, // user hobby
}

pub fn setup_config() -> Result<(), Box<dyn std::error::Error>> { // setting up config file
    if fs::metadata(CONFIG_FILE).is_ok() { // if config file exists, ok means no error
        println!("Using saved setup from {}.", CONFIG_FILE);
        return Ok(()); // return ok means no error.
    }

    println!("No setup file found.");
    println!("I will make one file only: {}", CONFIG_FILE);


    let api_key = ask("Enter your API key: ")?; // asking for api key to user, None means no old value
    let webhook_url = ask("Enter your webhook url: ")?; // asking for webhook url to user, None means no old value
    let user_name = ask("Enter your name: ")?; // asking for user name to user, None means no old value
    let user_hobby = ask("Enter your hobby: ")?; // asking for user hobby to user, None means no old value

    let config_text = format!( // formating user input to config file.
        "API_KEY={}\nWEBHOOK_URL={}\nUSER_NAME={}\nUSER_HOBBY={}\n", // \n is for new line, that why each variable is on new line
        api_key, webhook_url, user_name, user_hobby
    );

    fs::write(CONFIG_FILE, config_text)?; // that writing or adding into config file, ? means error handling

    println!("Saved setup in {}.", CONFIG_FILE);
    println!("You can edit this file manually if you want.");

    Ok(())
}

fn ask( // ask function
    label: &str, // label for user input. argument
) -> Result<String, Box<dyn std::error::Error>> { // return string or error, this is a function ask

    // user input takking
    let mut input = String::new(); // creating string variable
    print!("{}: ", label);
    io::stdout().flush()?; // flushing stdout, this for print statement to show immediately
    io::stdin().read_line(&mut input)?; // reading user input

    let input = input.trim(); // trimming user input

    if input.is_empty() { // checking if user input is empty
        return Err(format!("{} cant be empty.", label).into());
    }

    Ok(input.to_string()) // returning user input
}



fn read_config() -> Result<AppConfig, Box<dyn std::error::Error>> { // read config function
    let text = fs::read_to_string(CONFIG_FILE)?; // reading config file

    let mut api_key = String::new();
    let mut webhook_url = String::new();
    let mut user_name = String::new();
    let mut user_hobby = String::new();

    for line in text.lines() { // iterating through each line, iterating means going through each line
        let Some((key, value)) = line.split_once('=') else { // splitting line into key and value like hashmap
            continue;
        };

        match key { // matching key with value
            "API_KEY" => api_key = value.trim().to_string(),
            "WEBHOOK_URL" => webhook_url = value.trim().to_string(),
            "USER_NAME" => user_name = value.trim().to_string(),
            "USER_HOBBY" => user_hobby = value.trim().to_string(),
            _ => {}
        }
    }

    if api_key.is_empty() || webhook_url.is_empty() || user_name.is_empty() || user_hobby.is_empty()// checking if any of the values are empty
    {
        fs::remove_file(CONFIG_FILE)?; // removing config file if any of the values are empty
        return Err(format!(// returning error
            "{} is missing something. deleted it and run again. try again. pls",
            CONFIG_FILE
        )
        .into()); // converting error to boxxxy
    }

    Ok(AppConfig { // returning config
        api_key,
        webhook_url,
        user_name,
        user_hobby,
    })
}



pub async fn send_saved_webhook() -> Result<(), Box<dyn std::error::Error>> { // send saved webhook function
    let config = read_config()?;

    // Create one HTTP client
    let client = Client::new(); // creating http client using reqwest. :)

    // Send webhook using same client
    send_webhook(&client, &config).await?; // sending to webhook

    Ok(())
}

async fn get_ai_response( 
    client: &Client, // http client
    config: &AppConfig, // config
) -> Result<String, Box<dyn std::error::Error>> {

    println!("Asking AI for quote...");

    let response = client // sending post request to ai
        .post("https://ai.hackclub.com/proxy/v1/chat/completions") // this is the endpoint for the ai api. using hackclub proxy. this is only using hack club api.
        .bearer_auth(config.api_key.trim()) // adding api key to request
        .json(&json!({
            "model": "x-ai/grok-4.3", // using Elon Musk's great  grok-4.3 model. :)
            "messages": [ // messages array, passing user prompt
                {
                    "role": "user", // setting role to user for prompt
                    "content": format!( // prompt content
                        "Write one short quote for {} about not giving up with {}. Make it sound like a normal person wrote it. Simple words. A little rough is ok. No emoji. No hashtags. No author name. Don't sound like AI.", // this is simple prompt yu update if you want to change the prompt or other type of quote or other type of application
                        config.user_name,
                        config.user_hobby
                    )
                }
            ]
        }))
        .send()// sending post request to ai
        .await?; // waiting for response

    // Check API response status
    if !response.status().is_success() { // checking if response is successful, ! means not successful, so if response is not successful, return error, status() returns status code
        return Err(format!(
            "API request failed: {}",
            response.status() // returning error with status code
        ).into());
    }

    let body: Value = response.json().await?; // parsing response body

    let answer = body["choices"][0]["message"]["content"]// getting quote from response
        .as_str() // converting to string
        .unwrap_or("No response") // if no response, return "No response"
        .to_string(); // converting to string

// as_str and to_string deffrents are that as_str returns a string slice and to_string returns a string

    Ok(answer) // returning quote
}

async fn send_webhook( // sending quote to webhook
    client: &Client, // http client
    config: &AppConfig, // app config
) -> Result<(), Box<dyn std::error::Error>> { // returning result

    // Get AI quote first
    let quote = get_ai_response(client, config).await?;

    println!("Sending to webhook...");

    let response = client // that same client we used for ai
        .post(&config.webhook_url) // posting to webhook url
        .json(&json!({// json body
            "content": format!( // formatting content and senting to webhook
                "\n\nHello! Today is {}\n\n{}",
                time::get_day(),
                quote
            )
        }))
        .send()// sending post request to webhook
        .await?;// waiting for response

    // Print webhook status
    println!("Webhook Status: {}", response.status());

    // Check webhook success
    if !response.status().is_success() { // checking if response is successful
        return Err(format!(
            "Webhook failed: {}", // returning error with status code
            response.status()
        ).into());
    }

    Ok(())
}

