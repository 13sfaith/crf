use std::io::prelude::*;
use structopt::StructOpt;

/// Create a react component either class or function (default class)
#[derive(StructOpt)]
struct Cli {
	/// Name of component
	name: String,
	/// Type of component (-t c or -t f)
	#[structopt(default_value = "c", short = "t", long = "type")]
	comp_type: String,
}


fn main() -> std::io::Result<()>{
	let args = Cli::from_args();

	check_args(&args.name, &args.comp_type);
	let payload = get_payload(&args.name, args.comp_type);

	let path_str = &(args.name + ".js");

	if std::path::Path::new(path_str).exists()
	{
		panic!("File already exists!");
	}

	let path = std::path::Path::new(path_str);
	let mut file  = std::fs::File::create(path)?;
	file.write_all(payload.as_bytes())?;

	Ok(())
}

fn check_args(name: &String, comp_type: &String)
{

	if !(name.chars().next().unwrap().is_ascii_uppercase())
	{
		panic!("Component needs to start with uppercase letter!");
	}

	if comp_type != "f" && comp_type != "c"
	{
		panic!("Invalid component type please only use (f or c)");
	}
}

fn get_payload(name: &String, comp_type: String) -> String
{
	let payload;

	if comp_type == "c"
	{
		payload = get_payload_class(name);
	} 
	else 
	{
		payload = get_payload_func(name);
	}

	payload
}

fn get_payload_func(name: &String) -> String
{
	let mut payload = "import React from 'react'\n\nfunction ".to_owned();
	payload.push_str(name);
	payload.push_str("(props) \n{\n\n}\n\nexport default ");
	payload.push_str(name);
	payload.push_str("\n");

	payload
}

fn get_payload_class(name: &String) -> String
{
	let mut payload = "import React from 'react'\n\nclass ".to_owned();
	payload.push_str(name);
	payload.push_str(" extends React.component\n{\n\n}\n\nexport default ");
	payload.push_str(name);
	payload.push_str("\n");

	payload
}

