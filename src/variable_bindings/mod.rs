pub fn test_variables() {
    //block scope
    let _extend = 3u32;

    let mut _freezing = 99u32;
    {
        let inner = "Inner";
        println!("inner:{}", inner);

        //freezing
        let _freezing = 3u32;
        // freezing=5;
    }
    _freezing = 9;
    // let inner = 3;
    // println!("inner:{}", inner);
}
