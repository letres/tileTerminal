extern crate sdl2;

use std::path::Path;

use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::render::TextureCreator;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use std::time::Duration;



struct Cell {
    //Stores all data needed for each cell.
    char: u8,
    pos: Rect,
    red: u8,
    green: u8,
    blue: u8,
    b_red: u8,
    b_green: u8,
    b_blue: u8,
}
struct Cursor {
    pos:(u32, u32),
    store:(u32, u32),
}
struct Region<'cells> {
    //Stores a vector of all cells in a region.
    //Stores image of tileset basis with tile size as well.
//    cursor: Cursor,
    hole: Rect,
    texture:sdl2::render::Texture<'cells>,
    grid: Vec<Cell>,
}
struct Context {
    //Everything that is needed for the window?
    canvas: sdl2::render::Canvas<Window>,
    context: sdl2::Sdl,
}
impl Cell{
    fn new(size:(u32,u32),corner:(i32,i32))->Cell{
        //Creates a new cell with with the corner being the top left.
        let pos = Rect::new(corner.0,corner.1,size.0,size.1);
        let ret=Cell{
            //Default char=0,white color, black backround.
            char:1,
            pos:pos,
            red:255,
            green:255,
            blue:255,
            b_red:0,
            b_green:0,
            b_blue:0,
        };
        ret
    }
    fn render(&self,canvas:&mut sdl2::render::Canvas<Window>,
               texture:&mut sdl2::render::Texture,mut hole:Rect){
        //Renders backround first.
        let x = 11;
        let y = 13;
        texture.set_color_mod(self.b_red,self.b_green,self.b_blue);
        let x1 = (texture.query().width as i32)*x/16;
        let y1 = (texture.query().height as i32)*y/16;
        hole.set_x(x1 as i32);
        hole.set_y(y1 as i32);
        canvas.copy(texture,Some(hole),Some(self.pos)).unwrap();
        //Finds the position on the tile sheet according to the
        //char value
        let x = (self.char%16) as i32;
        let y = (self.char/16) as i32;
        texture.set_color_mod(self.red,self.green,self.blue);
        let x1 = (texture.query().width as i32)*x/16;
        let y1 = (texture.query().height as i32)*y/16;
        hole.set_x(x1 as i32);
        hole.set_y(y1 as i32);
        canvas.copy(texture,Some( hole),Some(self.pos)).unwrap();
    }
}
//impl Cursor{
//    fn new()->Cursor{
//        Cursor{
//            pos:(0,0),
//            store(0,0),
//        }
//    }
//    fn move(&self,m:u32,n,u32){
//        self.pos=(m,n)
//    }
//    fn write(&self,char ){
        
//    }
//}
impl<'cell> Region<'cell> {
    fn new(size:(u32,u32),mut corner:(i32,i32),path:&str
                ,creator:&'cell TextureCreator<WindowContext>)->Region<'cell>{
        let mut temp_surface = sdl2::surface::Surface::load_bmp(
                              Path::new(path)).unwrap();
        //Loads bmp at path and transparancies accordin to keying
        //Starts at the top left and makes a vector 
        temp_surface.set_color_key(true,Color::RGB(255,0,255)).unwrap();
        let texture = creator.create_texture_from_surface(&temp_surface).unwrap();
        let mut grid:Vec<Cell>=vec![];
        let x_start = corner.0;
        let hole_size = (16,16);
        for i in 0..(size.0*size.1){
            let new = Cell::new(hole_size,corner);
            grid.push(new);
            corner.0=corner.0+(hole_size.0 as i32);
            if ((i as u32) + 1)%size.0==0{
                corner.1=corner.1+(hole_size.1 as i32);
                corner.0=x_start;
            }
        }
        let hole=Rect::new(0,0,hole_size.0,hole_size.1);
        Region{
            texture:texture,
            grid:grid,
            hole:hole,
        }
    }
    fn render(&mut self,mut canvas:&mut sdl2::render::Canvas<Window>){
        //Renders all cells in the Region
        let x = self.grid.len();
        for i in 0..x{
            self.grid[i].render(&mut canvas,&mut self.texture,self.hole)
        }
    }
    fn render_by_cell(&mut self,mut canvas:&mut sdl2::render::Canvas<Window>, 
                      fit: fn(u32)-> bool ){
        //Renders all cells that fit a criteria
        let x = self.grid.len();
        for i in 0..x{
            if fit(i as u32){
                self.grid[i].render(&mut canvas,&mut self.texture,self.hole)
            }
        }
    }
}
impl Context{
    fn new(size:(u32, u32))->Context{
        let context = sdl2::init().unwrap();
        let video = context.video().unwrap();
        let window = video.window("Test", size.0,size.1)
                          .position_centered().build().unwrap();
        let mut canvas = window.into_canvas()
                               .accelerated().build().unwrap();
        canvas.set_draw_color(sdl2::pixels::Color::RGBA(0,0,0,255));
        Context{
            context:context,
            canvas:canvas,
        }
    }
}
fn main(){
//    let _image_context=sdl2::image::init(InitFlag::PNG).unwrap();
    let mut context = Context::new((640,480));
    let creator = context.canvas.texture_creator();
    let mut region  = Region::new((40,30),(0,0),"assets/tileSet.bmp",&creator);
    context.canvas.clear();
    region.render(&mut context.canvas);
    context.canvas.present();
    std::thread::sleep(Duration::from_millis(8000));
}
