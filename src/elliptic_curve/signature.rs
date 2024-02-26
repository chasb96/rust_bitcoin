use crate::field_element::FieldElement;

#[derive(Clone, Debug)]
pub struct Signature {
    r: FieldElement,
    s: FieldElement,
}

