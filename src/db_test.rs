 
#[cfg(test)] 
mod tests { 
    use crate::{execute_command, generate_command};
    use crate::generate_input_type;
    use crate::Commands;
    use crate::InputType;
    use crate::Res;
    use crate::db::{Database, Row, DbString, Status};

    #[test]
    fn test_insert_row(){
        let mut db = Database::new(); 
        let row = Row::new(
            32, 
            DbString::new("hate".to_string(), 32),  
            DbString::new("love@gmail.com".to_string(), 
            255 ) 
        );
        let res = db.insert_row( row );
        assert!(matches!(res, Status::Success));
    }
    #[test]
    fn test_insert_row_fail(){
        let mut db = Database::new(); 
        let mut long_string = "hatehatehatehatehatehatehatehatehatehatehatehate".to_string(); 
        let row = Row::new(
            32, 
            DbString::new(long_string, 32),  
            DbString::new("love@gmail.com".to_string(), 
            255 ) 
        );
        let res = db.insert_row( row );
        assert!(matches!(res, Status::Fail));
        assert_eq!(db.get_table_size(), 0);
    }
    #[test]
    fn test_hello(){
        let hello = "hello";
        assert_eq!("hello", hello);
    }
    
    #[test]
    fn test_generate_exit_command(){
        let input = ".exit".to_string();
        let result = generate_command(&input);
        assert!(matches!(result, Commands::CommandExit));
    }
    #[test]
    fn test_generate_error_command(){
        let input = ".efxit".to_string();
        let result = generate_command(&input);
        assert!(matches!(result, Commands::CommandError));
    }
    #[test]
    fn test_generate_input_type_command(){
        let input = ".efxit".to_string();
        let result = generate_input_type(&input);
        assert!(matches!(result, InputType::Command));
    }
    #[test]
    fn test_generate_input_type_statement(){
        let input = "efxit".to_string();
        let result = generate_input_type(&input);
        assert!(matches!(result, InputType::Statement));
    }

    #[test]
    fn test_execute_command_exit(){
        let input = ".exit".to_string();
        let result = execute_command(&input);
        assert!(matches!(result, Res::Exit)); 
    }

    #[test]
    fn test_execute_command_error(){
        let input = ".efxit".to_string();
        let result = execute_command(&input);
        assert!(matches!(result, Res::Fail)); 
    }



}