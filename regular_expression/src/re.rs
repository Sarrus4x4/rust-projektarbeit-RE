#[derive(Debug)]

// ######### Structs #########
pub struct Eps {
    //can it stay empty?
//pub val: String
}
pub struct Phi {
    //can it stay empty?
//pub val: String
}
pub struct C {
    pub val: String,
}
pub struct Alt<T, J> {
    pub l: T,
    pub r: J,
}
pub struct Conc<T, J> {
    pub l: T,
    pub r: J,
}
pub struct Star<T> {
    pub obj: T,
}

// ######### Traits #########
pub trait Pretty {
    //for printing out the result
    fn pretty(&self) -> String;
}
pub trait ContainsEps {
    //check if the element contains Eps
    fn contains_eps(&self) -> bool;
}
pub trait IsPhi {
    //check if the whole RE is Phi (not shure if i even need this)
    fn is_phi(&self) -> bool;
}

// ######### Implementation Pretty #########
impl Pretty for C {
    fn pretty(&self) -> String {
        self.val.to_string()
    }
}
impl<T: Pretty + ContainsEps + IsPhi, J: Pretty + ContainsEps + IsPhi> Pretty for Alt<T, J> { //TODO: Add logic r + r ==> r (Do I need a isEqual Function?)
    fn pretty(&self) -> String {
        if self.l.contains_eps() || self.l.is_phi() {
            if self.r.contains_eps() || self.r.is_phi() {
                format!("")
            } else {
                format!("{}", self.r.pretty())
            }
        } else if self.r.contains_eps() || self.r.is_phi() {
            format!("{}", self.l.pretty())
        } else {
            format!("({}|{})", self.l.pretty(), self.r.pretty()) //logic to check for phi and format accordingly
        }
    }
}
impl<T: Pretty + ContainsEps + IsPhi, J: Pretty + ContainsEps + IsPhi> Pretty for Conc<T, J> {
    fn pretty(&self) -> String {
        if self.l.is_phi() || self.r.is_phi() {
            //if left or right argument of concat is Phi -> the whole term is Phi
            format!("")
        } else {
            if self.l.contains_eps() {
                if self.r.contains_eps() {
                    format!("")
                } else {
                    format!("{}", self.r.pretty())
                }
            } else if self.r.contains_eps() {
                format!("{}", self.l.pretty())
            } else {
                format!("({}{})", self.l.pretty(), self.r.pretty()) //logic to check for phi and format accordingly
            }
        }
    }
}

impl<T: Pretty + ContainsEps + IsPhi> Pretty for Star<T> {
    fn pretty(&self) -> String {
        if self.obj.contains_eps() || self.obj.is_phi() {
            //logic to check for eps/phi and format accordingly
            format!("")
        } else {
            format!("({}*)", self.obj.pretty()) //TODO: Add logic for (r*)* ==> r* And also the logic for r* ==> eps falls L(r)={}
        }
    }
}
impl Pretty for Eps {
    fn pretty(&self) -> String {
        format!("")
    }
}
impl Pretty for Phi {
    fn pretty(&self) -> String {
        format!("phi")
    }
}

// ######### Implementation ContainsEps #########
impl ContainsEps for C {
    fn contains_eps(&self) -> bool {
        let mut contains: bool = false;
        if self.pretty() == "" {
            contains = true;
        }
        contains
    }
}
impl ContainsEps for Phi {
    fn contains_eps(&self) -> bool {
        let mut contains: bool = false;
        if self.pretty() == "" {
            contains = true;
        }
        contains
    }
}
impl ContainsEps for Eps {
    fn contains_eps(&self) -> bool {
        let mut contains: bool = false;
        if self.pretty() == "" {
            contains = true;
        }
        contains
    }
}
impl<T: Pretty + ContainsEps + IsPhi, J: Pretty + ContainsEps + IsPhi> ContainsEps for Alt<T, J> {
    fn contains_eps(&self) -> bool {
        let mut contains: bool = false;
        if self.pretty() == "" {
            contains = true;
        }
        contains
    }
}
impl<T: Pretty + ContainsEps + IsPhi, J: Pretty + ContainsEps + IsPhi> ContainsEps for Conc<T, J> {
    fn contains_eps(&self) -> bool {
        let mut contains: bool = false;
        if self.pretty() == "" {
            contains = true;
        }
        contains
    }
}
impl<T: Pretty + ContainsEps + IsPhi> ContainsEps for Star<T> {
    fn contains_eps(&self) -> bool {
        let mut contains: bool = false;
        if self.pretty() == "" {
            contains = true;
        }
        contains
    }
}

// ######### Implementation IsPhi #########
impl IsPhi for C {
    fn is_phi(&self) -> bool {
        let mut contains: bool = false;
        if self.pretty() == "phi" {
            contains = true;
        }
        contains
    }
}
impl<T: Pretty + ContainsEps + IsPhi, J: Pretty + ContainsEps + IsPhi> IsPhi for Alt<T, J> {
    fn is_phi(&self) -> bool {
        let mut contains: bool = false;
        if self.pretty() == "phi" {
            contains = true;
        }
        contains
    }
}
impl<T: Pretty + ContainsEps + IsPhi, J: Pretty + ContainsEps + IsPhi> IsPhi for Conc<T, J> {
    fn is_phi(&self) -> bool {
        let mut contains: bool = false;
        if self.pretty() == "phi" {
            contains = true;
        }
        contains
    }
}
impl<T: Pretty + ContainsEps + IsPhi> IsPhi for Star<T> {
    fn is_phi(&self) -> bool {
        let mut contains: bool = false;
        if self.pretty() == "phi" {
            contains = true;
        }
        contains
    }
}
impl IsPhi for Eps {
    fn is_phi(&self) -> bool {
        let mut contains: bool = false;
        if self.pretty() == "phi" {
            contains = true;
        }
        contains
    }
}
impl IsPhi for Phi {
    fn is_phi(&self) -> bool {
        let mut contains: bool = false;
        if self.pretty() == "phi" {
            contains = true;
        }
        contains
    }
}

// ######### main method #########
pub fn run() {
    let re1 = Alt {
        l: C {
            val: "a".to_string(),
        },
        r: C {
            val: "b".to_string(),
        },
    };
    println!("{}", re1.pretty());

    let re2 = Conc {
        l: C {
            val: "a".to_string(),
        },
        r: C {
            val: "b".to_string(),
        },
    };
    println!("{}", re2.pretty());

    let re3 = Star {
        obj: C {
            val: "a".to_string(),
        },
    };
    println!("{}", re3.pretty());

    let re4 = C {
        val: "a".to_string(),
    };
    println!("{}", re4.contains_eps());

    let re5 = C {
        val: "".to_string(),
    };
    println!("{}", re5.contains_eps());

    let re6 = C {
        val: "a".to_string(),
    };
    println!("{}", re6.is_phi());

    let re7 = C {
        val: "phi".to_string(),
    };
    println!("{}", re7.is_phi());

    let re8 = Conc {
        l: Star {
            obj: Star {
                obj: C {
                    val: "a".to_string(),
                },
            },
        },
        r: Alt {
            l: Phi {},
            r: C {
                val: "b".to_string(),
            },
        },
    };
    println!("{}", re8.pretty());

    //This needs to be displayed correctly ->  (a*) b
    let re9 = Conc {
        l: Eps {},
        r: Conc {
            l: Star {
                obj: Star {
                    obj: C {
                        val: "a".to_string(),
                    },
                },
            },
            r: Alt {
                l: Phi {},
                r: C {
                    val: "b".to_string(),
                },
            },
        },
    };
    println!("{}", re9.pretty());
}
