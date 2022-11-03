
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum OptionalBehavior {
    UnitSymbol,
    UnitValue,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum StructBehavior {
    ExcludeTyping,
    IncludeTyping,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum VariantNameBehavior {
    Short,
    Full,
    Index,
}

pub struct GarnishSerializationOptions {
    pub optional_behavior: OptionalBehavior,
    pub struct_typing_behavior: StructBehavior,
    pub variant_name_behavior: VariantNameBehavior,
}

impl GarnishSerializationOptions {
    pub fn new() -> Self {
        Self {
            optional_behavior: OptionalBehavior::UnitValue,
            struct_typing_behavior: StructBehavior::ExcludeTyping,
            variant_name_behavior: VariantNameBehavior::Full,
        }
    }

    pub fn optional_behavior(mut self, optional_behavior: OptionalBehavior) -> Self {
        self.optional_behavior = optional_behavior;
        self
    }

    pub fn struct_typing_behavior(mut self, struct_typing_behavior: StructBehavior) -> Self {
        self.struct_typing_behavior = struct_typing_behavior;
        self
    }

    pub fn variant_name_behavior(mut self, variant_name_behavior: VariantNameBehavior) -> Self {
        self.variant_name_behavior = variant_name_behavior;
        self
    }
}