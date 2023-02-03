use bevy::utils::HashMap;

pub type StatValueType = i32;
pub type StatModifierType = f32;
pub type StatIdentifier = String;

#[derive(Clone, Debug)]
pub struct StatModifier {
    pub identifier: StatIdentifier,
    pub value: StatModifierType,
}

impl StatModifier {
    pub fn new(identifier: StatIdentifier, value: StatModifierType) -> Self {
        Self { identifier, value }
    }
}

#[derive(Clone, Debug)]
pub struct StatAddition {
    pub identifier: StatIdentifier,
    pub value: StatValueType,
}

impl StatAddition{
    pub fn new(identifier: StatIdentifier, value: StatValueType) -> Self {
        Self{identifier, value}
    }
}

#[derive(Debug)]
pub struct Stat {
    identifier: StatIdentifier,
    base_value: StatValueType,
    pub additions: HashMap<StatIdentifier, StatAddition>,
    pub absolute_modifiers: HashMap<StatIdentifier, StatModifier>,
    pub base_modifiers: HashMap<StatIdentifier, StatModifier>,
    pub additional_modifiers: HashMap<StatIdentifier, StatModifier>,
}

impl Stat {
    pub fn new(identifier: StatIdentifier, base_value: StatValueType) -> Self {
        Self {
            identifier,
            base_value,
            additions: HashMap::new(),
            absolute_modifiers: HashMap::new(),
            base_modifiers: HashMap::new(),
            additional_modifiers: HashMap::new(),
        }
    }

    pub fn get_identifier(&self) -> &StatIdentifier {
        &self.identifier
    }

    pub fn get_base_value(&self) -> StatValueType {
        self.base_value
    }

    pub fn get_combined_absolute_modifiers(&self) -> StatModifierType {
        self.absolute_modifiers
            .iter()
            .map(|(_,v)| v.value)
            .fold(0., |x, y| x+y)
    }
    pub fn get_combined_additional_modifiers(&self) -> StatModifierType {
        self.additional_modifiers
            .iter()
            .map(|(_,v)| v.value)
            .sum()
    }
    pub fn get_combined_base_modifiers(&self) -> StatModifierType {
        self.base_modifiers
            .iter()
            .map(|(_,v)| v.value)
            .sum()
    }

    pub fn insert_addition(&mut self, addition: StatAddition){
        self.additions.insert(addition.identifier.clone(), addition);
    }
    pub fn insert_absolute_modifier(&mut self, modifier: StatModifier){
        self.absolute_modifiers.insert(modifier.identifier.clone(), modifier);
    }
    pub fn insert_base_modifier(&mut self, modifier: StatModifier){
        self.base_modifiers.insert(modifier.identifier.clone(), modifier);
    }
    pub fn insert_addition_modifier(&mut self, modifier: StatModifier){
        self.additional_modifiers.insert(modifier.identifier.clone(), modifier);
    }

    pub fn get_combined_additions(&self) -> StatValueType {
        self.additions
            .iter()
            .map(|(_,v)| v.value)
            .sum()
    }

    pub fn calculate_absolute_value(&self) -> StatValueType {
        let absolute_modifier = self.get_combined_absolute_modifiers();
        let additional_calculated = (absolute_modifier + self.get_combined_additional_modifiers() + 1.0) * self.get_combined_additions() as StatModifierType;
        let base_calculated = (absolute_modifier + self.get_combined_base_modifiers() + 1.0) * self.get_base_value() as StatModifierType;
        (additional_calculated + base_calculated).floor() as StatValueType
    }
    pub fn calculate_modified_base_value(&self) -> StatValueType {
        let result = self.get_base_value() as StatModifierType * self.get_combined_base_modifiers();
        result.floor() as StatValueType
    }
    pub fn calculate_modified_additional_value(&self) -> StatValueType {
        let result = self.get_combined_additions() as StatModifierType
            * self.get_combined_additional_modifiers();
        result.floor() as StatValueType
    }
}


pub struct StatBlock {
    stats: HashMap<StatIdentifier, Stat>
}

#[cfg(test)]
mod tests {
    use super::*;

    fn big_stat() -> Stat{
        let mut intelligence = Stat::new(String::from("intelligence"), 200);

        let head_gear = StatAddition::new(String::from("head_gear"), 10);
        let chest_gear = StatAddition::new(String::from("chest_gear"), 40);
        intelligence.insert_addition(head_gear.clone());
        intelligence.insert_addition(chest_gear.clone());

        let int_buff1 = StatModifier::new(String::from("int_buff1"), 0.05);
        let int_buff2 = StatModifier::new(String::from("int_buff2"), 0.05);
        intelligence.insert_absolute_modifier(int_buff1);
        intelligence.insert_absolute_modifier(int_buff2);

        let armor_buff1 = StatModifier::new(String::from("armor_buff1"), 0.1);
        let armor_buff2 = StatModifier::new(String::from("armor_buff2"), 0.1);
        intelligence.insert_addition_modifier(armor_buff1);
        intelligence.insert_addition_modifier(armor_buff2);

        let racial_buff1 = StatModifier::new(String::from("racial_buff1"), 0.02);
        let racial_buff2 = StatModifier::new(String::from("racial_buff2"), 0.02);
        intelligence.insert_addition_modifier(racial_buff1);
        intelligence.insert_addition_modifier(racial_buff2);

        return intelligence
    }

    fn empty_stat() -> Stat {
        Stat::new(String::from("intelligence"), 200)
    }

    #[test]
    fn correct_calc() {
        let mut intelligence = empty_stat();

        assert_eq!(intelligence.get_base_value(), 200);
        assert_eq!(intelligence.calculate_absolute_value(), intelligence.get_base_value());

        intelligence.insert_base_modifier(StatModifier::new(String::from("racial_buff"), 0.01));
        assert_eq!(intelligence.calculate_modified_base_value(), 2);
        assert_eq!(intelligence.calculate_absolute_value(), 202);

        intelligence = empty_stat();
        intelligence.insert_addition(StatAddition::new(String::from("head_gear"), 10));
        assert_eq!(intelligence.calculate_absolute_value(), 210);
        intelligence.insert_addition_modifier(StatModifier::new(String::from("armor_buff"), 0.1));
        assert_eq!(intelligence.calculate_absolute_value(), 211);
        intelligence.insert_addition(StatAddition::new(String::from("chest_gear"), 40));
        assert_eq!(intelligence.get_combined_additions(), 50);
        assert_eq!(intelligence.calculate_absolute_value(), 255);
        assert_eq!(intelligence.calculate_modified_additional_value(), 5);

        intelligence.insert_absolute_modifier(StatModifier::new(String::from("int_buff"), 0.05));
        assert_eq!(intelligence.calculate_modified_base_value(), 12);
        assert_eq!(intelligence.calculate_absolute_value(), 269);

    }
}