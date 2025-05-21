use super::ds::{
    DS0, 
    DS1,
    DS2,
    DS3,
    DS4
};

trait BPBuildDS {
    fn con(&self) -> Vec<u8>;
}

impl BPBuildDS for DS0 {
    fn con(&self) -> Vec<u8> {
        vec![
            self.v0
        ]
    }
}

impl BPBuildDS for DS1 {
    fn con(&self) -> Vec<u8> {
        vec![
            self.v0, 
            self.v1, 
            self.v2, 
            self.v3, 
            self.v4, 
            self.v5, 
            self.v6
        ]
    }
}

impl BPBuildDS for DS2 {
    fn con(&self) -> Vec<u8> {
        vec![
            self.v0, 
            self.v1, 
            self.v2
        ]
    }
}

impl BPBuildDS for DS3 {
    fn con(&self) -> Vec<u8> {
        vec![
            self.v0, 
            self.v1, 
            self.v2, 
            self.v3, 
            self.v4, 
            self.v5
        ]
    }
}

impl BPBuildDS for DS4 {
    fn con(&self) -> Vec<u8> {
        vec![
            self.v0, 
            self.v1, 
            self.v2, 
            self.v3, 
            self.v4, 
            self.v5, 
            self.v6, 
            self.v7
        ]
    }
}

fn con<V>(bp: &V) -> Vec<u8>
where V: BPBuildDS {
    bp.con()
}

pub(crate) struct DP {
    pub(crate) v0: Vec<u8>,
    pub(crate) v1: Vec<u8>,
    pub(crate) v2: Vec<u8>,
    pub(crate) v3: Vec<u8>,
    pub(crate) v4: Vec<u8>
}

impl DP {
    pub(crate) fn new() -> Self {
        let bp0 = DS0::new();
        let bp1 = DS1::new();
        let bp2 = DS2::new();
        let bp3 = DS3::new();
        let bp4 = DS4::new();

        Self {
            v0: con(&bp0),
            v1: con(&bp1),
            v2: con(&bp2),
            v3: con(&bp3),
            v4: con(&bp4)
        }
    }

    pub(crate) fn feed<Food>(&self, food: Food)
    where Food: Fn(&Vec<u8>),
    {
        food(&self.v4);
        food(&self.v3);
        food(&self.v2);
        food(&self.v1);
        food(&self.v0);
        food(&self.v0);
    }
}
