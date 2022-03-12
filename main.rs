struct Fonc{
    vector : Vec<i32>,
    vecprime : Vec<i32>, 
}

impl Fonc{

    fn new()->Fonc{
        Fonc{
            vector : vec![],
            vecprime : vec![],
        }
    }

    fn add(&mut self, k:i32){
        let mut pos = 0;
        let mut poz = 0;
        let mut nr= 0;
        for i in 0..self.vector.len(){
            if self.vector[i]<k{
               pos=pos+1;
            }
            else if self.vector[i]==k{
                return;
            }
        }
        self.vector.insert(pos, k);

        for j in 2..(k/2)+1{
            if k%j == 0{
                nr = nr + 1;
            }

        }

        if nr == 0{

            if self.vector[poz]<k{
                poz=poz+1;
             }

            self.vecprime.insert(poz, k);
        }

        self.vecprime.sort();

    }

    fn remove(&mut self, k:i32){
        let mut index=0;
        for i in 0..self.vector.len(){
            if self.vector[i] < k{
                index = index+1;
            }

            else if self.vector[i] == k{
                break;
            }

            else{
                return;
            }
        }

        if index>0{
            self.vector.remove(index);
        }
    }

    fn get_tranche(&mut self, a:i32, b:i32){
        let mut max = -1;
        let mut min = 10000;

        for i in 0..self.vecprime.len(){
            if self.vecprime[i] < min{
                min = self.vecprime[i];
            }
            
            if self.vecprime[i] > max{
                max = self.vecprime[i];
            }
        }

        for i in 0..self.vecprime.len(){
            if self.vecprime[i] > min && self.vecprime[i] < max{
                println!("{:?}", self.vecprime[i]);
            }

        }
    }

    fn display_vector(&self){
        
        println!("{:?}", self.vector);

    }

    fn display_vecprime(&self){

    println!("{:?}", self.vecprime);
    
    }
    

}


fn main(){

    let mut vector = Fonc::new();
    let mut vecprime = Fonc::new();
    vector.add(1);
    vector.add(2);
    vector.add(3);
    vector.add(5);
    vector.add(4);
    vector.add(7);
    vector.display_vector();
    vector.remove(3);
    vector.display_vector();
    vecprime.get_tranche(1,6);
    vecprime.display_vecprime();
    

}

#[cfg(test)]
mod tests{
    use crate::Fonc;


    #[test]
    fn test_add(){
        let mut test = Fonc::new();
        test.add(1);
        assert_eq! (test.vector, vec![1]);
        test.add(4);
        assert_eq! (test.vector, vec![1,4]);
        test.add(2);
        assert_eq! (test.vector, vec![1,2,4]);
        test.add(6);
        assert_eq! (test.vector, vec![1,2,4,6]);
        test.add(3);
        assert_eq! (test.vector, vec![1,2,3,4,6]);
        test.add(5);
        assert_eq! (test.vector, vec![1,2,3,4,5,6]);
        test.add(7);
        assert_eq! (test.vecprime, vec![1, 2, 3, 5, 7]);
    }

    #[test]
    fn test_remove(){
        let mut test = Fonc::new();
        test.add(1);
        assert_eq! (test.vector, vec![1]);
        test.add(4);
        assert_eq! (test.vector, vec![1,4]);
        test.add(2);
        assert_eq! (test.vector, vec![1,2,4]);
        test.add(6);
        assert_eq! (test.vector, vec![1,2,4,6]);
        test.add(3);
        assert_eq! (test.vector, vec![1,2,3,4,6]);
        test.add(5);
        assert_eq! (test.vector, vec![1,2,3,4,5,6]);

        test.remove(6);
        assert_eq!(test.vector, vec![1,2,3,4,5]);
        test.remove(2);
        assert_eq!(test.vector, vec![1,3,4,5]);

    }

    #[test]
    fn test_tranche(){
        let mut test = Fonc::new();
        test.add(1);
        assert_eq! (test.vector, vec![1]);
        test.add(4);
        assert_eq! (test.vector, vec![1,4]);
        test.add(2);
        assert_eq! (test.vector, vec![1,2,4]);
        test.add(6);
        assert_eq! (test.vector, vec![1,2,4,6]);
        test.add(3);
        assert_eq! (test.vector, vec![1,2,3,4,6]);
        test.add(5);
        assert_eq! (test.vector, vec![1,2,3,4,5,6]);

        test.get_tranche(1,6);
        assert_eq! (test.vector, vec![1,2,3,4,5,6]);
        
    }

}