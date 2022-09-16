mod arguments;
use arguments::get_args;

use computer_config::Config;

use computer::*;

mod display;
use display::display;

fn main()
{
    // test feature
    let args = get_args();
    let config = Config::from_args(args);
    let width = config.width();
    let height = config.height();

    let computer = Computer::new(config);

    display(computer, width, height);
}
