use std::ops;
use image::{Rgba,Pixel};
use serde::{Serialize, Deserialize};
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Color{
    pub red: f64, //Quizás no sea necesario que sea un f32, evaluar después
    pub green: f64,
    pub blue: f64,
}
impl Color{
    #[inline]
    pub fn new(red: f64,green:f64,blue:f64)->Self{
        Color{
            red,
            green,
            blue,
        }
    }
    #[inline]
    pub fn from_rgb(r:u8,g:u8,b:u8)->Self{
        Color::new(r as f64/255.0, g as f64/255.0, b as f64/255.0)
    }
    #[inline]
    pub fn to_rgb(mut self)->(u8,u8,u8){
        self.clamp();
        ((self.red * 255.0) as u8,(self.green * 255.0) as u8,(self.blue * 255.0) as u8)
    }
    #[inline]
    pub fn to_r(&self)->u8{
        (self.red.min(1.0).max(0.0) * 255.0) as u8
    }
    #[inline]
    pub fn to_g(&self)->u8{
        (self.green.min(1.0).max(0.0) * 255.0) as u8
    }
    #[inline]
    pub fn to_b(&self)->u8{
        (self.blue.min(1.0).max(0.0) * 255.0) as u8
    }
    #[inline]
    pub fn clamp(&mut self){
        self.blue = self.blue.min(1.0).max(0.0);
        self.red = self.red.min(1.0).max(0.0);
        self.green = self.green.min(1.0).max(0.0);
    }
    #[inline]
    pub fn new_white()->Self{
        Color{
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        }
    }
    #[inline]
    pub fn black()->Self{
        Color{
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }
    #[inline]
    pub fn to_rgba(self,a:u8)->Rgba<u8>{
        let canales = self.to_rgb();
        Rgba::from_channels(canales.0,canales.1,canales.2,a)
    }
    #[inline]
    pub fn from_rgba(rgba: Rgba<u8>)->Self{
        Color::new(rgba[0] as f64 / 255.0, rgba[1] as f64 / 255.0, rgba[2] as f64 / 255.0)
    }
}
impl ops::Mul for Color{
    type Output = Color;
    fn mul(self,other: Color)->Color{
        Color::new(self.red * other.red, self.green * other.green, self.blue * other.blue)
    }
}
impl ops::Mul<f64> for Color{
    type Output = Color;
    fn mul(self,other: f64)->Color{
        Color::new(self.red * other, self.green * other, self.blue * other)
    }
}
impl ops::Add for Color{
    type Output = Color;
    fn add(mut self,other: Color)->Color{
        self.red += other.red;
        self.blue += other.blue;
        self.green += other.green;
        self
    }
}