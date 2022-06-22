use std::{
    thread::sleep,
    time::Duration,
    sync::Arc,
};

use headless_chrome::{
    Browser,
    browser::tab::Tab,
    LaunchOptionsBuilder,
    protocol::dom::Node,
};

use rpassword;

use crate::format_input;

pub struct ChromeTab {
    browser: Browser,
    tab: Arc<Tab>,
}

impl ChromeTab {
    pub fn build() -> Self {
        let browser = Self::init_browser();
        let tab = Self::init_tab(&browser);

        Self {browser, tab}
    }

    fn init_browser() -> Browser {
        Browser::new(
            LaunchOptionsBuilder::default()
                .headless(true)
                .build()
                .unwrap()
            )
            .expect("Failed to launch Chrome process")
    }

    fn init_tab(browser: &Browser) -> Arc<Tab> {
        browser.wait_for_initial_tab().expect("Failed to init a Chrome tab")
    }

    pub fn git_login(self) -> Self {
        self.tab.navigate_to("https://www.github.com/login").expect("Failed to connect to github");

        let login_btn = self.tab.wait_for_element("#login_field").expect("Cannot get name input");
        login_btn.click().expect("Cannot focus the name input");
    
        let mut login_name = String::new();
    
        println!("Enter the login name/mail:");
    
        std::io::stdin().read_line(&mut login_name).unwrap();
    
        self.tab.type_str(&format_input(login_name)).expect("Failed to type login name");
    
        println!("Enter the login password:");
    
        let login_password = rpassword::read_password().unwrap();
    
        let login_p_btn = self.tab.wait_for_element("#password").expect("Cannot get password input");
        login_p_btn.click().expect("Cannot focus the password input");
    
        self.tab
            .type_str(&format_input(login_password))
            .expect("Failed to type login name")
            .press_key("Enter")
            .expect("Cannot press enter");
    
        let parent_node: Node = self.tab
            .wait_for_element(".lh-default")
            .expect("Cannot get password input")
            .get_description()
            .expect("Failed to get password description input");
        
        let children_nodes: Vec<Node> = parent_node.children.unwrap();
        
        let confirmation_node: &Node = children_nodes.get(0).unwrap();
    
        println!("You have 30s to accept the request on your github application with the code {}.", confirmation_node.node_value);
    
        sleep(Duration::from_secs(30));

        self
    }

    pub fn create_repo(self) -> String {
        self.tab.navigate_to("https://www.github.com/new").expect("Failed to go to https://www.github.com/new");

        let repo_name_input = self.tab.wait_for_element("#repository_name").expect("Cannot get repo name input");
        repo_name_input.click().expect("Cannot focus the repo name input");

        self.tab.type_str(&std::env::args().nth(1).unwrap()[..]).expect("Failed to type repo name");

        let private_checkbox = self.tab.wait_for_element("#repository_visibility_private").expect("Cannot get private_checkbox input");
        private_checkbox.click().expect("Cannot focus the private_checkbox input");

        sleep(Duration::from_secs(2));

        let submit_btn = self.tab.wait_for_element(".btn-primary").expect("Cannot get submit btn");
        submit_btn.click().expect("Cannot focus the submit btn");

        self.tab.press_key("Enter").expect("Cannot press enter");

        sleep(Duration::from_secs(4));

        self.tab
            .get_target_info()
            .expect("Cannot get tab info")
            .url
    }
}
