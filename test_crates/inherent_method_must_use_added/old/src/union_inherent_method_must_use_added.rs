pub union UnionWithMustUseMethods {
    bar: usize,
}

impl UnionWithMustUseMethods {

    // These methods did not have the #[must_use] attribute in the old version.
    // Addition of the attribute should be reported.

    pub fn MethodToMustUseMethod(&self) {}

    pub fn MethodToMustUseMessageMethod(&self) {}


    // These methods had the #[must_use] attribute in the old version. Changes
    // of the attribute, including deletion, should not be reported.

    #[must_use]
    pub fn MustUseMethodToMethod(&self) {}

    #[must_use]
    pub fn MustUseMethodToMustUseMessageMethod(&self) {}


    // These methods had the #[must_use] attribute in the old version.
    // They also included the user-defined warning message. Changes of
    // the attribute, including deletion, should not be reported.

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMustUseMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMustUseMessageMethod(&self) {}
}


// This public union's inherent method did not have the #[must_use] attribute
// in the old version. Because the method is private, adding the attribute
// should NOT be reported.

pub union UnionWithPrivateMustUseMethods {
    bar: usize,
}

impl UnionWithPrivateMustUseMethods {

    fn PrivateMethodToPrivateMustUseMethod(&self) {}
}


// This union is private and adding #[must_use] to its inherent method
// should NOT be reported.

union PrivateUnionWithMustUseMethods {
    bar: usize,
}

impl PrivateUnionWithMustUseMethods {

    fn PrivateUnionMethodToPrivateUnionMustUseMethod(&self) {}
}
