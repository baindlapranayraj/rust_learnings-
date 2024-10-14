mod config;
mod routes;

use config::printing_func::config_print;
use routes::health_print::print_health_route;
use routes::naruto_print::print_naruto;

fn main() {
    config_print();
    print_health_route();
    print_naruto()
}
