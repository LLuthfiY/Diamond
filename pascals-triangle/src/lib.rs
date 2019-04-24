pub struct PascalsTriangle{
    row : u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle{ row:row_count}
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut bigVec = Vec::new();
        
        if self.row>0{
            bigVec.push(vec![1]);
            
            if self.row>1{
                bigVec.push(vec![1,1]);
                if self.row>2{
                    let mut temp = vec![1,1];
                    for _x in 1..self.row-1{
                        let mut sv = Vec::new();
                        sv.push(1);
                        for y in 0..temp.len()-1{
                            sv.push(temp[y]+temp[y+1]);
                        }
                        sv.push(1);
                        temp = sv.clone();
                        bigVec.push(sv);
                        
                        
                    }
                }
            }
        }
        bigVec
    }
}
