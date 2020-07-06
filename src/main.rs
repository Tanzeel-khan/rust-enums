fn main() {
  
    // let student1 =Student::Online;
    // let student2 =Student::Onsite;
    // let ip_address1 = IpAddress{
    //     kind : IpAddrKind::V4,
    //     address: String::from("127.0.0.1")
    // };
    // let ip_address2 = IpAddress{
    //     kind : IpAddrKind::V6,
    //     address : String::from("127.0.0.1")
    // };
    // println!("{:#?}",ip_address1);
    // println!("{:#?}",ip_address2);
    let ip_address1 = IpAddrKind::V4(String::from("127.0.0.2"));
    let ip_address2 = IpAddrKind::V6(String::from("127.0.1.1"));
    println!("{:#?}",ip_address1);
    println!("{:#?}",ip_address2);

}
 #[derive(Debug)]
// enum Student{
//      Online,
//      Onsite
// }

 enum IpAddrKind {
    V4(String),
    V6(String)
 }
//  #[derive(Debug)]
//  struct IpAddress{
//      kind : IpAddrKind,
//     address : String
//  }
