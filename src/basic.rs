use crate::regex;

use crate::replacements;
use crate::emoji;

use regex::{ Captures, NoExpand, Regex };

use emoji::{ EMOJI };
use replacements::{ REPLACEMENTS };

pub fn clml(target: &str) -> String {
    let mut to_return = String::from(target);
    
    // Do simple replacements.
    for k in REPLACEMENTS.keys() {
        let regex = Regex::new(format!(r#"<{}>"#, k).as_str()).unwrap();
        to_return = regex.replace_all(to_return.clone().as_str(), NoExpand(REPLACEMENTS.get(k).unwrap())).to_string();
    }

    // Do 256 color replacements.
    {
        let regex = Regex::new(r#"<255(?:-bg)? (\d{1,3})>"#).unwrap();
        to_return = regex.replace_all(to_return.clone().as_str(), |capture: &Captures| {
            let mut to_return = String::from("\u{001b}[");
            if capture.get(0).unwrap().as_str().contains("-bg") {
                to_return = format!("{}48;5;", to_return);
            } else {
                to_return = format!("{}38;5;", to_return);
            }
            to_return = format!("{}{}m", to_return, capture.get(1).unwrap().as_str());
            to_return
        }).to_string();
    }

    // Do rgb color replacements.
    {
        let regex = Regex::new(r#"<rgb(?:-bg)? (\d{1,3}) (\d{1,3}) (\d{1,3})>"#).unwrap();
        to_return = regex.replace_all(to_return.clone().as_str(), |capture: &Captures| {
            let mut to_return = String::from("\u{001b}[");
            if capture.get(0).unwrap().as_str().contains("-bg") {
                to_return = format!("{}48;2;", to_return);
            } else {
                to_return = format!("{}38;2;", to_return);
            }
            to_return = format!("{__}{r};{g};{b}m",
                __ = to_return,
                r = capture.get(1).unwrap().as_str(),
                g = capture.get(2).unwrap().as_str(),
                b = capture.get(3).unwrap().as_str());
            to_return
        }).to_string();
    }

    // Do movement replacements.
    {
        {
            let regex = Regex::new(r#"<to (\d+) (\d+)>"#).unwrap();
            to_return = regex.replace_all(to_return.clone().as_str(), |capture: &Captures| {
                format!("\u{001b}[{};{}H",
                    capture.get(1).unwrap().as_str(),
                    capture.get(2).unwrap().as_str())
            }).to_string();
        }
        {
            let regex = Regex::new(r#"<up (\d+)>"#).unwrap();
            to_return = regex.replace_all(to_return.clone().as_str(), |capture: &Captures| {
                format!("\u{001b}[{}A", capture.get(1).unwrap().as_str())
            }).to_string();
        }
        {
            let regex = Regex::new(r#"<down (\d+)>"#).unwrap();
            to_return = regex.replace_all(to_return.clone().as_str(), |capture: &Captures| {
                format!("\u{001b}[{}B", capture.get(1).unwrap().as_str())
            }).to_string();
        }
        {
            let regex = Regex::new(r#"<forward (\d+)>"#).unwrap();
            to_return = regex.replace_all(to_return.clone().as_str(), |capture: &Captures| {
                format!("\u{001b}[{}C", capture.get(1).unwrap().as_str())
            }).to_string();
        }
        {
            let regex = Regex::new(r#"<backward (\d+)>"#).unwrap();
            to_return = regex.replace_all(to_return.clone().as_str(), |capture: &Captures| {
                format!("\u{001b}[{}D", capture.get(1).unwrap().as_str())
            }).to_string();
        }
    }

    // Do emoji replacements.
    {
        let regex = Regex::new(r#"<(:[a-zA-Z_\-0-9]+:)>"#).unwrap();
        to_return = regex.replace_all(to_return.clone().as_str(), |capture: &Captures| {
            if EMOJI.get(capture.get(1).unwrap().as_str()).is_some() {
                String::from(*EMOJI.get(capture.get(1).unwrap().as_str()).unwrap())
            } else {
                String::from(&capture[0])
            }
        }).to_string();
    }

    to_return
}

                                                                                    
#[cfg(test)]
mod tests {
	use super::clml;
    #[test]
    fn simple() {
        assert_eq!("\u{001b}[31mHi\u{001b}[0m", clml("<red>Hi<reset>"));
    }
    #[test]
    fn color_255() {
        assert_eq!("\u{001b}[38;5;10mHi\u{001b}[0m", clml("<255 10>Hi<reset>"));
    }
    #[test]
    fn color_rgb() {
        assert_eq!("\u{001b}[38;2;255;255;255mHi\u{001b}[0m", clml("<rgb 255 255 255>Hi<reset>"));
	}
	#[test]
	fn movement() {
		assert_eq!("\u{001b}[250CHi", clml("<forward 250>Hi"))
	}
    #[test]
    fn emoji() {
        assert_eq!("😄", clml("<:smile:>"));
    }
}
