// unicoqude - easy-to check is your terminal
// emulator has unicode support
//
// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

mod unicoqude {
    pub fn check() -> bool {
        #[cfg(target_os = "windows")]
        return true;

        #[cfg(target_family = "unix")]
        match std::env::var("TERM").unwrap().as_str() {
            "xterm-256color" | "alacritty" => return true,
            _ => {}
        }

        false
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ok() {
        print!("{} ", "Your terminal emulator has");

        match crate::unicoqude::check() {
            true => {
                print!("{} ", "support");
            },
            false => {
                print!("{} ", "not support");
            }
        }

        println!("{}", "unicode");
    }
}
