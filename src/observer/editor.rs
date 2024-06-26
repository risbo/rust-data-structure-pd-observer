use create::observer::{Event, Pusblisher};

#[derive(Default)]
pub struct Editor{
    publisher: Publisher,
    file_path: String
}

impl Editor {
    pub fn events(&mut self) -> &mut Pusblisher {
        &muth self.publisher
    }

    pub fn load(&mut self, path: String) {
        self.file_path = path.clone();
        self.publisher.notify(Event::Load, path);
    }

    pub fn save(&self) {
        self.publisher.notify(Event::Save, self.file_path.clone());
    }
}