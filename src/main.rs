#[macro_use]
extern crate serde_derive;

extern crate docopt;

use docopt::Docopt;

const USAGE: &'static str = "
JSON XPather

Usage:
  jx [-i <json_filepath>] [-x] <xpath_expression>
  jx (-h | --help)
  jx --version

Options:
  -h --help           Show this screen.
  --version           Show version.
  -i <json_filepath>  JSON File
  -x                  Output XML
  <xpath_expression>  The XPATH String [default: /]

Details:
  jx applies an xpath expression to the JSON document. 

";

#[derive(Debug, Deserialize)]
struct Args {
    flag_xml_out: bool,
    arg_xpath_expression: String,
    arg_json_filepath: Option<String>,
}


fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.deserialize())
                            .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
}
