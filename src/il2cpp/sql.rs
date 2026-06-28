use std::sync::{atomic::{self, AtomicBool, Ordering}, Mutex, RwLock};
use fnv::{FnvHashMap, FnvHashSet};
use sqlparser::ast;
use once_cell::sync::Lazy;
use crate::{
    core::{utils::{get_data_path, get_masterdb_path}, Hachimi, Interceptor},
    il2cpp::{ext::{StringExt, Il2CppStringExt}, hook::LibNative_Runtime::Sqlite3::{Connection, Query}, types::{Il2CppObject, Il2CppString}}
};

pub static RETRIEVED_RAW_KEY: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static AUTO_UNLOCK_NEXT_DB: AtomicBool = AtomicBool::new(false);
pub static META_DATA: Lazy<RwLock<MetaData>> = Lazy::new(|| RwLock::new(MetaData::default()));

static mut ORIG_SQLITE3_OPEN_V2: Option<extern "C" fn(*const i8, *mut *mut std::ffi::c_void, i32, *const i8) -> i32> = None;
static mut ORIG_SQLITE3_KEY: Option<extern "C" fn(*mut std::ffi::c_void, *const std::ffi::c_void, i32) -> i32> = None;

extern "C" fn sqlite3_open_v2_hook(filename: *const i8, pp_db: *mut *mut std::ffi::c_void, flags: i32, z_vfs: *const i8) -> i32 {
    let result = unsafe { ORIG_SQLITE3_OPEN_V2.unwrap()(filename, pp_db, flags, z_vfs) };
    if result == 0 && !pp_db.is_null() {
        if AUTO_UNLOCK_NEXT_DB.swap(false, Ordering::Relaxed) {
            let raw_key = RETRIEVED_RAW_KEY.lock().unwrap();
            if !raw_key.is_empty() {
                let db_ptr = unsafe { *pp_db };
                unsafe { ORIG_SQLITE3_KEY.unwrap()(db_ptr, raw_key.as_ptr() as *const std::ffi::c_void, raw_key.len() as i32) };
            }
        }
    }
    result
}

extern "C" fn sqlite3_key_hook(db: *mut std::ffi::c_void, p_key: *const std::ffi::c_void, n_key: i32) -> i32 {
    if !p_key.is_null() {
        let mut raw_guard = RETRIEVED_RAW_KEY.lock().unwrap();
        if raw_guard.is_empty() {
            let key_bytes = unsafe { std::slice::from_raw_parts(p_key as *const u8, n_key as usize) };
            *raw_guard = key_bytes.to_vec();
        }
    }
    unsafe { ORIG_SQLITE3_KEY.unwrap()(db, p_key, n_key) }
}

pub fn hook_sqlite(interceptor: &Interceptor, handle: usize) {
    #[cfg(target_os = "windows")]
    unsafe {
        use windows::Win32::System::LibraryLoader::GetProcAddress;
        use windows::core::PCSTR;
        let h_module = windows::Win32::Foundation::HMODULE(handle as _);
        let open_addr = GetProcAddress(h_module, PCSTR("sqlite3_open_v2\0".as_ptr()));
        let key_addr = GetProcAddress(h_module, PCSTR("sqlite3_key\0".as_ptr()));
        if let Some(addr) = open_addr {
            if let Ok(orig) = interceptor.hook(addr as usize, sqlite3_open_v2_hook as *const () as usize) {
                ORIG_SQLITE3_OPEN_V2 = Some(std::mem::transmute(orig));
            }
        }
        if let Some(addr) = key_addr {
            if let Ok(orig) = interceptor.hook(addr as usize, sqlite3_key_hook as *const () as usize) {
                ORIG_SQLITE3_KEY = Some(std::mem::transmute(orig));
            }
        }
    }
    #[cfg(target_os = "android")]
    unsafe {
        let handle_ptr = handle as *mut libc::c_void;
        let open_sym = b"sqlite3_open_v2\0".as_ptr() as *const libc::c_char;
        let key_sym = b"sqlite3_key\0".as_ptr() as *const libc::c_char;
        let open_addr = libc::dlsym(handle_ptr, open_sym);
        let key_addr = libc::dlsym(handle_ptr, key_sym);
        if !open_addr.is_null() {
            if let Ok(orig) = interceptor.hook(open_addr as usize, sqlite3_open_v2_hook as *const () as usize) {
                ORIG_SQLITE3_OPEN_V2 = Some(std::mem::transmute(orig));
                info!("Successfully hooked native sqlite3_open_v2 (Android)");
            }
        }
        if !key_addr.is_null() {
            if let Ok(orig) = interceptor.hook(key_addr as usize, sqlite3_key_hook as *const () as usize) {
                ORIG_SQLITE3_KEY = Some(std::mem::transmute(orig));
                info!("Successfully hooked native sqlite3_key (Android)");
            }
        }
    }
}

// public API
#[derive(Default)]
pub struct CharacterData {
    pub chara_ids: FnvHashSet<i32>,
    pub chara_names: FnvHashMap<i32, String>
}

impl CharacterData {
    pub fn load_from_db() -> Self {
        let mut chara_ids = FnvHashSet::default();
        let mut chara_names = FnvHashMap::default();

        let db_path = get_masterdb_path();
        let conn = Connection::new();

        if Connection::Open(conn, db_path.to_il2cpp_string(), std::ptr::null_mut(), std::ptr::null_mut(), 0) {
            let sql = "SELECT C.id, T.text FROM chara_data AS C JOIN text_data AS T ON C.id = T.\"index\" WHERE T.id = 6";
            let query = Connection::Query(conn, sql.to_il2cpp_string());

            if !query.is_null() {
                while Query::Step(query) {
                    let id = Query::GetInt(query, 0);
                    let name_ptr = Query::GetText(query, 1);

                    if let Some(name) = unsafe { name_ptr.as_ref() }.map(|s| s.as_utf16str().to_string()) {
                        chara_ids.insert(id);
                        chara_names.insert(id, name);
                    }
                }
                Query::Dispose(query);
            }
            Connection::CloseDB(conn);
        }

        CharacterData { chara_ids, chara_names }
    }

    pub fn exists(&self, id: i32) -> bool {
        self.chara_ids.contains(&id)
    }

    pub fn get_name(&self, id: i32) -> String {
        // check text_data_dict.json (category 170)
        if let Some(category_170) = Hachimi::instance().localized_data.load().text_data_dict.get(&170) {
            if let Some(name) = category_170.get(&id) {
                return name.clone();
            }
        }

        // fallback to default Japanese name from mdb
        if let Some(name) = self.chara_names.get(&id) {
            return name.clone();
        }

        // unknown character name
        "???".to_string()
    }
}

// untranslated skill info
#[derive(Default)]
pub struct SkillInfo {
    pub skill_names: FnvHashMap<i32, String>,
    pub skill_descs: FnvHashMap<i32, String>,
}

impl SkillInfo {
    pub fn load_from_db() -> Self {
        let mut skill_names = FnvHashMap::default();
        let mut skill_descs = FnvHashMap::default();

        let db_path = get_masterdb_path();
        let conn = Connection::new();

        if Connection::Open(conn, db_path.to_il2cpp_string(), std::ptr::null_mut(), std::ptr::null_mut(), 0) {
            // category 47 = names, 48 = descriptions
            let sql = "SELECT \"index\", text, id FROM text_data WHERE id IN (47, 48)";
            let query = Connection::Query(conn, sql.to_il2cpp_string());

            if !query.is_null() {
                while Query::Step(query) {
                    let index = Query::GetInt(query, 0);
                    let text_ptr = Query::GetText(query, 1);
                    let category = Query::GetInt(query, 2);

                    if let Some(text) = unsafe { text_ptr.as_ref() }.map(|s| s.as_utf16str().to_string()) {
                        match category {
                            47 => skill_names.insert(index, text),
                            48 => skill_descs.insert(index, text),
                            _ => None,
                        };
                    }
                }
                Query::Dispose(query);
            }
            Connection::CloseDB(conn);
        }

        SkillInfo { skill_names, skill_descs }
    }

    pub fn get_name(&self, id: i32) -> String {
        if let Some(name) = self.skill_names.get(&id) {
            return name.clone();
        }

        // unknown skill name
        "???".to_string()
    }

    pub fn get_desc(&self, id: i32) -> String {
        if let Some(desc) = self.skill_descs.get(&id) {
            return desc.clone();
        }

        // unknown skill desc
        "???".to_string()
    }
}

// All of this add column/param stuff could be simplified to two hash maps, but that's overkill.
pub trait SelectQueryState {
    /// Adds a column to the query.
    /// 
    /// Implementers are expected to only track the index of columns that they need.
    fn add_column(&mut self, idx: i32, name: &str);

    /// Adds a placeholder parameter to the query (WHERE param = ?).
    /// 
    /// Index starts at 1.
    fn add_param(&mut self, idx: i32, name: &str);

    /// Bind an int value to a placeholder.
    /// 
    /// Index starts at 1.
    fn bind_int(&mut self, idx: i32, value: i32);

    /// Gets the resulting string on the current row's column.
    fn get_text(&self, query: *mut Il2CppObject, idx: i32) -> Option<*mut Il2CppString>;

    /// Gets a tag describing the origin of the data.
    fn get_origin_tag(&self, _query: *mut Il2CppObject, _idx: i32) -> Option<String> { None }
}

#[derive(Default)]
struct Column {
    /// Index of the column in the SELECT statement.
    /// 
    /// Can be used to query the value later if needed.
    select_idx: Option<i32>,

    /// Index of the placeholder param for this column.
    /// 
    /// If this column's value is already binded as a param in the query, we won't need to query it later.
    param_idx: Option<i32>,

    /// The int value binded to this column as a parameter.
    int_value: Option<i32>
}

impl Column {
    fn is_select_idx(&self, idx: i32) -> bool {
        if let Some(i) = self.select_idx {
            idx == i
        }
        else {
            false
        }
    }

    fn is_param_idx(&self, idx: i32) -> bool {
        if let Some(i) = self.param_idx {
            idx == i
        }
        else {
            false
        }
    }

    fn try_bind_int(&mut self, idx: i32, value: i32) {
        if self.is_param_idx(idx) {
            self.int_value = Some(value);
        }
    }

    fn try_get_int(&self, query: *mut Il2CppObject) -> Option<i32> {
        if let Some(idx) = self.select_idx {
            Some(Query::GetInt(query, idx))
        }
        else {
            None
        }
    }

    fn value_or_try_get_int(&self, query: *mut Il2CppObject) -> Option<i32> {
        if let Some(value) = self.int_value {
            Some(value)
        }
        else if let Some(value) = self.try_get_int(query) {
            Some(value)
        }
        else {
            None
        }
    }
}

// text_data
#[derive(Default)]
pub struct TextDataQuery {
    // SELECT
    text: Column,

    // WHERE
    category: Column,
    index: Column
}
pub struct TextFormatting {
    pub line_len: i32,
    pub line_count: i32,
    pub font_size: i32,
    pub is_localized: bool
}

pub static TDQ_IS_SKILL_LEARNING_QUERY: AtomicBool = AtomicBool::new(false);
pub static IS_SYSTEM_TEXT_QUERY: AtomicBool = AtomicBool::new(false);

impl TextDataQuery {
    pub fn with_skill_learning_query(callback: impl FnOnce()) {
        TDQ_IS_SKILL_LEARNING_QUERY.store(true, atomic::Ordering::Relaxed);
        callback();
        TDQ_IS_SKILL_LEARNING_QUERY.store(false, atomic::Ordering::Relaxed);
    }

    fn is_skill_learning_query() -> bool {
        TDQ_IS_SKILL_LEARNING_QUERY.load(atomic::Ordering::Relaxed)
    }

    pub fn get_skill_name(index: i32) -> Option<*mut Il2CppString> {
        // Return None if skill name translation is disabled
        if Hachimi::instance().config.load().disable_skill_name_translation {
            return None;
        }

        let localized_data = Hachimi::instance().localized_data.load();
        let text_opt = localized_data
            .text_data_dict
            .get(&47)
            .map(|c| c.get(&index))
            .unwrap_or_default();

        if let Some(text) = text_opt {
            // append $(bf) if it's a skill learning query to let best fit do its job
            if Self::is_skill_learning_query() {
                 return Some(format!("{}", text).to_il2cpp_string());
            }
            Some(text.to_il2cpp_string())
        }
        else {
            None
        }
    }

    pub fn get_skill_desc(index: i32) -> Option<*mut Il2CppString> {
        let localized_data = Hachimi::instance().localized_data.load();
        let text_opt = localized_data
            .text_data_dict
            .get(&48)
            .map(|c| c.get(&index))
            .unwrap_or_default();

        if let Some(text) = text_opt {
            if Self::is_skill_learning_query() {
                return Some(format!("{}", text).to_il2cpp_string());
            }
            Some(text.to_il2cpp_string())
        }
        else {
            None
        }
    }
}

impl SelectQueryState for TextDataQuery {
    fn add_column(&mut self, idx: i32, name: &str) {
        if name == "text" {
            self.text.select_idx = Some(idx)
        }
    }

    fn add_param(&mut self, idx: i32, name: &str) {
        match name {
            "category" => self.category.param_idx = Some(idx),
            "index" => self.index.param_idx = Some(idx),
            _ => ()
        }
    }

    fn bind_int(&mut self, idx: i32, value: i32) {
        self.category.try_bind_int(idx, value);
        self.index.try_bind_int(idx, value);
    }

    fn get_text(&self, _query: *mut Il2CppObject, idx: i32) -> Option<*mut Il2CppString> {
        if !self.text.is_select_idx(idx) {
            return None;
        }

        if let Some(category) = self.category.int_value {
            if let Some(index) = self.index.int_value {
                // specialized handlers
                match category {
                    47 => return Self::get_skill_name(index),
                    48 => return Self::get_skill_desc(index),
                    _ => ()
                };

                
                return Hachimi::instance().localized_data.load()
                    .text_data_dict
                    .get(&category)
                    .map(|c| c.get(&index).map(|s| s.to_il2cpp_string()))
                    .unwrap_or_default()
            }
        }

        None
    }

    fn get_origin_tag(&self, _query: *mut Il2CppObject, idx: i32) -> Option<String> {
        if !self.text.is_select_idx(idx) {
            return None;
        }

        let category = self.category.int_value?;
        let index = self.index.int_value?;
        Some(format!("T:{}:{}", category, index))
    }
}

// character_system_text
#[derive(Default)]
pub struct CharacterSystemTextQuery {
    // SELECT
    text: Column,

    // WHERE
    character_id: Column,

    // may appear in both
    voice_id: Column
}

impl SelectQueryState for CharacterSystemTextQuery {
    fn add_column(&mut self, idx: i32, name: &str) {
        match name {
            "text" => self.text.select_idx = Some(idx),
            "voice_id" => self.voice_id.select_idx = Some(idx),
            _ => ()
        }
    }

    fn add_param(&mut self, idx: i32, name: &str) {
        match name {
            "character_id" => self.character_id.param_idx = Some(idx),
            "voice_id" => self.voice_id.param_idx = Some(idx),
            _ => ()
        }
    }

    fn bind_int(&mut self, idx: i32, value: i32) {
        self.character_id.try_bind_int(idx, value);
        self.voice_id.try_bind_int(idx, value);
    }

    fn get_text(&self, query: *mut Il2CppObject, idx: i32) -> Option<*mut Il2CppString> {
        if !self.text.is_select_idx(idx) {
            return None;
        }

        if let Some(character_id) = self.character_id.int_value {
            if let Some(voice_id) = self.voice_id.value_or_try_get_int(query) {
                return Hachimi::instance().localized_data.load()
                    .character_system_text_dict
                    .get(&character_id)
                    .map(|c| c.get(&voice_id).map(|s| s.to_il2cpp_string()))
                    .unwrap_or_default()
            }
        }

        None
    }

    fn get_origin_tag(&self, query: *mut Il2CppObject, idx: i32) -> Option<String> {
        if !self.text.is_select_idx(idx) {
            return None;
        }

        let character_id = self.character_id.int_value?;
        let voice_id = self.voice_id.value_or_try_get_int(query)?;
        Some(format!("C:{}:{}", character_id, voice_id))
    }
}

// race_jikkyo_comment
#[derive(Default)]
pub struct RaceJikkyoCommentQuery {
    // SELECT
    id: Column,
    message: Column
}

impl SelectQueryState for RaceJikkyoCommentQuery {
    fn add_column(&mut self, idx: i32, name: &str) {
        match name {
            "id" => self.id.select_idx = Some(idx),
            "message" => self.message.select_idx = Some(idx),
            _ => ()
        }
    }

    fn add_param(&mut self, _idx: i32, _name: &str) {}

    fn bind_int(&mut self, _idx: i32, _value: i32) {}

    fn get_text(&self, query: *mut Il2CppObject, idx: i32) -> Option<*mut Il2CppString> {
        if !self.message.is_select_idx(idx) {
            return None;
        }

        if let Some(id) = self.id.try_get_int(query) {
            return Hachimi::instance().localized_data.load()
                .race_jikkyo_comment_dict
                .get(&id)
                .map(|s| s.to_il2cpp_string())
        }

        None
    }

    fn get_origin_tag(&self, query: *mut Il2CppObject, idx: i32) -> Option<String> {
        if !self.message.is_select_idx(idx) {
            return None;
        }

        let id = self.id.try_get_int(query)?;
        Some(format!("RJC:{}", id))
    }
}

// race_jikkyo_message
#[derive(Default)]
pub struct RaceJikkyoMessageQuery {
    // SELECT
    id: Column,
    message: Column
}

impl SelectQueryState for RaceJikkyoMessageQuery {
    fn add_column(&mut self, idx: i32, name: &str) {
        match name {
            "id" => self.id.select_idx = Some(idx),
            "message" => self.message.select_idx = Some(idx),
            _ => ()
        }
    }

    fn add_param(&mut self, _idx: i32, _name: &str) {}

    fn bind_int(&mut self, _idx: i32, _value: i32) {}

    fn get_text(&self, query: *mut Il2CppObject, idx: i32) -> Option<*mut Il2CppString> {
        if !self.message.is_select_idx(idx) {
            return None;
        }

        if let Some(id) = self.id.try_get_int(query) {
            return Hachimi::instance().localized_data.load()
                .race_jikkyo_message_dict
                .get(&id)
                .map(|s| s.to_il2cpp_string())
        }

        None
    }

    fn get_origin_tag(&self, query: *mut Il2CppObject, idx: i32) -> Option<String> {
        if !self.message.is_select_idx(idx) {
            return None;
        }

        let id = self.id.try_get_int(query)?;
        Some(format!("RJM:{}", id))
    }
}


// sqlparser extensions
pub trait SelectExt {
    fn get_first_table_name(&self) -> Option<&String>;
}

impl SelectExt for ast::Select {
    fn get_first_table_name(&self) -> Option<&String> {
        if let Some(table_with_joins) = self.from.get(0) {
            if let ast::TableFactor::Table { name: object_name, .. } = &table_with_joins.relation {
                if let Some(ident) = object_name.0.get(0) {
                    return Some(&ident.value);
                }
            }
        }

        None
    }
}

pub trait SelectItemExt {
    fn get_unnamed_expr_ident(&self) -> Option<&String>;
}

impl SelectItemExt for ast::SelectItem {
    fn get_unnamed_expr_ident(&self) -> Option<&String> {
        if let ast::SelectItem::UnnamedExpr(expr) = self {
            return expr.get_ident_value();
        }

        None
    }
}

pub trait ExprExt {
    fn binary_op_iter<'a>(&'a self) -> BinaryOpIter<'a>;
    fn get_ident_value(&self) -> Option<&String>;
    fn is_placeholder_value(&self) -> bool;
}

impl ExprExt for ast::Expr {
    fn binary_op_iter<'a>(&'a self) -> BinaryOpIter<'a> {
        BinaryOpIter { stack: vec![self] }
    }

    fn get_ident_value(&self) -> Option<&String> {
        if let ast::Expr::Identifier(ident) = self {
            return Some(&ident.value);
        }

        None
    }

    fn is_placeholder_value(&self) -> bool {
        if let ast::Expr::Value(value) = self {
            if let ast::Value::Placeholder(_) = value {
                return true;
            }
        }

        false
    }
}

pub struct BinaryOpIter<'a> {
    stack: Vec<&'a ast::Expr>
}

pub struct BinaryOpRef<'a> {
    pub left: &'a Box<ast::Expr>,
    pub op: &'a ast::BinaryOperator,
    pub right: &'a Box<ast::Expr>
}

impl<'a> Iterator for BinaryOpIter<'a> {
    type Item = BinaryOpRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let Some(expr) = self.stack.pop() else {
                return None;
            };

            let ast::Expr::BinaryOp { left, op, right } = expr else {
                continue;
            };

            self.stack.push(right);
            self.stack.push(left); // left will be pop'd first

            return Some(BinaryOpRef { left, op, right })
        }
    }
}

#[derive(Default)]
pub struct MetaData {
    pub logical_name_to_hash: FnvHashMap<String, String>,
}

impl MetaData {
    pub fn get_hash(logical_name: &str) -> Option<String> {
        {
            let meta_read = META_DATA.read().unwrap();
            if !meta_read.logical_name_to_hash.is_empty() {
                return meta_read.logical_name_to_hash.get(logical_name).cloned();
            }
        }
        
        let mut meta_write = META_DATA.write().unwrap();
        if meta_write.logical_name_to_hash.is_empty() {
            if RETRIEVED_RAW_KEY.lock().unwrap().is_empty() {
                return None;
            }
            let loaded = Self::load_from_db();
            meta_write.logical_name_to_hash = loaded.logical_name_to_hash;
        }
        meta_write.logical_name_to_hash.get(logical_name).cloned()
    }

    fn load_from_db() -> Self {
        let mut logical_name_to_hash = FnvHashMap::default();
        let meta_path = std::path::PathBuf::from(get_data_path()).join("meta");
        let db_path_str = meta_path.to_string_lossy().to_string();
        
        let conn = Connection::new();
        AUTO_UNLOCK_NEXT_DB.store(true, Ordering::Relaxed);
        
        if Connection::Open(conn, db_path_str.to_il2cpp_string(), std::ptr::null_mut(), std::ptr::null_mut(), 0) {
            let sql = "SELECT n, h FROM a";
            let query = Connection::Query(conn, sql.to_il2cpp_string());
            
            if !query.is_null() {
                while Query::Step(query) {
                    let path_ptr = Query::GetText(query, 0);
                    let hash_ptr = Query::GetText(query, 1);
                    
                    if let (Some(path_str), Some(hash_str)) = (
                        unsafe { path_ptr.as_ref() }.map(|s| s.as_utf16str().to_string()),
                        unsafe { hash_ptr.as_ref() }.map(|s| s.as_utf16str().to_string()),
                    ) {
                        let logical_name = if let Some(idx) = path_str.rfind('/') {
                            format!("{}.a", &path_str[idx + 1..])
                        } else {
                            format!("{}.a", path_str)
                        };
                        logical_name_to_hash.insert(logical_name, hash_str);
                    }
                }
                Query::Dispose(query);
            }
            Connection::CloseDB(conn);
        } else {
            error!("Failed to open meta database at: {}", db_path_str);
        }
        
        MetaData { logical_name_to_hash }
    }
}