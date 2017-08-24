#[macro_use]
extern crate serde_derive;

extern crate docopt;

use docopt::Docopt;

const USAGE: &'static str = "
JSON XPather

Usage:
  jx [-x] [-i JSONFILE] XPATH_EXPRESSION
  jx (-h | --help)
  jx --version


Arguments:
  XPATH_EXPRESSION    The XPATH String [default: /]

Options:
  -i JSONFILE         JSON File
  -x                  Output XML
  -h --help           Show this screen.
  --version           Show version.

Details:
  jx applies an xpath expression to the JSON document. 

";

#[derive(Debug, Deserialize)]
struct Args {
    flag_x: bool,
    arg_xpath_expression: String,
    arg_json_filepath: Option<String>,
}


fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.deserialize())
                            .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
}
