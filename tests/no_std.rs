// #[cfg(feature = "no_std")]
mod tests_no_std {
    use murmur3::*;

    #[test]
    fn test_cursor() {
        let mut cursor = Cursor::new(vec!{ 1, 2, 3, 4, 5 });
        let mut buf = [0; 2];

        assert_eq!(cursor.read(&mut buf).unwrap(), 2);
        println!("{:?}", buf);
        assert!(buf == [1, 2]);

        assert_eq!(cursor.read(&mut buf).unwrap(), 2);
        println!("{:?}", buf);
        assert!(buf == [3, 4]);

        assert_eq!(cursor.read(&mut buf).unwrap(), 1);
        println!("{:?}", buf);
        assert!(buf == [5, 4]);
    }
}
