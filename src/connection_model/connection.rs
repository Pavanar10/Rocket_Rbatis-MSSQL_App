
 
use rbatis::{RBatis};
use rbdc_mssql::driver::MssqlDriver;
use config::Config;

pub struct Connection{
    pub db:RBatis
}

impl Connection{
    fn new()->Connection{
        Self {
            db:RBatis::new(),
        }
    }

    fn init(&self) {
        let conf: ConnectionConfig = ConnectionConfig::new();

        let url = format!(
            "jdbc:sqlserver://{}:{};User={};Password={};Database={}",
            conf.server, conf.port, conf.user, conf.password, conf.database
        );
        println!("{:?}",url);
        self.db.init(MssqlDriver {},&url)
            .expect("failed to connect");
        
           println!("Successfully established");

    }

    pub fn create_and_init() -> Connection {
        let conn: Connection = Connection::new();
        conn.init();
        conn
    }
}


#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct ConnectionConfig {
    server: String,
    port: String,
    user: String,
    password: String,
    database: String,
}

impl ConnectionConfig {
    pub fn new() -> ConnectionConfig {
        let settings = match Config::builder()
            .add_source(config::File::with_name("Connection"))
            .build()
        {
            Ok(c) =>{ println!("successful");  c},
            Err(_) => {
                panic!("Connection properties file not found.");
            }
        };

        Self {
            server: settings.get("server").expect(Self::missing_property_msg().as_str()),
            port: settings.get("port").expect(Self::missing_property_msg().as_str()),
            user: settings.get("user").expect(Self::missing_property_msg().as_str()),
            password: settings.get("password").expect(Self::missing_property_msg().as_str()),
            database: settings.get("database").expect(Self::missing_property_msg().as_str()),
        }
    }

    fn missing_property_msg() -> String {
        String::from("Missing property in the Connection file")
    }
}