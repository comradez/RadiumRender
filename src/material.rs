pub trait Material {
    fn has_emission(&self) -> bool;
    fn texture_mapping(&self) -> bool;
    fn two_sided(&self) -> bool;
}