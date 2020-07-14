//Module responsible for creating the models of Logs
//Uses uuid library to generate unique uuids
//Level is the types of data that can be used for sql data
//Log is the model for logs
use uuid::Uuid;
use chrono::{DateTime,Utc};

//Level Enum
//Debug is for debug logs
//Info is for general logs
//Error is for Error logs
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Level{
    Debug,
    Info,
    Error
}

//Log Struct
//log_id is a general UUID that should be unique to the Log, prudent to check against database to ensure uniqueness
//level is the Log type
//Status Code is a message about the status about the current request
//Header should be the type of the log
//Message should be data specific to the log, probably should include parameters
//start_time is the time given to start the request, if not supplied it will be provided by post_gres
//complete_time is the time the request, if not supplied, it just defaults to none
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
//This functions creates a new Log
//The only required thing is a level, should probbly have one of status,header, or message,
//id is provided by the code
//Returns a log if input is valid
//Returns Option::<Log>::None if it is invalid
pub fn new_log(level: Level,
                status_code: Option<String>,
                header: Option<String>,
                message: Option<String>,
                start_time: Option<String>) -> Option<Log>{
      
    //If start_time is invalid move on
    //There is seperate processing if there is a start time
    if  start_time == Option::<String>::None {
        
        //Checking if there is data to add
        if status_code != Option::<String>::None ||
            header != Option::<String>::None ||
            message != Option::<String>::None {
        
            //Get uuid and generate Log struct
            let uuid : String = Uuid::new_v4().to_string();
            
            let log= Log {
                log_id: uuid,
                level:level,
                status_code:status_code,
                header: header,
                message: message,
                start_time: Option::<String>::None,
                complete_time: None
            };

            //Return result with Log
            return Some(log);

        }  else {
            //Data is invalid so return none
            return Option::<Log>::None;
        }
    } else {
        //if time is invalid, return nothing
        if (&(start_time.clone().unwrap())).parse::<DateTime<Utc>>().is_err() {
            return Option::<Log>::None;
        } else { 
            //Checking if status, header, or message exist
            if status_code != Option::<String>::None ||
                header != Option::<String>::None ||
                message != Option::<String>::None {
                
                //Get uuid and generate Log struct
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

                //Return Log
                return Some(log);

            //Return Nothing
            } else {
                return Option::<Log>::None;
            }
        }
    }   
}


//Testing if new_log_test is giving proper output
#[test]
fn new_log_test() {

    //Making sure bad_time returns None
    assert_eq!(
        new_log(
            Level::Info, 
            Some("200".to_string()), 
            Some("Header".to_string()), 
            Some("Message".to_string()), 
            Some("fake_time".to_string())),
       Option::<Log>::None);

    //Checking that no_time returns none
    assert_ne!(
        new_log(
            Level::Info, 
            Some("200".to_string()), 
            Some("Header".to_string()), 
            Some("Message".to_string()), 
            Option::<String>::None),
       Option::<Log>::None);

       //Checking that correct time is not none
       assert_ne!(
            new_log(
                Level::Info, 
                Some("200".to_string()), 
                Some("Header".to_string()), 
                Some("Message".to_string()), 
                Some("2014-11-28T21:00:09+09:00".to_string())),
       Option::<Log>::None);
    
    //Checking that empty log is none
   assert_eq!(
       new_log(
            Level::Info,
            Option::<String>::None,
            Option::<String>::None,
            Option::<String>::None,
            Option::<String>::None),
        Option::<Log>::None);
}

//Testing my understanding of the chrono library
#[test]
fn test_time() {
   assert!("".parse::<DateTime<Utc>>().is_err());
   assert!( &(Some("example_time".to_string()).unwrap()).parse::<DateTime<Utc>>().is_err());
}