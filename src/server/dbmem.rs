#[derive(Debug)]
struct DbMem {
    pub db_name: String,
    pub tables: Vec<String>,
}

struct All_Databases{
    pub databases: Mutex<Vec<DbMem>>
}

pub struct AllTablesSingelton;

static INSTANCE: OnceLock<All_Databases> = OnceLock::new();

impl AllTablesSingelton {
    pub fn instance() -> &'static Mutex<DbMem> {
        INSTANCE.get_or_init(|| All_Databases{
            databases: Mutex::new(Vec::new()),
        })
    }
}