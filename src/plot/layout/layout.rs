
#[derive(Default)]
pub enum LayoutPriority {
    #[default] Maximize,
    Minimize,
}

pub trait Layout {
    fn calculate_size(&self, parent: Option<&Box<dyn Layout>>) -> (u32, u32);
    fn calculate_abs_size(&self, parent: Option<&Box<dyn Layout>>) -> (u32, u32);
}

pub struct EmptyLayout;

impl Layout for EmptyLayout {
    fn calculate_size(&self, _parent: Option<&Box<dyn Layout>>) -> (u32, u32) {
        (0, 0)
    }
    
    fn calculate_abs_size(&self, parent: Option<&Box<dyn Layout>>) -> (u32, u32) {
        match parent {
            Some(parent) => {
                parent.calculate_abs_size(Some(parent))
            },
            None => {
                (0, 0)
            }
        }
    }
}
