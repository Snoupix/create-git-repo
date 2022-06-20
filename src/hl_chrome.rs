use std::{
    thread::sleep,
    time::Duration,
};

use headless_chrome::{
    Browser,
    LaunchOptionsBuilder,
    protocol::dom::Node,
};

use rpassword;

use crate::format_input;

pub fn goto_github() {
    let browser = Browser::new(
        LaunchOptionsBuilder::default()
            .headless(false)
            .build()
            .unwrap()
        )
        .expect("Failed to launch Chrome process");

    let tab = browser.wait_for_initial_tab().expect("Failed to init a Chrome tab");

    tab.navigate_to("https://github.com/login").expect("Failed to connect to github");

    let login_btn = tab.wait_for_element("#login_field").expect("Cannot get name input");
    login_btn.click().expect("Cannot focus the name input");

    let mut login_name = String::new();
    //let mut login_password = String::new();

    println!("Enter the login name/mail:");

    std::io::stdin().read_line(&mut login_name).unwrap();

    tab.type_str(&format_input(login_name)).expect("Failed to type login name");

    println!("Enter the login password:");

    let login_password = rpassword::read_password().unwrap();
    //std::io::stdin().read_line(&mut login_password).unwrap();

    let login_p_btn = tab.wait_for_element("#password").expect("Cannot get password input");
    login_p_btn.click().expect("Cannot focus the password input");

    tab
        .type_str(&format_input(login_password))
        .expect("Failed to type login name")
        .press_key("Enter")
        .expect("Cannot press enter");

    let confirmation_node: Node = tab
        .wait_for_element(".lh-default")
        .expect("Cannot get password input")
        .get_description()
        .expect("Failed to get password description input");

    sleep(Duration::from_secs(5));

    println!("{:?}\n", confirmation_node);
    println!("You have 60s to accept the request on your github application with the node_value");

    sleep(Duration::from_secs(30));

    tab.navigate_to("https://github.com/new").expect("Failed to go to https://github.com/new");

    let repo_name_input = tab.wait_for_element("#repository_name").expect("Cannot get repo name input");
    repo_name_input.click().expect("Cannot focus the repo name input");

    tab.type_str(&std::env::args().nth(1).unwrap()[..]).expect("Failed to type repo name");

    let private_checkbox = tab.wait_for_element("#repository_visibility_private").expect("Cannot get private_checkbox input");
    private_checkbox.click().expect("Cannot focus the private_checkbox input");

    sleep(Duration::from_secs(2));

    let submit_btn = tab.wait_for_element(".btn-primary").expect("Cannot get private_checkbox input");
    submit_btn.click().expect("Cannot focus the private_checkbox input");

    tab.press_key("Enter").expect("Cannot press enter");

    sleep(Duration::from_secs(4));

    // get the url

    loop {};
}
