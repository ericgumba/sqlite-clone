use std::usize;


pub struct DbString {
    val: String,
    max_length: usize
}

impl DbString {
    pub fn new(val: String, max_length: usize)->DbString{

        DbString { val, max_length}
    }
}
pub struct Row{
    id: u32,
    username: DbString,
    email: DbString,
}
pub struct Table{
    rows: Vec<Row>,
    // metadata: Hash!() {columns: u16, }, 
}

impl Row {
    pub fn new(id:u32, username:DbString, email:DbString)-> Row {
   
        Row{
            id, 
            username,
            email 
        }

    }
}

pub enum Status {
    Success,
    Fail
}
pub struct Database{
    table: Table,
}

impl Database {
    pub fn new()->Database{
        return Database{
            table: Table { rows: Vec::new(), }
        }
    }
    pub fn invalid_row(&mut self,row:&Row) -> bool { 
        row.email.max_length < row.email.val.len() || row.username.max_length < row.username.val.len()
    }
    
    pub fn insert_row(&mut self, row:Row ) -> Status {

        if self.invalid_row(&row){
            return Status::Fail
        }

        self.table.rows.push(row);
        Status::Success
    }
    pub fn print_table() -> Database{
        return Database{
            table: Table { rows: Vec::new(), }
        }
    }
    pub fn get_table_size(&mut self) -> usize{
        self.table.rows.len()
        
    }
}