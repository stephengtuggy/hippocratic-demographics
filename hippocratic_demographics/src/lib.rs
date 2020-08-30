pub mod human {
    use std::collections::{HashSet, HashMap};
    use std::fmt;
    use std::hash::{Hash, Hasher};
    use std::rc::Rc;
    use std::str::FromStr;
    use super::option_date_time::OptionDate;
    use super::entity::*;
    // use super::organization::Organization;

    pub type SSN = TIN;

    #[derive(Debug, PartialEq, Eq)]
    pub struct Human {
        name: HumanName,
        ssn: SSN,                                               // I wish this was more internationalized, but I haven't figured out how to do it yet
        birth_date: OptionDate,
        addresses: HashMap<AddressType, Address>,
        phone_numbers: HashMap<PhoneNumberType, PhoneNumber>,
        email_addresses: HashMap<EmailAddressType, EmailAddress>,
        employers: HashSet<Rc<String>>,                       // employers: HashSet<Organization>,
        // TODO: Methods? Any more fields?
    }

    impl Hash for Human {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.name.hash(state);
            self.ssn.hash(state);
            self.birth_date.hash(state);
            for (addr_type, addr) in &self.addresses {
                addr_type.hash(state);
                addr.hash(state);
            }
            for (phone_type, phone) in &self.phone_numbers {
                phone_type.hash(state);
                phone.hash(state);
            }
            for (email_type, email) in &self.email_addresses {
                email_type.hash(state);
                email.hash(state);
            }
            for e in &self.employers {
                e.hash(state);
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum HumanName {
        FirstMiddleLast { first_name: Rc<String>, middle_name: Rc<String>, last_name: Rc<String> },
        FirstLastNoMiddle { first_name: Rc<String>, last_name: Rc<String> },
        FirstMiddleMaidenLast { first_name: Rc<String>, middle_name: Rc<String>, maiden_name: Rc<String>, last_name: Rc<String> },
        FirstMiddleLastMothersMaiden { first_name: Rc<String>, middle_name: Rc<String>, last_name: Rc<String>, mothers_maiden_name: Rc<String> },
        FamilyNameGivenNames { family_name: Rc<String>, given_names: Vec<Rc<String>> },
        Patronymic1 { given_name: Rc<String>, fathers_name: Rc<String>, grandfathers_name: Rc<String> },
        Patronymic2 { given_name: Rc<String>, middle_name: Rc<String>, fathers_name: Rc<String> },
        FirstMiddleMultipleLastNames { first_name: Rc<String>, middle_name: Rc<String>, last_names: Vec<Rc<String>> },
        Fallback { name_components: Vec<Rc<String>> },
    }

    #[derive(Debug)]
    pub struct HumanNameParseErr;
    pub type HumanNameResult = Result<HumanName, HumanNameParseErr>;

    fn write_name_components(f: &mut fmt::Formatter, name_components: &Vec<Rc<String>>) -> fmt::Result {
        if name_components.len() > 0 {
            write!(f, "{}", name_components[0])?;
            for i in 1..name_components.len() {
                write!(f, " {}", name_components[i])?;
            }
        }
        Ok(())
    }

    impl FromStr for HumanName {
        type Err = HumanNameParseErr;

        // TODO: Implement more sophisticated parsing
        fn from_str(s: &str) -> HumanNameResult {
            let name_components_iter = s.split_whitespace();
            let mut name_components = Vec::<Rc<String>>::new();
            for s in name_components_iter {
                name_components.push(Rc::new(s.to_string()));
            }
            let ret_val = HumanName::Fallback { name_components: name_components };
            Ok(ret_val)
        }
    }

    impl fmt::Display for HumanName {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                HumanName::FirstMiddleLast{first_name, middle_name, last_name}                                      => write!(f, "{} {} {}", first_name, middle_name, last_name),
                HumanName::FirstLastNoMiddle{first_name, last_name}                                                 => write!(f, "{} {}", first_name, last_name),
                HumanName::FirstMiddleMaidenLast{first_name, middle_name, maiden_name, last_name}                   => write!(f, "{} {} {} {}", first_name, middle_name, maiden_name, last_name),
                HumanName::FirstMiddleLastMothersMaiden{first_name, middle_name, last_name, mothers_maiden_name}    => write!(f, "{} {} {} {}", first_name, middle_name, last_name, mothers_maiden_name),
                HumanName::FamilyNameGivenNames{family_name, given_names}                                           => {
                    write!(f, "{} ", family_name)?;
                    write_name_components(f, given_names)?;
                    Ok(())
                },
                HumanName::Patronymic1{given_name, fathers_name, grandfathers_name}                                 => write!(f, "{} {} {}", given_name, fathers_name, grandfathers_name),
                HumanName::Patronymic2{given_name, middle_name, fathers_name}                                       => write!(f, "{} {} {}", given_name, middle_name, fathers_name),
                HumanName::FirstMiddleMultipleLastNames{first_name, middle_name, last_names}                        => {
                    write!(f, "{} {} ", first_name, middle_name)?;
                    write_name_components(f, last_names)?;
                    Ok(())
                },
                HumanName::Fallback{name_components}                                                                => write_name_components(f, name_components),
            }
        }
    }
}

pub mod option_date_time {
    use std::fmt;
    use std::hash::Hash;
    use std::str::FromStr;
    use regex::Regex;

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct OptionDate {
        year: Option<i64>,
        month: Option<u8>,
        day: Option<u8>,
        // TODO: Any more methods?
    }

    #[derive(Debug)]
    pub struct OptionDateParseErr;
    pub type OptionDateResult = Result<OptionDate, OptionDateParseErr>;

    impl FromStr for OptionDate {
        type Err = OptionDateParseErr;
        
        // TODO: Allow for more flexible parsing
        fn from_str(s: &str) -> OptionDateResult {
            // TODO: Make this lazy_static
            let re = Regex::new(r"^(\d{4})-(\d{2})-(\d{2})$").unwrap();
            if re.is_match(s) {
                let caps = re.captures(s).unwrap();
                let year: i64 = caps[1].parse().unwrap();
                let month: u8 = caps[2].parse().unwrap();
                let day: u8 = caps[3].parse().unwrap();
                let ret_val = OptionDate { year: Some(year), month: Some(month), day: Some(day) };
                Ok(ret_val)
            } else {
                Err(OptionDateParseErr)
            }
        }
    }

    impl fmt::Display for OptionDate {
        // TODO: Allow for the possibility that one or more of these optional components are missing
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}-{}-{}", self.year.unwrap(), self.month.unwrap(), self.day.unwrap())
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct OptionTime {
        hour: Option<u8>,
        minute: Option<u8>,
        second: Option<u8>,
        nanosecond: Option<u32>,
        // TODO: Any more methods?
    }

    #[derive(Debug)]
    pub struct OptionTimeParseErr;
    pub type OptionTimeResult = Result<OptionTime, OptionTimeParseErr>;

    impl FromStr for OptionTime {
        type Err = OptionTimeParseErr;

        // TODO: Allow for more flexible parsing
        fn from_str(s: &str) -> OptionTimeResult {
            // TODO: Make this lazy_static
            let re = Regex::new(r"^(\d{2}):(\d{2}):(\d{2}):(\d{9})$").unwrap();
            if re.is_match(s) {
                let caps = re.captures(s).unwrap();
                let hour: u8 = caps[1].parse().unwrap();
                let minute: u8 = caps[2].parse().unwrap();
                let second: u8 = caps[3].parse().unwrap();
                let nanosecond: u32 = caps[4].parse().unwrap();
                let ret_val = OptionTime { hour: Some(hour), minute: Some(minute), second: Some(second), nanosecond: Some(nanosecond) };
                Ok(ret_val)
            } else {
                Err(OptionTimeParseErr)
            }
        }
    }

    impl fmt::Display for OptionTime {
        // TODO: Allow for the possibility that one or more of these optional components are missing
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}:{}:{}.{}", self.hour.unwrap(), self.minute.unwrap(), self.second.unwrap(), self.nanosecond.unwrap())
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct OptionDateTime {
        date_part: OptionDate,
        time_part: OptionTime,
        // TODO: Any more methods?
    }

    #[derive(Debug)]
    pub struct OptionDateTimeParseErr;
    pub type OptionDateTimeResult = Result<OptionDateTime, OptionDateTimeParseErr>;

    impl FromStr for OptionDateTime {
        type Err = OptionDateTimeParseErr;

        // TODO: Allow for more flexible parsing
        fn from_str(s: &str) -> OptionDateTimeResult {
            // TODO: Make this lazy_static
            let re = Regex::new(r"^(\d{4}-\d{2}-\d{2})[ T](\d{2}:\d{2}:\d{2}:\d{9})$").unwrap();
            if re.is_match(s) {
                let caps = re.captures(s).unwrap();
                let date_part: OptionDate = caps[1].parse().unwrap();
                let time_part: OptionTime = caps[2].parse().unwrap();
                let ret_val = OptionDateTime { date_part: date_part, time_part: time_part };
                Ok(ret_val)
            } else {
                Err(OptionDateTimeParseErr)
            }
        }
    }

    impl fmt::Display for OptionDateTime {
        // TODO: Allow for the possibility that one or more of these optional components are missing
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} {}", self.date_part, self.time_part)
        }
    }
}

pub mod entity {
    use std::fmt;
    use std::hash::Hash;
    use std::rc::Rc;
    use std::str::FromStr;
    use regex::Regex;
    use unicode_segmentation::UnicodeSegmentation;

    pub type AddressType = String;
    pub type PhoneNumberType = String;
    pub type EmailAddressType = String;

    // FIXME: Implement a custom type, with custom validation
    pub type PhoneNumber = String;
    // FIXME: Implement a custom type, with custom validation
    pub type EmailAddress = String;
    
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct TIN {
        unencrypted_string: Rc<String>,
    }
    
    #[derive(Debug)]
    pub enum EncryptedBytesErr {
        EncryptionError,
        NotYetImplementedError,
    }
    pub type EncryptedBytesResult = Result<Vec<u8>, EncryptedBytesErr>;
    
    #[derive(Debug)]
    pub struct TINParseErr;
    pub type TINResult = Result<TIN, TINParseErr>;

    impl TIN {
        pub fn as_unencrypted_string(&self) -> Rc<String> {
            Rc::clone(&self.unencrypted_string)
        }
        // FIXME
        pub fn as_encrypted_bytes(&self) -> EncryptedBytesResult {
            Err(EncryptedBytesErr::NotYetImplementedError)
        }
        pub fn last_few_chars(&self) -> Rc<String> {
            let g = UnicodeSegmentation::graphemes(self.unencrypted_string.as_str(), true).collect::<Vec<&str>>();
            let last_few = &g[g.len()-4..];
            Rc::new(last_few.join(""))
        }
    }
    
    impl FromStr for TIN {
        type Err = TINParseErr;
        
        // TODO: Implement more sophisticated parsing
        fn from_str(s: &str) -> TINResult {
            Ok(TIN { unencrypted_string: Rc::new(s.to_string()) })
        }
    }

    impl fmt::Display for TIN {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "XXX-XX-{}", self.last_few_chars())
        }
    }

    #[derive(Debug)]
    pub struct AddressParseErr;
    pub type AddressResult = Result<Address, AddressParseErr>;

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct Address {
        line_1: Rc<String>,
        line_2: Rc<String>,
        line_3: Rc<String>,
        city: Rc<String>,
        state_or_province: Rc<String>,
        zip_or_postal_code: Rc<String>,
        country: Rc<String>,
        // TODO: Any more methods?
    }

    impl FromStr for Address {
        type Err = AddressParseErr;
        
        // TODO: Implement better parsing
        fn from_str(s: &str) -> AddressResult {
            // TODO: Make this lazy_static
            let re = Regex::new(r"^([^,]+), *([^,]+), *([[:alpha:]]{2}) *(\d{5}), *([^,])$").unwrap();
            if re.is_match(s) {
                let caps = re.captures(s).unwrap();
                let line_1 = Rc::new(caps[1].to_string());
                let line_2 = Rc::new("".to_string());
                let line_3 = Rc::new("".to_string());
                let city = Rc::new(caps[2].to_string());
                let state_or_province = Rc::new(caps[3].to_string());
                let zip_or_postal_code = Rc::new(caps[4].to_string());
                let country = Rc::new(caps[5].to_string());
                let ret_val = Address { line_1: line_1, line_2: line_2, line_3: line_3, city: city, state_or_province: state_or_province, zip_or_postal_code: zip_or_postal_code, country: country };
                Ok(ret_val)
            } else {
                Err(AddressParseErr)
            }
        }
    }

    impl fmt::Display for Address {
        // TODO: Include lines 2 and 3 if present; separate with newlines?
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}, {}, {}, {}, {}", self.line_1, self.city, self.state_or_province, self.zip_or_postal_code, self.country)
        }
    }
}

pub mod organization {
    use std::collections::HashMap;
    // use std::fmt;
    use std::hash::{Hash, Hasher};
    use std::rc::Rc;
    // use std::str::FromStr;
    use super::entity::*;

    #[derive(Debug, PartialEq, Eq)]
    pub struct Organization {
        name: Rc<String>,
        tin_number: TIN,
        addresses: HashMap<AddressType, Address>,
        phone_numbers: HashMap<PhoneNumberType, PhoneNumber>,
        email_addresses: HashMap<EmailAddressType, EmailAddress>,
    }

    impl Hash for Organization {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.name.hash(state);
            self.tin_number.hash(state);
            for (addr_type, addr) in &self.addresses {
                addr_type.hash(state);
                addr.hash(state);
            }
            for (phone_type, phone) in &self.phone_numbers {
                phone_type.hash(state);
                phone.hash(state);
            }
            for (email_type, email) in &self.email_addresses {
                email_type.hash(state);
                email.hash(state);
            }
        }
    }
}

pub mod health_insurance {
    // use std::fmt;
    use std::hash::Hash;
    use super::option_date_time::OptionDate;
    use super::organization::Organization;
    use super::human::Human;
    
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct Visit {
        date_of_service: OptionDate,
        // TODO: Combine these into an enum Provider, with three subtypes/elements?
        medical_facility: Option<MedicalFacility>,
        medical_provider: Option<MedicalProvider>,
    }

    pub type MedicalFacility = Organization;
    pub type MedicalProvider = Human;
}

pub mod fuzzy_matching {
    use std::collections::{HashSet, HashMap};
    // use std::fmt;
    use std::hash::{Hash, Hasher};
    use std::rc::Rc;
    // use std::str::FromStr;

    pub type Similarity = f64;
    pub type EditDistance = usize;

    pub trait SimilarityCalculator {
        fn algorithm_name(&self) -> &'static str;
        fn get_similarity(&self, item1: Rc<String>, item2: Rc<String>) -> Similarity;
    }

    pub trait EditDistanceCalculator {
        fn algorithm_name(&self) -> &'static str;
        fn get_edit_distance(&self, item1: &String, item2: &String) -> EditDistance;
        fn max_possible_edit_distance(&self, item1: &String, item2: &String) -> EditDistance;
    }

    #[derive(Debug)]
    pub struct BKTree<RecordType, EditDistanceCalc>
        where RecordType: PartialEq + Eq + Hash,
        EditDistanceCalc: EditDistanceCalculator {
            root_node: BKTreeNode<RecordType>,
            edit_distance_calculator: Rc<EditDistanceCalc>,
            max_distance_to_consider: EditDistance,
    }

    impl<RecordType: PartialEq + Eq + Hash, EditDistanceCalc: EditDistanceCalculator> BKTree<RecordType, EditDistanceCalc> {
        pub fn new(first_value: Rc<String>, first_record: Rc<RecordType>, edit_distance_calculator: Rc<EditDistanceCalc>, max_distance_to_consider: EditDistance) -> Self {
            let root_node = BKTreeNode::<RecordType>::new(first_value, first_record);
            let ret_val = BKTree::<RecordType, EditDistanceCalc> { root_node: root_node, edit_distance_calculator: edit_distance_calculator, max_distance_to_consider: max_distance_to_consider };
            ret_val
        }

        pub fn insert(&mut self, value: Rc<String>, record_found_in: Rc<RecordType>) -> bool {
            self.root_node.recursive_insert(Rc::clone(&value), Rc::clone(&record_found_in), Rc::clone(&self.edit_distance_calculator))
        }

        pub fn search(&self, value: Rc<String>, record_found_in: Rc<RecordType>) -> Vec<(Rc<String>, Rc<RecordType>)> {
            let mut rtn = Vec::new();
            self.recursive_search(&self.root_node, value, record_found_in, &mut rtn);
            rtn
        }

        fn recursive_search(&self, node: &BKTreeNode<RecordType>, value: Rc<String>, record_found_in: Rc<RecordType>, rtn: &mut Vec<(Rc<String>, Rc<RecordType>)>) {
            let cur_edit_distance = self.edit_distance_calculator.get_edit_distance(&node.value, &value);
            let min_distance = cur_edit_distance - self.max_distance_to_consider;
            let max_distance = cur_edit_distance + self.max_distance_to_consider;
            if cur_edit_distance <= self.max_distance_to_consider {
                rtn.push((Rc::clone(&value), Rc::clone(&record_found_in)));
            }
            let children = &node.children;
            for k in children.keys() {
                if (k >= &min_distance) && (k <= &max_distance) {
                    let child_node = &children[k];
                    self.recursive_search(child_node, Rc::clone(&value), Rc::clone(&record_found_in), rtn);
                }
            }
        }
    }

    #[derive(Debug)]
    pub struct BKTreeNode<RecordType>
        where RecordType: PartialEq + Eq + Hash {
            value: Rc<String>,
            records_found_in: HashSet<Rc<RecordType>>,
            children: HashMap<EditDistance, BKTreeNode<RecordType>>,
    }

    impl<RecordType: PartialEq + Eq + Hash> BKTreeNode<RecordType> {
        pub fn new(value: Rc<String>, first_record_found_in: Rc<RecordType>) -> Self {
            let mut records_found_in = HashSet::<Rc<RecordType>>::new();
            records_found_in.insert(first_record_found_in);
            let children = HashMap::new();
            let ret_val = BKTreeNode::<RecordType> { value: value, records_found_in: records_found_in, children: children };
            ret_val
        }

        fn recursive_insert<EditDistanceCalc: EditDistanceCalculator>(&mut self, value: Rc<String>, record_found_in: Rc<RecordType>, edit_distance_calculator: Rc<EditDistanceCalc>) -> bool {
            let dist = edit_distance_calculator.get_edit_distance(&self.value, &value);
            if dist == 0 {
                assert!(&self.value == &value);
                return self.records_found_in.insert(record_found_in);
            } else if self.children.contains_key(&dist) {
                let mut c = &mut *self.children.get_mut(&dist).unwrap();
                return c.recursive_insert::<EditDistanceCalc>(Rc::clone(&value), Rc::clone(&record_found_in), Rc::clone(&edit_distance_calculator));
            } else {
                let new_node = BKTreeNode::new(value, record_found_in);
                match self.children.insert(dist, new_node) {
                    Some(new_node) => return true,
                    None => return false,
                }
            }
        }
    }

    impl<RecordType: PartialEq + Eq + Hash> Hash for BKTreeNode<RecordType> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.value.hash(state);
            for r in &self.records_found_in {
                r.hash(state);
            }
        }
    }

    impl<RecordType: PartialEq + Eq + Hash> PartialEq for BKTreeNode<RecordType> {
        fn eq(&self, other: &Self) -> bool {
            self.value == other.value && self.records_found_in == other.records_found_in
        }
    }

    impl<RecordType: PartialEq + Eq + Hash> Eq for BKTreeNode<RecordType> {}
}

// TODO: Implement actual tests!
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
