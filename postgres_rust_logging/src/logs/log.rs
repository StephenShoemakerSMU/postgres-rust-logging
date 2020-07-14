//Module responsible for creating the models of Logs
//Uses uuid library to generate unique uuids
//
use uuid::Uuid;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Level{
    Debug,
    Info,
    Error
}


//Log Struct
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Log {
    log_id: String,
    level: Level,
    status_code: Option<String>,
    header: Option<String>,
    message: Option<String>,
    start_time: Option<String>,
    complete_time: Option<String>
}

pub fn new_log(level: Level,
                status_code: Option<String>,
                header: Option<String>,
                message: Option<String>,
                start_time: Option<String>) -> Option<Log>{
    
    
    
    if status_code != Option::<String>::None ||
        header != Option::<String>::None ||
        message != Option::<String>::None {
        
        let uuid : String = Uuid::new_v4().to_string();
        
        let log= Log {
            log_id: uuid,
            level:level,
            status_code:status_code,
            header: header,
            message: message,
            start_time: start_time,
            complete_time: None
        };

            return Some(log);
        } else {
            return Option::<Log>::None;
        }

    }

#[test]
fn new_log_test() {
    assert_ne!(
        new_log(
            Level::Info, 
            Some("200".to_string()), 
            Some("Header".to_string()), 
            Some("Message".to_string()), 
            Some("example_time".to_string())),
       Option::<Log>::None);
    
   assert_eq!(
       new_log(
            Level::Info,
            Option::<String>::None,
            Option::<String>::None,
            Option::<String>::None,
            Option::<String>::None),
        Option::<Log>::None);
}