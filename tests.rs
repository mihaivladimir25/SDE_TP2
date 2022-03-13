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