

#[derive(Debug)]
pub struct TableMem {
    pub tables: Vec<String>,
}

pub struct TableMemSingelton;

static INSTANCE: OnceLock<Mutex<TableMem>> = OnceLock::new();

impl TableMemSingelton {
    pub fn instance() -> &'static Mutex<TableMem> {
        INSTANCE.get_or_init(|| Mutex::new(TableMem { tables: Vec::new() }))
    }
}