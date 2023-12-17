pub struct NameSpace(Vec<String>);

impl NameSpace {
    pub fn new<S: Into<String>>(namespace: S) -> Self {
        let namespace = namespace
            .into()
            .split("::")
            .map(|x| x.to_string())
            .collect();
        Self(namespace)
    }

    pub fn get_super(&self) -> NameSpace {
        let last_level = self.0[0..self.0.len() - 1].to_vec();
        NameSpace(last_level)
    }

    pub fn append(&self, name: &str) -> NameSpace {
        let name = name.split("::").collect::<Vec<_>>();
        let mut v = NameSpace(self.0.clone());
        for n in name {
            if n == "super" {
                v = self.get_super();
            } else {
                v.push(n);
            }
        }
        v
    }

    fn push(&mut self, name: &str) {
        self.0.push(name.into());
    }
}

impl ToString for NameSpace {
    fn to_string(&self) -> String {
        self.0.join("::")
    }
}
