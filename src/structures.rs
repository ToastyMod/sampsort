pub enum Pathmeta {
    file(Filelink),
    folder(Folderlink),
    null
}

pub struct Folderlink {
    pub(crate) path: String,
    pub(crate) is_loaded: bool,
    pub(crate) contains: Vec<Pathmeta>
}

pub struct Filelink {
    pub(crate) path: String
}