pub mod human {
    use std::collections::{HashSet, HashMap};
    use std::hash::Hash;
    use std::str::FromStr;
    use unicode_segmentation::UnicodeSegmentation;
    use super::option_date_time::OptionDate;
    use super::entity::{Entity, Address, GovernmentID};
    use super::organization::Organization;

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct Human {
        name: HumanName,
        // governmentID: GovernmentID,
        birth_date: OptionDate,
        // addresses: HashMap<&'static str, Address>,
        // phone_numbers: HashMap<&'static str, &'static str>,
        // email_addresses: HashMap<&'static str, &'static str>,
        employers: HashSet<Organization>,
        // TODO: Methods? Any more fields?
    }

    impl Entity for Human {}
    
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum HumanName {
        FirstMiddleLast { first_name: &'static str, middle_name: &'static str, last_name: &'static str },
        FirstLastNoMiddle { first_name: &'static str, last_name: &'static str },
        FirstMiddleMaidenLast { first_name: &'static str, middle_name: &'static str, maiden_name: &'static str, last_name: &'static str },
        FirstMiddleLastMothersMaiden { first_name: &'static str, middle_name: &'static str, last_name: &'static str, mothers_maiden_name: &'static str },
        FamilyNameGivenNames { family_name: &'static str, given_names: Vec<&'static str> },
        Patronymic1 { given_name: &'static str, fathers_name: &'static str, grandfathers_name: &'static str },
        Patronymic2 { given_name: &'static str, middle_name: &'static str, fathers_name: &'static str },
        FirstMiddleMultipleLastNames { first_name: &'static str, middle_name: &'static str, last_names: Vec<&'static str> },
        Fallback { name_components: Vec<&'static str> },
    }

    pub struct HumanNameParseErr;

    impl FromStr for HumanName {
        type Err = HumanNameParseErr;

        // TODO: Implement more sophisticated parsing
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let name_components = vec!(s.split_whitespace());
            let ret_val = Fallback { name_components: name_components };
            Ok(ret_val)
        }
    }

    pub type SSNParseErr;
    
    // TODO: Custom implementation of these traits?
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SocialSecurityNumber {
        unencrypted_string: &'static str,
    }

    impl FromStr for SocialSecurityNumber {
        type Err = SSNParseErr;
        
        // TODO: Implement more sophisticated parsing
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(SocialSecurityNumber { unencrypted_string: s })
        }
    }

    impl GovernmentID for SocialSecurityNumber {
        // pub fn new(from_string: &'static str) -> SocialSecurityNumber {
        //     SocialSecurityNumber { unencrypted_string: from_string }
        // }

        pub fn as_unencrypted_string(&self) -> &str {
            self.unencrypted_string
        }

        pub fn as_encrypted_bytes(&self) -> Result<&Vec<u8>, str> {
            // TODO: Implement this
            Err("Not yet implemented")
        }

        pub fn last_few_chars(&self) -> Result<&Vec<char>, str> {
            let g = self.unencrypted_string.graphemes(true).collect::<Vec<&str>>();
            let last_few = &g[g.len-4..];
            Ok(last_few)
        }
    }
}

pub mod option_date_time {
    use std::hash::Hash;

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct OptionDate {
        year: Option<i64>,
        month: Option<u8>,
        day: Option<u8>,
        // TODO: Figure out methods
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct OptionTime {
        hour: Option<u8>,
        minute: Option<u8>,
        second: Option<u8>,
        nanosecond: Option<u32>,
        // TODO: Figure out methods
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct OptionDateTime {
        date_part: OptionDate,
        time_part: OptionTime,
        // TODO: Figure out methods
    }
}

pub mod entity {
    use std::hash::Hash;
    use std::collections::{HashSet, HashMap};

    pub type AddressType = String;
    pub type PhoneNumberType = String;
    pub type EmailAddressType = String;

    // FIXME: Implement a custom type, with custom validation
    pub type PhoneNumber = String;
    // FIXME: Implement a custom type, with custom validation
    pub type EmailAddress = String;
    
    pub trait Entity: Debug + PartialEq + Eq + Hash {
        governmentID: GovernmentID,
        addresses: HashSet<(AddressType, Address)>,
        phone_numbers: HashSet<(PhoneNumberType, PhoneNumber)>,
        email_addresses: HashSet<(EmailAddressType, EmailAddress)>,
    }
    
    pub trait GovernmentID : Debug + PartialEq + Eq + Hash + FromStr {
        // pub fn new(from_string: &'static str) -> Self;
        pub fn as_unencrypted_string(&self) -> &str;
        pub fn as_encrypted_bytes(&self) -> Result<&Vec<u8>, str>;
        pub fn last_few_chars(&self) -> Result<&Vec<char>, str>;
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct Address {
        line_1: &'static str,
        line_2: &'static str,
        line_3: &'static str,
        city: &'static str,
        state_or_province: &'static str,
        zip_or_postal_code: &'static str,
        country: &'static str,
        // TODO: Methods?
    }
}

pub mod organization {
    use std::collections::{HashSet, HashMap};
    use std::hash::Hash;
    use std::str::FromStr;
    use unicode_segmentation::UnicodeSegmentation;
    use super::entity::{Entity, Address, GovernmentID};

    // TODO: Custom implementation of these?
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct TIN {
        unencrypted_string: &'static str,
    }

    pub type TINParseErr;

    impl GovernmentID for TIN {
        // pub fn new(from_string: &'static str) -> TIN {
        //     TIN { unencrypted_string: from_string }
        // }

        pub fn as_unencrypted_string(&self) -> &str {
            self.unencrypted_string
        }

        pub fn as_encrypted_bytes(&self) -> Result<&Vec<u8>, str> {
            // TODO: Implement this
            Err("Not yet implemented")
        }

        pub fn last_few_chars(&self) -> Result<&Vec<char>, str> {
            let g = self.unencrypted_string.graphemes(true).collect::<Vec<&str>>();
            let last_few = &g[g.len-4..];
            Ok(last_few)
        }
    }

    impl FromStr for TIN {
        type Err = TINParseErr;

        // TODO: Implement more sophisticated parsing and validation
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(TIN { unencrypted_string: s })
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub trait Organization : Entity {
        name: &'static str,
        // governmentID: GovernmentID,
        // addresses: HashMap<&'static str, Address>,
        // phone_numbers: HashMap<&'static str, &'static str>,
        // email_addresses: HashMap<&'static str, &'static str>,
    }
}

pub mod health_insurance {
    use std::hash::Hash;
    use super::option_date_time::OptionDate;
    use super::entity::Organization;
    use super::human::Human;
    
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct Visit {
        date_of_service: OptionDate,
        // TODO: Combine these into an enum Provider, with three subtypes/elements?
        medical_facility: Option<MedicalFacility>,
        medical_provider: Option<Human>,
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MedicalFacility {
        street_address: Address,
        mailing_address: Address,
    }

    // TODO: Fill in?
    impl Organization for MedicalFacility {}

    // TODO: Finish filling in this module
}

pub mod fuzzy_matching {
    use std::hash::{Hash, Hasher};

    pub type Similarity = f64;
    pub type EditDistance = usize;

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub trait SimilarityCalculator<T> {
        pub algorithm_name: &'static str;
        pub fn get_similarity<T>(&T item1, &T item2) -> Similarity;
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub trait EditDistanceCalculator<T> : SimilarityCalculator<T> {
        pub fn get_edit_distance<T>(&T item1, &T item2) -> EditDistance;
        pub fn max_possible_edit_distance<T>(&T item1, &T item2) -> EditDistance;
        pub fn get_similarity<T>(&T item1, &T item2) -> Similarity {
            get_edit_distance(item1, item2) / max_possible_edit_distance(item1, item2)
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct BKTree<ValueType, RecordType, EditDistanceCalc>
        where ValueType: Debug, PartialEq, Eq, Hash
        where RecordType: Debug, PartialEq, Eq, Hash
        where EditDistanceCalc: EditDistanceCalculator<ValueType> {
            root_node: BKTreeNode<ValueType, RecordType>;
            edit_distance_calculator: &EditDistanceCalc;
            max_distance_to_consider: EditDistance;
    }

    impl BKTree<ValueType, RecordType, EditDistanceCalc> {
        pub fn new(first_value: ValueType, first_record: RecordType, edit_distance_calculator: EditDistanceCalculator, max_distance_to_consider: EditDistance) -> Self {
            // let records_found_in = HashSet::new();
            // records_found_in.insert(first_record);
            let root_node = BKTreeNode::new(first_value, first_record /*, edit_distance_calculator, max_distance_to_consider*/);
            let ret_val = BKTree { root_node: root_node, edit_distance_calculator: edit_distance_calculator, max_distance_to_consider: max_distance_to_consider };
            ret_val
        }

        pub fn insert(&mut self, value: &ValueType, record_found_in: &RecordType) -> bool {
            let mut cur_node = self.root_node;
            let mut dist = self.edit_distance_calculator.get_edit_distance(cur_node.value, value);
            while cur_node.children.contains_key(dist) {
                if dist == 0 {
                    assert!(cur_node.value == value);
                    return cur_node.records_found_in.insert(record_found_in);
                }
                cur_node = cur_node.children[dist];
                dist = self.edit_distance_calculator.get_edit_distance(cur_node.value, value);
            }

            let new_node = BKTreeNode::new(value, record_found_in /*, &self.edit_distance_calculator, &self.max_distance_to_consider*/ );
            let new_children_hash_set = BKTreeNodeSet<ValueType, RecordType /*, EditDistanceCalc*/ >::new();
            assert!(new_children_hash_set.insert(new_node));
            assert!(cur_node.children.insert(dist, new_children_hash_set));
            return true;
        }

        pub fn search(&self, value: &ValueType, record_found_in: &RecordType) -> Vec<(ValueType, RecordType)> {
            let rtn = Vec::new();
            self.recursive_search(value, record_found_in, rtn);
            rtn
        }

        fn recursive_search(&self, node: &BKTreeNode, &value: ValueType, &record_found_in: RecordType, &mut rtn: Vec<(ValueType, RecordType)>) {
            let cur_edit_distance = self.edit_distance_calculator.get_edit_distance(node.value, value);
            let min_distance = cur_edit_distance - self.max_distance_to_consider;
            let max_distance = cur_edit_distance + self.max_distance_to_consider;
            if cur_edit_distance <= self.max_distance_to_consider {
                rtn.push((value, record_found_in));
            }
            for k in &node.children.keys {
                if k >= min_distance && k <= max_distance {
                    self.recursive_search(node.children[k], value, record_found_in, rtn);
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct BKTreeNodeSet<ValueType, RecordType /*, EditDistanceCalc*/ >
        where ValueType: Debug, PartialEq, Eq, Hash
        where RecordType: Debug, PartialEq, Eq, Hash
        /* where EditDistanceCalc: EditDistanceCalculator<ValueType> */ {
            set: HashSet<BKTreeNode<ValueType, RecordType /*, EditDistanceCalc*/ >>;
    }

    impl BKTreeNodeSet<ValueType, RecordType /*, EditDistanceCalc*/ > {
        pub fn new() -> Self {
            let ret_val = BKTreeNodeSet { set = HashSet<BKTreeNode<ValueType, RecordType /*, EditDistanceCalc*/ >>::new() };
            ret_val
        }

        // pub fn insert(&mut self, BKTreeNode<ValueType, RecordType, EditDistanceCalc> new_child) -> bool {
        //     return self.set.insert(new_child);
        // }
    }

    #[derive(Debug)]
    pub struct BKTreeNode<ValueType, RecordType /*, EditDistanceCalc*/ >
        where ValueType: Debug, PartialEq, Eq, Hash
        where RecordType: Debug, PartialEq, Eq, Hash
        /*where EditDistanceCalc: EditDistanceCalculator<ValueType> */ {
            value: &ValueType;
            records_found_in: HashSet<RecordType>;
            //edit_distance_calculator: &EditDistanceCalc;
            //max_distance_to_consider: EditDistance;
            // edit_distance_from_parent: EditDistance;
            children: HashMap<EditDistance, BKTreeNodeSet<ValueType, RecordType /*, EditDistanceCalc*/ >>;
    }

    impl BKTreeNode<ValueType, RecordType /*, EditDistanceCalc*/ > {
        pub fn new(value: &ValueType, first_record_found_in: &RecordType /*, edit_distance_calculator: &EditDistanceCalculator, max_distance_to_consider: EditDistance*/) -> Self {
            let mut records_found_in = HashSet::new();
            records_found_in.insert(first_record_found_in);
            let children = HashMap::new();
            return BKTreeNode { value: value, records_found_in: records_found_in /*, edit_distance_calculator: edit_distance_calculator, max_distance_to_consider: max_distance_to_consider*/ , children: children };
        }

        // pub fn insert(&mut self, value: &ValueType, record_found_in: &RecordType) -> bool {
        //     let cur_edit_distance = self.edit_distance_calculator.get_edit_distance(&self.value, &value);
        //     if cur_edit_distance == 0 {
        //         assert!(self.value == value);
        //         return self.records_found_in.insert(record_found_in);
        //     } else {
        //         let new_node = BKTreeNode::new(value, record_found_in, &self.edit_distance_calculator, &self.max_distance_to_consider);
        //         let min_distance = cur_edit_distance - &self.max_distance_to_consider;
        //         let max_distance = cur_edit_distance + &self.max_distance_to_consider;
        //         if let Some(children_at_this_edit_distance: BKTreeNodeSet<ValueType, RecordType, EditDistanceCalc>) = self.children.get_mut(&cur_edit_distance) {
        //             return children_at_this_edit_distance.insert(new_node);
        //         } else {
        //             let new_children_hash_set = BKTreeNodeSet<ValueType, RecordType, EditDistanceCalc>::new();
        //             assert!(new_children_hash_set.insert(new_node));
        //             return self.children.insert(cur_edit_distance, new_children_hash_set);
        //         }
        //     }
        // }
    }

    impl Hash for BKTreeNode<ValueType, RecordType> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.value.hash(state);
            self.records_found_in.hash(state);
        }
    }

    impl PartialEq for BKTreeNode<ValueType, RecordType> {
        fn eq(&self, other: &Self) -> bool {
            self.value == other.value && self.records_found_in == other.records_found_in
        }
    }

    impl Eq for BKTreeNode<ValueType, RecordType> {}
}

// TODO: Implement actual tests!
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
