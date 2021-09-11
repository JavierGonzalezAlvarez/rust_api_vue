//para deserializar nuestros "config Structs"
use serde::Deserialize;

//del modulo del toml lo importamos aqui
use config::ConfigError;

//configuracion para el server
//colocamos el uso del deserializador
#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32
}

//configracion para la app
//podemos incluir la configuracion del server dentro de la app, como una jeraquía
#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    //conexion con postgres
    pub pg: deadpool_postgres::Config
}

//creamos una fncion que carga la configuracion del fichero .env a traves
// de Environment::new()
impl Config {
    // la funcion tiene un resultado: Self => si es ok, ConfigError => si no es ok
    pub fn from_env() -> Result<Self, ConfigError> {        
        let mut cfg = config::Config::new();        
        cfg.merge(config::Environment::new())?;        
        cfg.try_into()
    }
}