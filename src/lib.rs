use std::collections::{HashSet, HashMap};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CLIParser {

	/// **Positional arguments**.
	/// 
	/// These are the standard arguments without any special syntax. 
	/// 
	/// Example:
	/// ```bash
	/// ./my_program posit_argument_1 posit_argument_2
	/// ```
	pub posits: Vec<String>,

	/// **Flags**.
	/// 
	/// These arguments are prefixed with a singular dash line. They are unique, unordered and don't take any values.
	/// 
	/// Example:
	/// ```bash
	/// ./my_program -test_mode -verbose
	/// ```
	pub flags: HashSet<String>,

	/// **Key - value pairs**.
	/// 
	/// These arguments are prefixed with a double dash line. They need to be connected to their value with an equal sign.
	/// 
	/// Example
	/// ```bash
	/// ./my_program --debug_level=2 --id=5 --name="John Smith"
	/// ```
	pub pairs: HashMap<String, String>,
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CLIError {
	FlagWithSign(String),
	FlagMalformed(String),
	PairMissingSign(String),
	PairBadSign(String),
	PairMalformed(String),
	DashesMalformed(String)
}


// All of these are baseline error with no underlying cause. Simply bad CLI arguments.
impl std::error::Error for CLIError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            CLIError::FlagWithSign(_) => None,
            CLIError::FlagMalformed(_) => None,
            CLIError::PairMissingSign(_) => None,
            CLIError::PairBadSign(_) => None,
            CLIError::PairMalformed(_) => None,
            CLIError::DashesMalformed(_) => None,
        }
    }
}


impl std::fmt::Display for CLIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CLIError::FlagWithSign(ref arg) => write!(f, "Equal signs not allowed in flags: `{0}`\nProper syntax: `./my_program -flag`", arg),
            CLIError::FlagMalformed(ref arg) => write!(f, "Malformed flag: `{0}`\nProper syntax: `./my_program -flag`", arg),
            CLIError::PairMissingSign(ref arg) => write!(f, "Key-value pair arguments need an equal sign: `{0}`\nProper syntax: `./my_program --key=value`", arg),
            CLIError::PairBadSign(ref arg) => write!(f, "Improper use of equal sign in key-value pair: `{0}`\nProper syntax: `./my_program --key=value`", arg),
            CLIError::PairMalformed(ref arg) => write!(f, "Malformed key-value pair: `{0}`\nProper syntax: `./my_program --key=value`", arg),
            CLIError::DashesMalformed(ref arg) => write!(f, "Arguments cannot start with 3 or more dash lines: `{0}`", arg),
        }
    }
}

impl Default for CLIParser {
	fn default() -> Self {
		Self {
			posits: Vec::new(),
			flags: HashSet::new(),
			pairs: HashMap::new(),
		}
	}
}

impl CLIParser {
	
	/// Creates a new cli-parser object, with empty data structures. 
	pub fn new() -> Self {
		Self::default()
	}

	/// Parses the `std::env::args()` and collects them into data structures.
	/// 
	/// Will throw error if CLI arguments are considered malformed by this crate.
	/// 
	/// ```
	/// // Initialize parser
	/// let parser = cliparser::CLIParser::new().init().unwrap();
	/// 
	/// // Extract parsed data structures
	/// let posit_arguments = parser.posits.clone(); // Vector
	/// let flags = parser.flags.clone(); // HashSet
	/// let pairs = parser.pairs.clone(); // HashMap
	/// ```
	pub fn init(mut self) -> Result<Self, CLIError> {
		
		for argument in std::env::args() {

			// Positional
			if !argument.starts_with("-") {
				self.posits.push(argument);
				continue;
			}

			else if !argument.starts_with("--") {
				
				if argument.contains("=") {
					return Err(CLIError::FlagWithSign(argument));
				}

				if argument.len() < 2 {
					return Err(CLIError::FlagWithSign(argument));
				}

				self.flags.insert(argument[1..].to_string());
				continue;
			}
			
			else if !argument.starts_with("---") {
				if !argument.contains("=") {
					return Err(CLIError::PairMissingSign(argument));
				}

				if argument.len() < 5 {
					return Err(CLIError::PairMalformed(argument));
				}

				let equal_sign_pos: usize = argument.find('=').unwrap();
				if equal_sign_pos == 2 || equal_sign_pos == argument.len() - 1 {
					return Err(CLIError::PairBadSign(argument));
				}

				let kwarg: (&str, &str) = argument.split_once("=").unwrap();
				self.pairs.insert(kwarg.0[2..].to_string(), kwarg.1.to_string());
			}

			else {
				return Err(CLIError::DashesMalformed(argument));
			}
		}

		Ok(self)
	}

}

