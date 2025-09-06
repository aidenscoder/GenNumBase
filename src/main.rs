pub mod number;

macro_rules! Record {
    ($name:ident, $arg_type:ty, ($($arg:ident),*), {$($body:item),*}) => {
        pub struct $name {
            $(pub $arg: $arg_type,)*
        }

        impl $name {
            $($body)*
        }
    };
}



Record!( 
    Point,u32,(x,y),{
        pub fn sum(self) -> u32 {
            self.x + self.y
        },
        pub fn dif(self) -> u32 {
            self.x - self.y
        }
    }
);

fn main(){
    
}