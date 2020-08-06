use crate::mlua;
use crate::rand;
use crate::regex;

use crate::basic;

use mlua::prelude::{ * };
use rand::{ random };
use regex::{ Captures, Regex };

use std::fs;
use std::collections::{ HashMap };
use std::process::{ Command, id };

use basic::{ clml };

pub struct CLML {
	env: HashMap<String, String>,
	pub lua_env: Lua,
	bash_env: HashMap<String, String>,
}

impl CLML {
	pub fn new() -> Self {
		let lua = Lua::new();
		{
			let globals = lua.globals();
			globals.set("__", {
				let t = lua.create_table().expect("");
				t.set("output", "").expect("");
				t
			}).expect("");
			lua.load(r#"function print(v) __.output = __.output..v end"#).exec().expect("");
		}
		CLML {
			env: HashMap::new(),
			lua_env: lua,
			bash_env: HashMap::new(),
		}
	}
	pub fn env(&mut self, k: &str, v: &str) -> &mut Self { self.env.insert(String::from(k), String::from(v)); self }
	pub fn bash_env(&mut self, k: &str, v: &str) -> &mut Self { self.bash_env.insert(String::from(k), String::from(v)); self }
	pub fn parse(&self, target: &str) -> Result<String, String> {
		let mut to_return = String::from(target);
		
		// Handle {} replacements.
		{
			let regex = Regex::new(r#"(?i)\{([a-z\.\-\d]+)\}"#).unwrap();
			to_return = String::from(regex.replace_all(&to_return, |c: &Captures| {
				let k = String::from(c.get(1).unwrap().as_str());
				if self.env.contains_key(&k) {
					self.env.get(&k).unwrap().clone()
				} else {
					String::new()
				}
			}));
		}

		// Handle lua`` evaluations.
		{
			let regex = Regex::new(r#"(?is)lua`(.*?)`"#).unwrap();
			to_return = String::from(regex.replace_all(&to_return, |c: &Captures| {
				{
					let try_lua = self.lua_env.load(c.get(1).unwrap().as_str()).exec();
					if try_lua.is_err() { return String::from("") }
				}
				let __ = self.lua_env
					.globals()
					.get::<&str, LuaTable>("__")
					.expect("Failed to get internal Lua value.");
				let to_return = String::from(
					__.get::<&str, LuaString>("output")
						.expect("Failed to get internal Lua value.")
						.to_str()
						.expect("Failed to convert `LuaString` to `&str`."));
				__.set("output", "").expect("Failed to set internal Lua value.");
				to_return
			}));
		}

		// Handle bash`` evaluations.
		{
			let mut i = random::<u32>();
			let bash_env = {
				let mut to_return = String::new();
				for (k,v) in self.bash_env.iter() {
					to_return = format!("{}{}",
						to_return,
						format!("export {k}={v}\n",
							k = k,
							v = v,));
				}
				to_return
			};
			let regex = Regex::new(r#"(?is)bash`(.*?)`"#).unwrap();
			to_return = String::from(regex.replace_all(&to_return, |c: &Captures| {
				let try_write = fs::write(
					&format!(
						"/tmp/clml{id}-{i}.sh",
						id = id(),
						i = i),
					&format!("{}{}",
						&bash_env,
						c.get(1).unwrap().as_str()));
				if try_write.is_err() { return String::new() }
				let try_bash = Command::new("bash")
					.arg(&format!("/tmp/clml{}-{}.sh", id(), i))
					.output();
				if try_bash.is_err() { return String::new() }
				let mut to_return = String::new();
				for byte in try_bash.unwrap().stdout { to_return.push(byte as char); }
				let _ = fs::remove_file(&format!("/tmp/clml{}-{}.sh", id(), i));
				i += 1;
				to_return
			}));
		}

		to_return = clml(&to_return);
		Ok(to_return)
	}
}

#[cfg(test)]
mod tests {
	use super::CLML;
	#[test]
	fn replacements() {
		let mut clml = CLML::new();
		clml.env("foo", "Hello world!");
		let try_parse = clml.parse("{foo}");
		assert!(try_parse.is_ok());
		assert_eq!(try_parse.unwrap(), "Hello world!");
	}
	#[test]
	fn lua_simple() {
		let clml = CLML::new();
		let try_parse = clml.parse(r#"lua`print("Hello world!")`"#);
		assert!(try_parse.is_ok());
		assert_eq!(try_parse.unwrap(), "Hello world!");
	}
	#[test]
	fn lua_complex() {
		let clml = CLML::new();
		{
			let lua = &clml.lua_env;
			let globals = lua.globals();
			assert!(globals.set("myValue", {
				let t = {
					let try_t = lua.create_table();
					assert_eq!(try_t.is_ok(), true);
					try_t.unwrap()
				};
				assert!(t.set(1, "Hello world").is_ok());
				assert!(t.set(2, "Hello universe").is_ok());
				t
			}).is_ok());
		}
		let try_parse = clml.parse(r#"lua`
		print(""..myValue[1]..", "..myValue[2]..".")
		`"#);
		assert!(try_parse.is_ok());
		assert_eq!(try_parse.unwrap(), "Hello world, Hello universe.");
	}
	#[test]
	fn bash_simple() {
		let clml = CLML::new();
		let try_parse = clml.parse(r#"bash`printf "Hello world!"`"#);
		assert!(try_parse.is_ok());
		assert_eq!(try_parse.unwrap(), "Hello world!");
	}
	#[test]
	fn bash_complex() {
		let mut clml = CLML::new();
		clml.bash_env("myValue", r#"( "Hello world" "Hello universe" )"#);
		let try_parse = clml.parse(r#"bash`printf "${myValue[0]}, ${myValue[1]}."`"#);
		assert!(try_parse.is_ok());
		assert_eq!(try_parse.unwrap(), "Hello world, Hello universe.");
	}
}