







date :2/april/2026
12:30 pm
------------------------------------------------------------------------------------------
todays  greatest success is this little code 

fn main(){

    let val1 : i32  = 120 ;
    let val2 : i32 = 32 ;
    let touple1 : (&i32 , &i32) = (&val1 , &val2) ;
    let touple2 : &(&i32 , &i32) = &touple1 ;
    let touple3 : &&(&i32 , &i32) = &touple2 ;
    println!("The touple 3 is {:?}" , touple3) ;
}
___________________________________________________________________________________________
