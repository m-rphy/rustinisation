fn main() {
    
    // tuples
    let rgb_color = (255, 108, 16);
    let cmky_color = (23, 108, 36, 244);

    // tuple structs
    //  There is more structure for out data
    struct RGB(i32, i32, i32);
    struct CMYK(i32, i32, i32, i32);

    let color_1 = RGB(255, 108, 16);
    let color_2 = CMYK(23, 108, 36, 244);
    
    //Unit struct (rarely used)
    struct MyStruct; 
}
