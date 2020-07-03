fn main() {
  
    let student1 =Student::Online;
    let student2 =Student::Onsite;
    println!("{:?}",student1);
}
#[derive(Debug)]
enum Student{
    Online,
    Onsite
}