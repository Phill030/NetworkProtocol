pub struct Hwid(String, String);

impl Hwid {
    pub fn new(cpu_id: String, system_id: String) -> Self {
        Self(cpu_id, system_id)
    }
}
