use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct ArraySet {
    array: Vec<String>,
    set: HashMap<String, usize>,
}

impl ArraySet {
    /// What is the element at the given index?
    /// # Examples
    /// ```
    /// use rusty_source_map::array_set;
    /// let set = array_set::ArraySet::new();
    ///
    /// assert_eq!(set.size(), 0);
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// What is the element at the given index?
    /// # Examples
    /// ```
    /// use rusty_source_map::array_set;
    /// let set = array_set::ArraySet::from_array(vec!["a".to_owned()], false);
    ///
    /// assert_eq!(set.size(), 1);
    /// ```
    pub fn from_array(arr: Vec<String>, allow_duplicates: bool) -> Self {
        let mut array_set = ArraySet::new();
        for i in arr {
            array_set.add(i, allow_duplicates);
        }
        array_set
    }

    /// What is the element at the given index?
    /// # Examples
    /// ```
    /// use rusty_source_map::array_set;
    /// let set = array_set::ArraySet::from_array(vec!["a".to_owned()], false);
    ///
    /// assert_eq!(set.size(), 1);
    /// ```
    pub fn size(&self) -> usize {
        self.set.len()
    }

    /// What is the element at the given index?
    /// # Examples
    /// ```
    /// use rusty_source_map::array_set;
    /// let mut set = array_set::ArraySet::from_array(vec!["a".to_owned()], false);
    /// set.add("string".to_owned(), false);
    ///
    /// assert_eq!(set.size(), 2);
    /// ```
    pub fn add(&mut self, data: String, allow_duplicates: bool) {
        let is_duplicate = self.has(data.clone());
        let idx = self.array.len();

        if !is_duplicate || allow_duplicates {
            self.array.push(data.clone());
        }
        if !is_duplicate {
            self.set.insert(data, idx);
        }
    }

    /// What is the element at the given index?
    /// # Examples
    /// ```
    /// use rusty_source_map::array_set;
    /// let set = array_set::ArraySet::from_array(vec!["a".to_owned()], false);
    /// assert_eq!(set.has("a".to_owned()), true);
    /// ```
    pub fn has(&self, data: String) -> bool {
        self.set.contains_key(data.as_str())
    }

    /// What is the element at the given index?
    /// # Examples
    /// ```
    /// use rusty_source_map::array_set;
    /// let set = array_set::ArraySet::from_array(vec!["a".to_owned()], false);
    /// assert_eq!(set.index_of("a".to_owned()), Some(0));
    /// ```
    pub fn index_of(&self, data: String) -> Option<usize> {
        self.set.get(data.as_str()).cloned()
    }

    /// What is the element at the given index?
    /// # Examples
    /// ```
    /// use rusty_source_map::array_set;
    /// let set = array_set::ArraySet::from_array(vec!["a".to_owned()], false);
    /// assert_eq!(set.at(0), Some("a".to_owned()));
    /// ```
    pub fn at(&self, idx: i32) -> Option<String> {
        if idx >= 0 && idx < self.array.len() as i32 {
            self.array.get(idx as usize).cloned()
        } else {
            None
        }
    }

    ///
    /// Returns the array representation of this set (which has the proper indices
    /// indicated by indexOf). Note that this is a copy of the internal array used
    /// for storing the members so that no one can mess with internal state.
    ///
    /// # Examples
    /// ```
    /// use rusty_source_map::array_set;
    /// let set = array_set::ArraySet::from_array(vec!["a".to_owned()], false);
    /// assert_eq!(set.to_vec().len(), 1);
    /// ```
    pub fn to_vec(&self) -> Vec<String> {
        self.array.clone()
    }
}
