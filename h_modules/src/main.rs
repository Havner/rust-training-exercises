//mod utils {
    mod filesystem {
        pub fn find_file(_file: &str) {}
        pub fn mk_dir(_path: &str) {}
        pub fn read_file(_file: &str) {}
        pub fn write_file(_file: &str) {}
    }

    fn new_log(addr: &str) {
        filesystem::mk_dir(addr);
        filesystem::write_file(addr);
    }

    fn check_system() {
        filesystem::read_file("/some/file/");
        println!("System consistent!");
    }

    fn check_server(s: &mut Server) {
        filesystem::read_file("/another/file/");
        println!("Server consistent");
        s.checked = true;
    }

    fn turn_debug() {
        println!("Debug mode turned ON!");
    }
//}

//mod server {
    struct Database {
        addr: String,
        checked: bool,
    }

    impl Database {
        fn connect(addr: &str) -> Database {
            new_log(&format!("{}.log", addr));
            filesystem::find_file(addr);
            println!("DB opened: {}", addr);
            Database { addr: addr.to_string(), checked: false }
        }
        fn close(&self) {
            println!("DB closed: {}", self.addr);
        }
        fn check_db(&mut self) {
            self.checked = true;
            println!("DB consistent!");
        }
        fn query_db(&self, query: &str) {
            println!("Querying DB: {}", query);
        }
    }

    struct Server {
        d: Database,
        checked: bool,
    }

    impl Server {
        fn start(d: Database) -> Server {
            println!("Starting server with DB: {}", d.addr);
            check_system();
            if !d.checked {
                panic!("database not safe!");
            }
            Server { d, checked: false, }
        }
        fn stop(&self) {
            println!("Stopping server: {}", self.d.addr);
            self.d.close();
        }
        fn do_smth(&self) {
            if !self.checked {
                panic!("server not safe");
            }
            self.d.query_db("GET service list");
        }
    }
//}

// RULES:
// 1. move everything above to utils.rs, server.rs
// 2. "mod utils" and "mod server" in lib.rs have to be private (no "pub" keyword)
// 3. NO changes to main.rs except uncommenting one use line
// 4. NO changes to implementations: functions signature, functions body
// 5. NO changes to code structure
// 6. create only new "use SOMETHING" lines and add "pub" where required

//use modules::*;

fn main() {
    turn_debug();

    let mut d = Database::connect("127.0.0.1");
    d.check_db();
    let mut s = Server::start(d);
    check_server(&mut s);
    s.do_smth();
    s.stop();
}
