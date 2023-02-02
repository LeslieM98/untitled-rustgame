pub type StatValueType = i32;
pub type StatModifierType = f32;
pub type StatIdentifier = String;

pub trait StatModifier {
    fn get_identifier(&self) -> &StatIdentifier;
    fn get_modifier(&self) -> StatModifierType;
}

pub trait StatAddition {
    fn get_identifier(&self) -> &StatIdentifier;
    fn get_addition_value(&self) -> StatValueType;
}

pub trait Stat {
    fn get_identifier(&self) -> &StatIdentifier;

    fn get_base_value(&self) -> StatValueType;

    fn get_combined_absolute_modifiers(&self) -> StatModifierType;
    fn get_combined_addition_modifiers(&self) -> StatModifierType;
    fn get_combined_base_modifiers(&self) -> StatModifierType;

    fn get_combined_additions(&self) -> StatValueType;

    fn calculate_absolute_value(&self) -> StatValueType {
        let first_step =
            self.calculate_modified_base_value() + self.calculate_modified_additional_value();

        let applied_absolute_modifiers = (self.get_base_value() + self.get_combined_additions())
            as StatModifierType
            * self.get_combined_absolute_modifiers();

        first_step + applied_absolute_modifiers.floor() as StatValueType
    }
    fn calculate_modified_base_value(&self) -> StatValueType {
        let result = self.get_base_value() as StatModifierType * self.get_combined_base_modifiers();
        result.floor() as StatValueType
    }
    fn calculate_modified_additional_value(&self) -> StatValueType {
        let result = self.get_combined_additions() as StatModifierType
            * self.get_combined_addition_modifiers();
        result.floor() as StatValueType
    }

    fn set_base_value(&mut self, value: StatValueType);

    fn add_stat_addition(&mut self, addition: dyn StatAddition);
    fn add_stat_modifier(&mut self, addition: dyn StatModifier);

    fn remove_stat_addition(&mut self, identifier: StatIdentifier)
        -> Option<Box<dyn StatAddition>>;
    fn remove_stat_modifier(&mut self, identifier: StatIdentifier)
        -> Option<Box<dyn StatModifier>>;
}

pub mod default_stats {
    use crate::stats::*;

    pub struct GenericStatModifier {
        identifier: StatIdentifier,
        value: StatModifierType,
    }
    impl GenericStatModifier {
        pub fn new(identifier: StatIdentifier, value: StatModifierType) -> GenericStatModifier {
            GenericStatModifier { identifier, value }
        }
    }

    impl StatModifier for GenericStatModifier {
        fn get_identifier(&self) -> &StatIdentifier {
            &self.identifier
        }

        fn get_modifier(&self) -> StatModifierType {
            self.value
        }
    }

    pub struct GenericStatAddition {
        identifier: StatIdentifier,
        value: StatValueType,
    }

    impl GenericStatAddition {
        pub fn new(identifier: StatIdentifier, value: StatValueType) -> GenericStatAddition {
            Self { identifier, value }
        }
    }

    impl StatAddition for GenericStatAddition {
        fn get_identifier(&self) -> &StatIdentifier {
            &self.identifier
        }

        fn get_addition_value(&self) -> StatValueType {
            self.value
        }
    }

    pub struct GenericStat {
        base_value: StatValueType,
        identifier: StatIdentifier,
        absolute_modifiers: Vec<Box<dyn StatModifier>>,
    }

    impl GenericStat {
        pub fn new() -> GenericStat {
            todo!()
        }
    }

    impl Stat for GenericStat {
        fn get_identifier(&self) -> &StatIdentifier {
            &self.identifier
        }

        fn get_base_value(&self) -> StatValueType {
            self.base_value
        }

        fn get_combined_absolute_modifiers(&self) -> StatModifierType {
            todo!()
        }

        fn get_combined_addition_modifiers(&self) -> StatModifierType {
            todo!()
        }

        fn get_combined_base_modifiers(&self) -> StatModifierType {
            todo!()
        }

        fn get_combined_additions(&self) -> StatValueType {
            todo!()
        }

        fn set_base_value(&mut self, value: StatValueType) {
            todo!()
        }

        fn add_stat_addition(&mut self, addition: dyn StatAddition) {
            todo!()
        }

        fn add_stat_modifier(&mut self, addition: dyn StatModifier) {
            todo!()
        }

        fn remove_stat_addition(
            &mut self,
            identifier: StatIdentifier,
        ) -> Option<Box<dyn StatAddition>> {
            todo!()
        }

        fn remove_stat_modifier(
            &mut self,
            identifier: StatIdentifier,
        ) -> Option<Box<dyn StatModifier>> {
            todo!()
        }
    }
}
