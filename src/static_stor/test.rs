create_static_stor!(pub akw: Vec<u8>);

#[test]
fn test() {
    akw::with(|e| assert_eq!(e, &vec![]) );
    akw::with_mut(|e| e.push(1) );
    akw::with(|e| assert_eq!(e, &vec![1]) );
    akw::with_mut(|e| e.push(2) );
    akw::with(|e| assert_eq!(e, &vec![1,2]) );
    akw::with_mut(|e| e.push(4) );
    akw::with(|e| assert_eq!(e, &vec![1,2,4]) );
    akw::with_mut(|e| e.push(8) );
    akw::with(|e| assert_eq!(e, &vec![1,2,4,8]) );
}