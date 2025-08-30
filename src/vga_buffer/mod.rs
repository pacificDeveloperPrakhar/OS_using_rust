#![allow(dead_code)]
use core::fmt::Write;
use core::fmt;
use  volatile::Volatile;
use spin::Mutex;
// partial eq expects symmetry,transivity and reflexivity
// if u apply eq then u should also apply partial eq
// reflexivity x==y then y==x
// transivity x==y y==z x==z
//symmetry all the value of x and y are equal
// now defining the colors
#[derive(Clone,Copy)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

// now usuaully this was supposed to be 4 bit but the least bits we can store is 8 bit so we have to work around this

// now creating a struct store the color only
#[derive(Debug,Clone,Copy,Eq,PartialEq)]
#[repr(transparent)]
#[allow(dead_code)]
pub struct ColorCode(u8);

// first 4 bit correspond to the foregorund and the following 4 bits correspond to background 
// 0000 0000  backgorund foreground
impl ColorCode {
    pub fn new(background:Color,foreground:Color)->ColorCode {
        let shifted_background_color:u8=(background as u8)<<4;
        let result=shifted_background_color|(foreground as u8) ;
        return ColorCode(result);
    }
}

// now a struct to represent the single character
#[derive(Debug,Copy,Eq,PartialEq,Clone)]
#[repr(C)]
pub struct ScreenChar {
    pub ascii_value:u8,
    pub color:ColorCode,
}

pub const BUFFER_HEIGHT:usize=25;
pub const BUFFER_WIDTH:usize=80;

#[repr(transparent)]
pub struct Buffer {
    pub chars:[[Volatile<ScreenChar>;BUFFER_WIDTH];BUFFER_HEIGHT]
}

// this is the writer to write the character to the buffer
pub struct Writer {
    pub column_position:usize,
    pub color_code_background:Color,
    pub color_code_foreground:Color,
    pub buffer:&'static mut Buffer,
}
// now implement the write for the writer
impl fmt::Write for Writer
{
    fn write_str(& mut self,s:&str)->fmt::Result
    {

     self.write_string(s);
     Ok(())
    }
}
impl Writer {
    pub fn write_byte(&mut self,byte:u8) {
        match byte {
            b'\n'=>{
              self.new_line();
            },
            _=>{
                let column_position=self.column_position;
                let row=BUFFER_HEIGHT-1;
                if column_position>=BUFFER_WIDTH {
                    self.new_line();
                };
                self.buffer.chars[row][column_position].write(ScreenChar{
                    ascii_value:byte,
                    color:ColorCode::new(self.color_code_background,self.color_code_foreground),
                });

                self.column_position+=1;
            }
        }
    }

    // now the function that will be called for the new line
    pub fn new_line(&mut self) {
       for row in 1..BUFFER_HEIGHT
       {
        for col in 0..BUFFER_WIDTH
        {
            let character=self.buffer.chars[row][col].read();
             self.buffer.chars[row-1][col].write(character);
        }
        self.clear_line(row);
        self.column_position=0;
       }
    }

    //implemtation for the writing of the whole string
    pub fn write_string(&mut self,sen:&str) {
        for byte in sen.bytes() {
            match byte {
                0x20..=0x7e|b'\n' =>{
                    self.write_byte(byte);
                },
                _=>{
                    self.write_byte(63);
                }
            }
           
        }
    }

    pub fn clear_line(& mut self,row:usize)
    {
     for x in 0..self.column_position
     {
         self.buffer.chars[row][x].write(ScreenChar
        {
            ascii_value:b' ',
            color:ColorCode::new(Color::Black,Color::White)

        });
     }
    }
}
//create the global static writer which when accesed for the first time will inititalize
//then after creating that it will use that everywhere
use lazy_static::lazy_static;
lazy_static!{
    pub static ref writer:Mutex<Writer>=Mutex::new(Writer
    {
        column_position:0,
        color_code_foreground:Color::White,
        color_code_background:Color::Black,
        buffer:unsafe{
            &mut *(0xb8000 as *mut Buffer)
        }
    });
}
//now this mutex from the spin does not require the os functionalities because right now we are applying the
// most basic lockingin mechaninism
// where the thread or consumer just access the data using the loop so ,keeps trying to lock in the loop
// once its successfull it locks on
// now the function to write something
pub fn write_something()
{
   write!(writer.lock(),"now here is the new sentence").unwrap();
}

// now we will override the print and println function from the std library but since we have no 
// std library we hav eto create of our own from the scratch