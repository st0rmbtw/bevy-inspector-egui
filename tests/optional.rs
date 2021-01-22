use bevy_inspector_egui::Inspectable;

#[derive(Default)]
struct CustomAttributes {
    optional: Option<u8>,
    defaulted: f32,
}

struct CustomType;
impl Inspectable for CustomType {
    type Attributes = CustomAttributes;
    fn ui(&mut self, _: &mut bevy_inspector_egui::egui::Ui, _: Self::Attributes) {}
}

#[derive(Inspectable)]
struct Data {
    #[inspectable(optional = Some(10), defaulted = 1.0)]
    explicit: CustomType,
    #[inspectable(optional = 10)]
    implicit: CustomType,
    none: CustomType,
}
