use rand::prelude::*;
use raylib::prelude::*;

#[derive(Clone, Debug)]
struct QuadTree {
    max_points: usize,
    w: i32,
    h: i32,
    x: i32,
    y: i32,
    points: Option<Vec<Point>>,
    lt: Option<Box<QuadTree>>,
    lb: Option<Box<QuadTree>>,
    rt: Option<Box<QuadTree>>,
    rb: Option<Box<QuadTree>>,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl QuadTree {
    fn new(max_points: usize, w: i32, h: i32, x: i32, y: i32) -> Self {
        Self {
            max_points,
            points: Some(vec![]),
            w,
            h,
            x,
            y,
            lt: None,
            lb: None,
            rt: None,
            rb: None,
        }
    }

    fn insert_point(&mut self, point: Point) -> () {
        if self.points.is_none() {
            if point.x >= self.x + self.w / 2 {
                // right side of quad
                if point.y >= self.y + self.h / 2 {
                    //bottom right
                    if let Some(rb) = self.rb.as_mut() {
                        rb.insert_point(point);
                    } else {
                        let mut rb = QuadTree::new(
                            self.max_points,
                            self.w / 2,
                            self.h / 2,
                            self.x + self.w / 2,
                            self.y + self.h / 2,
                        );
                        rb.insert_point(point);
                        self.rb = Some(Box::new(rb));
                    }
                } else {
                    //top right
                    if let Some(rt) = self.rt.as_mut() {
                        rt.insert_point(point);
                    } else {
                        let mut rt = QuadTree::new(
                            self.max_points,
                            self.w / 2,
                            self.h / 2,
                            self.x + self.w / 2,
                            self.y,
                        );
                        rt.insert_point(point);
                        self.rt = Some(Box::new(rt));
                    }
                }
            } else {
                if point.y >= self.y + self.h / 2 {
                    //bottom left

                    if let Some(lb) = self.lb.as_mut() {
                        lb.insert_point(point);
                    } else {
                        let mut lb = QuadTree::new(
                            self.max_points,
                            self.w / 2,
                            self.h / 2,
                            self.x,
                            self.y + self.h / 2,
                        );
                        lb.insert_point(point);
                        self.lb = Some(Box::new(lb));
                    }
                } else {
                    //top left
                    if let Some(lt) = self.lt.as_mut() {
                        lt.insert_point(point);
                    } else {
                        let mut lt =
                            QuadTree::new(self.max_points, self.w / 2, self.h / 2, self.x, self.y);
                        lt.insert_point(point);
                        self.lt = Some(Box::new(lt));
                    }
                }
            }
        } else {
            if let Some(points) = self.points.as_mut() {
                if points.len() >= self.max_points {
                    let cloned = points.clone();
                    self.points = None;
                    for &cloned_point in cloned.iter() {
                        self.insert_point(cloned_point);
                    }
                } else {
                    if (self.x > point.x || self.x + self.w < point.x)
                        || (self.y > point.y || self.y + self.h < point.y)
                    {
                        println!("no place for such a point over here");
                        return;
                    }
                    points.push(point);
                }
            }
        }
    }

    // pass coordinates to find the quad
    fn find_quad(&self, x: i32, y: i32) -> Option<&QuadTree> {
        println!("{}, {}, {x}, {y}", self.x, self.y);
        if (self.x > x || self.x + self.w < x) || (self.y > y || self.y + self.h / 2 < y) {
            return None; //since we dont need to traverse children cus point is out of bbox
        }
        if self.points.is_none() {
            let quads = vec![&self.lt, &self.lb, &self.rt, &self.rb];
            for &quad in quads.iter() {
                if let Some(quad) = quad {
                    return quad.find_quad(x, y);
                }
            }
        } else {
            return Some(self);
        }
        None
    }

    fn draw(&self, d: &mut RaylibDrawHandle) -> () {
        d.draw_rectangle_lines(self.x, self.y, self.w, self.h, Color::BLACK);

        if let Some(points) = &self.points {
            for &point in points.iter() {
                d.draw_circle(point.x, point.y, 2.0, Color::BLACK);
            }
        } else {
            let selfs = vec![&self.lt, &self.lb, &self.rt, &self.rb];
            for &qt_child in selfs.iter() {
                if let Some(qt_child) = qt_child {
                    qt_child.draw(d);
                }
            }
        }
    }
}

static HEIGHT: i32 = 640;
static WIDTH: i32 = 640;

pub fn main() {
    let mut qt = QuadTree::new(4, WIDTH, HEIGHT, 0, 0);
    for _ in 0..1_000 {
        qt.insert_point(Point {
            x: (random::<f64>() * (WIDTH as f64)) as i32,
            y: (random::<f64>() * (HEIGHT as f64)) as i32,
        });
    }

    let (mut rl, thread) = raylib::init().size(WIDTH, HEIGHT).title("Quadtree").build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        qt.draw(&mut d);
    }
}
